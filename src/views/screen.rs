use tui::backend::CrosstermBackend;
use tui::Frame;
use std::time::Duration;

/// Defines a primary view, that shows various widgets
trait Screen {

    /// Instantiates new instance of this screen
    fn new() -> Self;

    /// Sets up the screen for rendering
    fn setup(&self);

    /// Renders the screen onto the terminal
    fn render<B: Backend>(&self, frame: &mut Frame<B>, deltatime: Duration);

    /// Performs clean up when screen is dropped
    fn tear_down(&self);

}

impl<T: Drop> T {

    fn drop(&mut self) {
        self.tear_down()
    }

}
