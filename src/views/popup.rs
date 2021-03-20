use std::collections::VecDeque;
use crate::game::GameTick;
use std::io::Stdout;
use super::ScreenManagerCallback;
use tui::Frame;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;

/// Defines a popup view that is showed on top existing displays
pub trait Popup {

    // Instantiates new popup instance
    fn new() -> Self where Self:Sized;

    /// Renders the screen onto the terminal
    fn render(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>, tick: GameTick);

    /// Returns `Rect` representing where on screen it'll draw its contents
    fn draw_location(&self) -> Rect;

    /// Performs clean up when screen is dropped
    fn tear_down(&mut self) {}

    /// Add a `ScreenManager` function to be called after next loop
    fn add_screen_manager_callback(&mut self, callback: Box<ScreenManagerCallback>);

    /// Get all queued `ScreenManager` functions to be called
    fn get_screen_manager_callbacks(&mut self) -> VecDeque<Box<ScreenManagerCallback>>;

}
