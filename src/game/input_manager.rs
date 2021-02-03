use crate::game::Command;
use crossterm::{
    event::{self, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

#[derive(PartialEq)]
enum Event<I> {
    Input(I),
    Tick,
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

                if event::poll(timeout).unwrap() {
                    let event = match event::read() {
                        Ok(result) => result,
                        _ => return
                    };
                    if let Err(_) = sx.send(Event::Input(event)) {
                        return
                    }
                }

                if last_tick.elapsed() >= tick_rate {
                    match sx.send(Event::Tick) {
                        Ok(()) => last_tick = Instant::now(),
                        _ => return
                    };
                }
            }
        });
    }

    /// Blocks until getting an input
    pub fn tick(&self) -> Result<Command, Box<dyn Error>> {
        let command = match self.rx.recv()? {
            Event::Input(event) => Self::match_crossterm_event(event),
            Event::Tick => Command::Tick(self.tick_rate)
        };

        Ok(command)
    }

    fn match_crossterm_event(event: CEvent) -> Command {
        match event {
            CEvent::Key(key) => Self::match_key_event(key),
            CEvent::Mouse(_) => Command::None,
            CEvent::Resize(_, _) => Command::None
        }
    }

    fn match_key_event(key: KeyEvent) -> Command {
        match key.code {
            KeyCode::Char('k') => Command::Up,
            KeyCode::Char('j') => Command::Down,
            KeyCode::Char('h') => Command::Left,
            KeyCode::Char('l') => Command::Right,
            _ => Command::None
        }
    }

}


#[cfg(test)]
mod test {
    use serial_test::serial;
    use std::sync::mpsc::Receiver;
    use super::*;
    use enigo::*;
    use std::time::Duration;

    type InputEvent = Event<CEvent>;

    /// Sets up the terminal for input testing
    fn setup() -> Result<(), Box<dyn Error>> {
        clear_inputs();
        enable_raw_mode()?;
        Ok(())
    }

    /// Cleans up the terminal after input testing
    fn tear_down() -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        Ok(())
    }

    /// Return a vector of all Crossterm Events received by the `Receiver`
    fn get_crossterm_events(rx: &Receiver<InputEvent>) -> Vec<InputEvent> {
        let mut results = vec!();

        while let Ok(command) = rx.try_recv() {
            results.push(command);
        }

        results
    }

    /// Returns standard Crossterm Key Event from a character (doesn't include Alt/Ctrl/etc.)
    fn crossterm_key(letter: char) -> CEvent {
        CEvent::Key(KeyEvent::from(KeyCode::Char(letter)))
    }

    /// Returns `Event<CEvent>` corresponding to keyboard input character
    fn key_event(letter: char) -> InputEvent {
        Event::Input(crossterm_key(letter))
    }

    /// Removes all `Event::Tick` from `events`, leaving just input events
    fn remove_ticks(events: Vec<InputEvent>) -> Vec<InputEvent> {
        events.into_iter().filter_map(|ev: InputEvent| {
            match ev {
                Event::Tick => None,
                event => Some(event)
            }
        }).collect::<Vec<InputEvent>>()
    }

    /// Returns true if `events` contains `item`
    fn has_event(events: &Vec<InputEvent>, item: &InputEvent) -> bool {
        events.iter().any(|ev: &InputEvent| *ev == *item)
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
    fn test_simple_input() -> Result<(), Box<dyn Error>> {
        setup()?;

        let mut enigo = Enigo::new();

        let input_rate = Duration::from_millis(16);
        let input_manager = InputManager::new(input_rate);

        thread::sleep(input_rate);
        enigo.key_down(Key::Layout('k'));
        thread::sleep(input_rate);

        let results = get_crossterm_events(&input_manager.rx);
        let key_pressed = crossterm_key('k');
        assert!(results.contains(&Event::Input(key_pressed)));

        tear_down()?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_no_input() -> Result<(), Box<dyn Error>> {
        setup()?;

        let input_rate = Duration::from_millis(16);
        let input_manager = InputManager::new(input_rate);

        thread::sleep(input_rate);

        let results = get_crossterm_events(&input_manager.rx);
        let all_ticks = results.into_iter().all(|ev: InputEvent| {
            ev == Event::Tick
        });
        assert!(all_ticks);

        tear_down()?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_multiple_keys() -> Result<(), Box<dyn Error>> {
        setup()?;
        let mut enigo = Enigo::new();

        let input_rate = Duration::from_millis(16);
        let input_manager = InputManager::new(Duration::from_millis(16));

        thread::sleep(input_rate);
        enigo.key_down(Key::Layout('a'));
        thread::sleep(input_rate);
        enigo.key_down(Key::Layout('b'));
        thread::sleep(input_rate);
        enigo.key_down(Key::Layout('c'));
        thread::sleep(input_rate);

        let results = get_crossterm_events(&input_manager.rx);
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
    fn test_key_order() -> Result<(), Box<dyn Error>> {
        setup()?;
        let mut enigo = Enigo::new();

        let input_rate = Duration::from_millis(16);
        let input_manager = InputManager::new(Duration::from_millis(16));

        thread::sleep(input_rate);
        enigo.key_down(Key::Layout('1'));
        enigo.key_down(Key::Layout('2'));
        enigo.key_down(Key::Layout('3'));
        thread::sleep(input_rate);

        let results = get_crossterm_events(&input_manager.rx);
        let no_ticks = remove_ticks(results);

        let first_is_first = no_ticks[0] == key_event('1');
        let second_is_second = no_ticks[1] == key_event('2');
        let third_is_third = no_ticks[2] == key_event('3');

        assert!(first_is_first && second_is_second && third_is_third);

        tear_down()?;
        Ok(())
    }

}
