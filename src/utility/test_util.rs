use crossterm::event;
use std::error::Error;
use std::time::Duration;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub type TestResult = std::result::Result<(), Box<dyn Error>>;

/// Sets up terminal for test that involves receiving input
pub fn setup_input_test() -> TestResult {
    clear_inputs(Duration::from_millis(100));
    enable_raw_mode()?;
    Ok(())
}

/// Cleans up terminal after a test that involved input
pub fn tear_down_input_test() -> TestResult {
    disable_raw_mode()?;
    Ok(())
}

/// Reads all events until there are none left
/// - `timeout` is the longest to wait for any event
pub fn clear_inputs(timeout: Duration) {
    while event::poll(timeout).unwrap() {
        let _ = event::read();
    }
}
