use crate::game::{GameTick, Command};
use crossterm::event::{self, Event as CEvent, KeyCode, KeyEvent};
use std::{
    error::Error,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

/// Abstraction that tracks time between the last input event and Crossterm Events
#[derive(PartialEq, Debug)]
enum Event<I> {
    Input(Duration, I),
    Tick(Duration),
}

/// Manages user input by polling on another thread
pub struct InputManager {
    rx: mpsc::Receiver<Event<CEvent>>,
    tick_rate: Duration,
}

impl InputManager {

    /// Creates `InputManager` and starts asynchronously polling user input
    pub fn new(tick_rate: Duration) -> Self {
        let (sx, rx) = mpsc::channel();
        let input_manager = InputManager {
            rx,
            tick_rate
        };

        input_manager.start_async_polling(sx);

        input_manager
    }

    fn start_async_polling(&self, sx: mpsc::Sender<Event<CEvent>>) {
        let tick_rate = self.tick_rate;
        thread::spawn(move || {
            let mut last_tick = Instant::now();

            loop {
                // poll for tick rate duration. If no evetns, send tick event
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                // if has event, send it
                if event::poll(timeout).unwrap() {
                    let time_since_last_tick = Instant::now() - last_tick;
                    let event = match event::read() {
                        Ok(result) => result,
                        _ => return
                    };
                    if let Err(_) = sx.send(Event::Input(time_since_last_tick, event)) {
                        return
                    }
                }

                // Send a Tick since `tick_rate` has passed
                if last_tick.elapsed() >= tick_rate {
                    match sx.send(Event::Tick(tick_rate)) {
                        Ok(()) => last_tick = Instant::now(),
                        _ => return
                    };
                }
            }
        });
    }

    /// Blocks until getting an input
    pub fn tick(&self) -> Result<GameTick, Box<dyn Error>> {
        let game_tick = match self.rx.recv()? {
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

    // TODO rename
    fn match_key_event(key: KeyEvent) -> Command {
        match key.code {
            KeyCode::Char('k') => Command::Up,
            KeyCode::Char('j') => Command::Down,
            KeyCode::Char('h') => Command::Left,
            KeyCode::Char('l') => Command::Right,
            _ => Command::Up
        }
    }

}


#[cfg(test)]
mod test {
    use super::*;
    use serial_test::serial;
    use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
    use enigo::*;
    use std::sync::mpsc::{Receiver};
    use std::time::Duration;
    use std::fmt;

    type InputEvent = Event<CEvent>;
    type TestResult = Result<(), Box<dyn Error>>;

    /// Error for all timeout events
    #[derive(Debug, Clone)]
    struct InputTimeoutError(Duration);

    impl Error for InputTimeoutError {}

    impl fmt::Display for InputTimeoutError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Timed out when waiting for input (waited for over {:?})", self.0)
        }
    }

    /// Sets up the terminal for input testing
    fn setup() -> TestResult {
        clear_inputs();
        enable_raw_mode()?;
        Ok(())
    }

    /// Cleans up the terminal after input testing
    fn tear_down() -> TestResult {
        disable_raw_mode()?;
        Ok(())
    }

    /// Shorter to call wrapper of `get_input_manager_events`
    fn get_input_events(rx: &Receiver<InputEvent>, number_events: u32) -> Result<Vec<InputEvent>, InputTimeoutError> {
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
    ) -> Result<Vec<InputEvent>, InputTimeoutError> {
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

    /// Returns standard Crossterm Key Event from a character (doesn't include Alt/Ctrl/etc.)
    fn crossterm_key(letter: char) -> CEvent {
        CEvent::Key(KeyEvent::from(KeyCode::Char(letter)))
    }

    /// Returns `Event<CEvent>` corresponding to keyboard input character
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

    /// Reads all events until there are none left
    /// - `timeout` is the longest to wait for any event
    fn clear_inputs() {
        while event::poll(Duration::from_millis(100)).unwrap() {
            let _ = event::read();
        }
    }

    #[test]
    #[serial]
    fn test_simple_input() -> TestResult {
        setup()?;

        let mut enigo = Enigo::new();

        let input_manager = InputManager::new(Duration::from_millis(16));

        enigo.key_down(Key::Layout('k'));

        let results = get_input_events(&input_manager.rx, 1)?;

        let key_pressed = key_event('k');
        assert!(has_event(&results, &key_pressed));

        tear_down()?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_no_input() -> TestResult {
        setup()?;

        let input_rate = Duration::from_millis(16);
        let input_manager = InputManager::new(input_rate);

        let results = get_input_events(&input_manager.rx, 0)?;

        let all_ticks = results.into_iter().all(|ev: InputEvent| {
            ev == Event::Tick(Duration::from_millis(0))
        });

        assert!(all_ticks);

        tear_down()?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_multiple_keys() -> TestResult {
        setup()?;
        let mut enigo = Enigo::new();

        let input_rate = Duration::from_millis(16);
        let input_manager = InputManager::new(Duration::from_millis(16));

        enigo.key_down(Key::Layout('a'));
        enigo.key_down(Key::Layout('b'));
        enigo.key_down(Key::Layout('c'));

        let results = get_input_events(&input_manager.rx, 3)?;

        let no_ticks = remove_ticks(results);

        let has_proper_keys =
            has_event(&no_ticks, &key_event('a')) &&
            has_event(&no_ticks, &key_event('b')) &&
            has_event(&no_ticks, &key_event('c'));


        assert!(has_proper_keys);

        tear_down()?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_key_order() -> TestResult {
        setup()?;
        let mut enigo = Enigo::new();

        let input_manager = InputManager::new(Duration::from_millis(16));

        enigo.key_down(Key::Layout('1'));
        enigo.key_down(Key::Layout('2'));
        enigo.key_down(Key::Layout('3'));

        let results = get_input_events(&input_manager.rx, 3)?;
        let no_ticks = remove_ticks(results);

        let first_is_first = compare(&no_ticks[0], &key_event('1'));
        let second_is_second = compare(&no_ticks[1], &key_event('2'));
        let third_is_third = compare(&no_ticks[2], &key_event('3'));

        assert!(first_is_first && second_is_second && third_is_third);

        tear_down()?;
        Ok(())
    }

}
