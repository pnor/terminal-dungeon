use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Frame as TuiFrame;

pub mod screens;

mod screen;
mod popup;
mod screen_manager;

pub use screen::Screen;
pub use popup::Popup;
pub use screen_manager::ScreenManager;

type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;
