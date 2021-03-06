use std::collections::VecDeque;
use crate::game::GameTick;
use super::screen_manager::BoxedCallback;
use tui::layout::Rect;
use super::Frame;

/// Defines a popup view that is showed on top existing displays
pub trait Popup {

    // Instantiates new popup instance
    fn new() -> Self where Self:Sized;

    /// Renders the screen onto the terminal
    fn render(&mut self, frame: &mut Frame, tick: GameTick);

    /// Returns `Rect` representing where on screen it'll draw its contents
    fn draw_location(&self) -> Rect;

    /// Performs clean up when screen is dropped
    fn tear_down(&mut self) {}

    /// Add a `ScreenManager` function to be called after next loop
    fn add_screen_manager_callback(&mut self, callback: BoxedCallback);

    /// Get all queued `ScreenManager` functions to be called
    fn get_screen_manager_callbacks(&mut self) -> VecDeque<BoxedCallback>;

}
