use tui::Frame as TuiFrame;
use tui::Terminal as TuiTerminal;
use tui::backend::CrosstermBackend;
use std::io::Stdout;

pub mod screens;

mod screen;
mod popup;
mod screen_manager;

pub use screen::Screen;
pub use popup::Popup;
pub use screen_manager::ScreenManager;

// TODO move this into screen manager (and change impl)
type Terminal = TuiTerminal<CrosstermBackend<Stdout>>;
type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;
