use std::marker::Send;
use crate::game::{GameTick, Command};
use crate::game::source::Source;
use crossterm::event::{Event as CEvent, KeyCode, KeyEvent};
use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

type Result<T> = std::result::Result<T, InputManagerError>;

/// Abstraction that tracks time between the last input event and Crossterm Events
#[derive(PartialEq, Debug)]
enum Event<I> {
    Input(Duration, I),
    Tick(Duration),
}

type InputEvent = Event<CEvent>;

/// Manages user input by polling on another thread
/// Cleans up when this struct is dropped
pub struct InputManager {
    /// Receiver for communicating input on polling thread
    rx: mpsc::Receiver<InputEvent>,
    /// How long until async thread sends a Event::Tick (max deltatime)
    tick_rate: Duration,
    /// Longest time to wait for any 1 tick from the receiver
    tick_timeout: Duration
}

impl InputManager {

    /// Creates `InputManager` and starts asynchronously polling user input
    pub fn new(source: impl Source + Send + 'static, tick_rate: Duration, tick_timeout: Duration) -> Self {
        let (sx, rx) = mpsc::channel();
        let input_manager = InputManager {
            rx,
            tick_rate,
            tick_timeout
        };

        input_manager.start_async_polling(source, sx);

        input_manager
    }

    /// Starts the async thread that polls for input and returns ticks
    /// Thread exits if any kind of Error is encountered when working with input
    fn start_async_polling(&self, mut source: impl Source + Send + 'static, sx: mpsc::Sender<InputEvent>) {
        let tick_rate = self.tick_rate;
        thread::spawn(move || -> Result<()> {
            let mut last_tick = Instant::now();

            loop {
                // poll for tick rate duration. If no events, send tick event
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                // if has event, send it
                if source.has_event(timeout) {
                    let time_since_last_tick = Instant::now() - last_tick;
                    let event = source.read()?;
                    sx.send(Event::Input(time_since_last_tick, event))?;
                }

                // Send a Tick since `tick_rate` has passed
                if last_tick.elapsed() >= tick_rate {
                    sx.send(Event::Tick(tick_rate))?;
                    last_tick = Instant::now();
                }
            }
        });
    }

    /// Blocks until getting an input
    pub fn tick(&self) -> Result<GameTick> {
        let rx_result = self.rx.recv_timeout(self.tick_timeout)?;

        let game_tick = match rx_result {
            Event::Input(deltatime, event) => Self::match_crossterm_event(deltatime, event),
            Event::Tick(deltatime) => GameTick::Tick(deltatime)
        };

        Ok(game_tick)
    }

    fn match_crossterm_event(deltatime: Duration, event: CEvent) -> GameTick {
        match event {
            CEvent::Key(key) => {
                let command = Self::match_key_event(key);
                GameTick::Command(deltatime, command)
            },
            CEvent::Mouse(_) => GameTick::Tick(deltatime), // TODO replace
            CEvent::Resize(_, _) => GameTick::Tick(deltatime) // TODO replace
        }
    }

    // TODO change to have changeable configs
    fn match_key_event(key: KeyEvent) -> Command {
        match key.code {
            KeyCode::Char('k') => Command::Up,
            KeyCode::Char('j') => Command::Down,
            KeyCode::Char('h') => Command::Left,
            KeyCode::Char('l') => Command::Right,
            KeyCode::Char('q') => Command::Quit,
            _ => Command::Up
        }
    }

}

/// Error for InputManager related errors
#[derive(Debug)]
pub enum InputManagerError {
    CrosstermError(crossterm::ErrorKind),
    RecvTimoutERror(mpsc::RecvTimeoutError),
    SendError,
}

impl From<crossterm::ErrorKind> for InputManagerError {

    fn from(error: crossterm::ErrorKind) -> Self {
        Self::CrosstermError(error)
    }
}

impl From<mpsc::RecvTimeoutError> for InputManagerError {

    fn from(error: mpsc::RecvTimeoutError) -> Self {
        Self::RecvTimoutERror(error)
    }

}

impl From<mpsc::SendError<InputEvent>> for InputManagerError {

