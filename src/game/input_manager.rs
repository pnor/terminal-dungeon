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
    use std::io;
    use enigo::*;
    use std::time::Duration;

    /// Return a vector of all Crossterm Events received by the `Receiver`
    fn get_crossterm_events(rx: &Receiver<Event<CEvent>>) -> Vec<Event<CEvent>> {
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

    fn has_event<'a, I>(events: &I, item: &Event<CEvent>) -> bool
    where I: Iterator<Item = &'a Event<CEvent>>
    {
        true
    }

    /// Reads all events until there are none left
    ///
    /// - `timeout` is the longest to wait for any event
    fn clear_inputs() {
        while event::poll(Duration::from_millis(100)).unwrap() {
            let _ = event::read();
        }
    }

    #[test]
    #[serial]
    fn test_simple_input() -> Result<(), Box<dyn Error>> {
        clear_inputs();
        enable_raw_mode()?;
        let mut enigo = Enigo::new();

        let input_manager = InputManager::new(Duration::from_millis(16));

        thread::sleep(Duration::from_millis(100));
        enigo.key_down(Key::Layout('k'));
        thread::sleep(Duration::from_millis(100));

        let results = get_crossterm_events(&input_manager.rx);
        let key_pressed = crossterm_key('k');
        assert!(results.contains(&Event::Input(key_pressed)));

        disable_raw_mode()?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_no_input() -> Result<(), Box<dyn Error>> {
        clear_inputs();
        enable_raw_mode()?;

        let input_manager = InputManager::new(Duration::from_millis(16));

        thread::sleep(Duration::from_millis(100));

        let results = get_crossterm_events(&input_manager.rx);
        for res in &results {
            match res {
                Event::Tick => println!("..."),
                Event::Input(e) => {
                    match e {
                        CEvent::Key(key) => println!("key: {:?}", key.code),
                            _ => println!("?")
                    }
                    // println!("input")
                }
            }
        }
        let all_ticks = results.into_iter().all(|ev: Event<CEvent>| {
            ev == Event::Tick
        });
        assert!(all_ticks);

        disable_raw_mode()?;
        Ok(())
    }

    #[test]
    #[serial]
    fn test_multiple_keys() -> Result<(), Box<dyn Error>> {
        clear_inputs();
        enable_raw_mode()?;
        let mut enigo = Enigo::new();

        let input_manager = InputManager::new(Duration::from_millis(16));

        thread::sleep(Duration::from_millis(20));
        enigo.key_down(Key::Layout('a'));
        thread::sleep(Duration::from_millis(20));
        enigo.key_down(Key::Layout('b'));
        thread::sleep(Duration::from_millis(20));
        enigo.key_down(Key::Layout('c'));

        let results = get_crossterm_events(&input_manager.rx);
        let filtered_results = results.iter().filter_map(|ev: &Event<CEvent>| {
            match ev {
                Event::Tick => None,
                event => Some(event)
            }
        });

        let a_key = Event::Input(crossterm_key('a'));
        let b_key = Event::Input(crossterm_key('b'));
        let c_key = Event::Input(crossterm_key('c'));

        let has_proper_keys =
            has_event(&filtered_results, &a_key) &&
            has_event(&filtered_results, &b_key) &&
            has_event(&filtered_results, &c_key);


        assert!(has_proper_keys);

        disable_raw_mode()?;
        Ok(())
    }
}
