use std::io::Stdout;
use tui::backend::Backend;
use tui::backend::CrosstermBackend;
use tui::Frame;
use std::time::Duration;
use crate::game::GameTick;

/// Defines a primary view, that shows various widgets
pub trait Screen {

    /// Instantiates new instance of this screen
    fn new() -> Self where Self:Sized;

    /// Renders the screen onto the terminal
    fn render(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>, tick: GameTick);

    /// Performs clean up when screen is dropped
    fn tear_down(&mut self) {}

}
