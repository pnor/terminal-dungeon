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

pub type ScreenManagerCallback = dyn FnMut(&mut screen_manager::ScreenManager);
type Terminal = TuiTerminal<CrosstermBackend<Stdout>>;
type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;
