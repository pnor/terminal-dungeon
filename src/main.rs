mod world;
mod entities;
mod utility;
mod systems;
mod game;
mod views;

use crossterm::terminal::Clear;
use crossterm::event::EnableMouseCapture;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::ClearType;

use crate::views::Screen;
use crate::views::screens::GameScreen;
use crate::views::ScreenManager;

use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::execute;
use std::error::Error;

use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    setup_ui()?;

    let mut screen_manager = ScreenManager::new()?;

    let game_screen = GameScreen::new();

    screen_manager.push_screen(game_screen);

    screen_manager.start_main_loop()?;

    disable_raw_mode()?;

    Ok(())
}

/// Sets up stdout for drawing
fn setup_ui() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    write!(stdout, "{}", Clear(ClearType::All))?;

    Ok(())
}
