use std::collections::VecDeque;
use std::time::Duration;
use crossterm::event::{self, Event};
use crossterm::ErrorKind;

type Result<T> = std::result::Result<T, ErrorKind>;

/// Abstraction of the source used for interacting with input events
pub trait Source {

    /// True if the `Source` has an input event to be read
    fn has_event(&self, timeout: Duration) -> bool;

    /// Reads a Crossterm Event from the source
    fn read(&mut self) -> Result<Event>;

}

/// `Source` implementation using Crossterm library functions to read from `stdin`
pub struct EventSource;

impl EventSource {

    pub fn new() -> Self {
        return EventSource;
    }
}

impl Source for EventSource {

    fn has_event(&self, timeout: Duration) -> bool {
        event::poll(timeout).unwrap()
    }

    fn read(&mut self) -> Result<Event> {
        event::read()
    }

}

/// `Source` implementation for debugging
pub struct FakeSource {
    events: VecDeque<Event>
}

impl FakeSource {

    pub fn new(mut events: Vec<Event>) -> Self {
        FakeSource { events: events.drain(0..).collect() }
    }
}

impl Source for FakeSource {


    fn has_event(&self, timeout: Duration) -> bool {
        !self.events.is_empty()
    }

    fn read(&mut self) -> Result<Event> {
        let io_error = ErrorKind::IoError(std::io::Error::new(std::io::ErrorKind::Other, "Error in FakeSource!"));
        self.events.pop_front().ok_or(io_error)
    }

}
