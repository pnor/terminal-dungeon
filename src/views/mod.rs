mod screen;
mod popup;
mod screen_manager;

pub use screen::Screen;
pub use popup::Popup;
pub use screen_manager::ScreenManager;

pub type ScreenManagerCallback = dyn FnMut(&mut screen_manager::ScreenManager);
