use std::sync::mpsc::Receiver;
use crossterm::event;
use std::error::Error;
use std::time::Duration;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub type TestResult = std::result::Result<(), Box<dyn Error>>;

/// Returns standard Crossterm Key Event from a character (doesn't include Alt/Ctrl/etc.)
pub fn crossterm_key(letter: char) -> event::Event {
    event::Event::Key(event::KeyEvent::from(event::KeyCode::Char(letter)))
}

/// Reads all events until there are none left
/// - `timeout` is the longest to wait for any event
pub fn clear_inputs(timeout: Duration) {
    while event::poll(timeout).unwrap() {
        let _ = event::read();
    }
}