    fn from(_: mpsc::SendError<InputEvent>) -> Self {
        Self::SendError
    }

}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utility::test_util::{crossterm_key, TestResult};
    use std::sync::mpsc::Receiver;
    use std::time::Duration;
    use std::fmt;
    use std::error::Error;
    use crate::game::source::FakeSource;

    /// Error for all timeout events
    #[derive(Debug, Clone)]
    struct InputTimeoutError(Duration);

    impl Error for InputTimeoutError {}

    impl fmt::Display for InputTimeoutError {

        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Timed out when waiting for input (waited for over {:?})", self.0)
        }

    }

    /// Convenienve method for creating the InputManager
    fn make_input_manager(events: Vec<CEvent>) -> InputManager {
        InputManager::new(FakeSource::new(events), Duration::from_millis(16), Duration::from_secs(1))
    }

    /// Convenience Wrapper of `get_input_manager_events`
    fn get_input_events(
        rx: &Receiver<InputEvent>,
        number_events: u32
    ) -> std::result::Result<Vec<InputEvent>, InputTimeoutError> {
        let single_event_timeout: Duration = Duration::from_secs(1);
        let total_timeout: Duration = Duration::from_secs(5);

        get_input_manager_events(rx, number_events, single_event_timeout, total_timeout)
    }

    /// Return a vector of all Crossterm Events received by the `Receiver` until `number_events` non-tick events were
    /// received
    ///
    /// - `number_events`: total number of NON-TICK events to wait on
    /// - `timeout`: longest time to wait on any one event
    /// - `total_timeout`: longest time to wait to receive all `number_events` events
    ///
    fn get_input_manager_events(
        rx: &Receiver<InputEvent>,
        number_events: u32,
        timeout: Duration,
        total_timeout: Duration
    ) -> std::result::Result<Vec<InputEvent>, InputTimeoutError> {
        let mut results = vec!();
        let mut events_so_far = 0;
        let mut accum_time = Duration::from_millis(0);

        while events_so_far < number_events {
            if accum_time > total_timeout {
                return Err(InputTimeoutError(accum_time));
            }

            let new_event = match rx.recv_timeout(timeout) {
                Ok(event) => event,
                Err(_) => return Err(InputTimeoutError(timeout))
            };

            match new_event {
                Event::Input(delta, _) => {
                    events_so_far += 1;
                    accum_time += delta;
                },
                Event::Tick(delta) => {
                    accum_time += delta;
                }
            }

            results.push(new_event);
        }

        Ok(results)
    }

    /// Returns `InputEvent` corresponding to keyboard input character
    fn key_event(letter: char) -> InputEvent {
        Event::Input(Duration::from_millis(0), crossterm_key(letter))
    }

    /// Compares `Event`s ignoring deltatime
    fn compare(former: &InputEvent, latter: &InputEvent) -> bool {
        match (former, latter) {
            (Event::Tick(_), Event::Tick(_)) => true,
            (Event::Input(_, former_key), Event::Input(_, latter_key)) => former_key == latter_key,
            _ => false
        }
    }

    /// Removes all `Event::Tick` from `events`, leaving just input events
    fn remove_ticks(events: Vec<InputEvent>) -> Vec<InputEvent> {
        events.into_iter().filter_map(|ev: InputEvent| {
            match ev {
                Event::Tick(_) => None,
                event => Some(event)
            }
        }).collect::<Vec<InputEvent>>()
    }

    /// Returns true if `events` contains `item`
    fn has_event(events: &Vec<InputEvent>, input_event: &InputEvent) -> bool {
        events.iter().any(|event| compare(&event, input_event))
    }

    #[test]
    fn test_simple_input() -> TestResult {
        let input_manager = make_input_manager(
            vec!(crossterm_key('k'))
        );

        let results = get_input_events(&input_manager.rx, 1)?;

        let key_pressed = key_event('k');
        assert!(has_event(&results, &key_pressed));

        Ok(())
    }

    #[test]
    fn test_no_input() -> TestResult {
        let input_manager = make_input_manager(vec!());

        let results = get_input_events(&input_manager.rx, 0)?;

        let all_ticks = results.into_iter().all(|ev: InputEvent| {
            ev == Event::Tick(Duration::from_millis(0))
        });

        assert!(all_ticks);

        Ok(())
    }

    #[test]
    fn test_multiple_keys() -> TestResult {
        let input_manager = make_input_manager(
            vec!(
               crossterm_key('a'),
               crossterm_key('b'),
               crossterm_key('c')
            )
        );

        let results = get_input_events(&input_manager.rx, 3)?;

        let no_ticks = remove_ticks(results);

        let has_proper_keys =
            has_event(&no_ticks, &key_event('a')) &&
            has_event(&no_ticks, &key_event('b')) &&
            has_event(&no_ticks, &key_event('c'));

        assert!(has_proper_keys);

        Ok(())
    }

    #[test]
    fn test_key_order() -> TestResult {
        let input_manager = make_input_manager(
            vec!(
               crossterm_key('1'),
               crossterm_key('2'),
               crossterm_key('3'),
            )
        );

        let results = get_input_events(&input_manager.rx, 3)?;
        let no_ticks = remove_ticks(results);

        let first_is_first = compare(&no_ticks[0], &key_event('1'));
        let second_is_second = compare(&no_ticks[1], &key_event('2'));
        let third_is_third = compare(&no_ticks[2], &key_event('3'));

        assert!(first_is_first && second_is_second && third_is_third);

        Ok(())
    }

}
