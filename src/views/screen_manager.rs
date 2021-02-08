use crate::game::GameTick;
use crate::game::input_manager::InputManager;
use crate::game::input_manager::InputManagerError;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::enable_raw_mode;
use std::rc::Rc;
use std::io::{Stdout, Write};
use std::io;
use std::time::Duration;
use super::Popup;
use super::Screen;
use tui::Terminal as TuiTerminal;
use tui::Frame as TuiFrame;
use tui::backend::CrosstermBackend;

type Terminal = TuiTerminal<CrosstermBackend<Stdout>>;
type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;
type Result<T> = std::result::Result<T, ScreenManagerError>;

/// Screen Manager general error
#[derive(Debug)]
pub enum ScreenManagerError {
    IoError(io::Error),
    CrosstermError(crossterm::ErrorKind),
    InputManagerError(InputManagerError)
}

impl From<io::Error> for ScreenManagerError {

    fn from(error: io::Error) -> ScreenManagerError {
        ScreenManagerError::IoError(error)
    }

}

impl From<crossterm::ErrorKind> for ScreenManagerError {

    fn from(error: crossterm::ErrorKind) -> ScreenManagerError {
        ScreenManagerError::CrosstermError(error)
    }

}

impl From<InputManagerError> for ScreenManagerError {

    fn from(error: InputManagerError) -> ScreenManagerError {
        ScreenManagerError::InputManagerError(error)
    }

}

/// Manages screens and popups in the game, and controls which views get inputs
pub struct ScreenManager {
    screens: Vec<Box<dyn Screen>>,
    popups: Vec<Box<dyn Popup>>,
    input_manager: InputManager,
    terminal: Terminal,
    pub should_quit: bool
}

impl ScreenManager {

    /// Initializes a `ScreenManager` with no screens or popups
    pub fn new() -> Result<ScreenManager> {
        let tick_rate = Duration::from_millis(16);
        let tick_timeout = Duration::from_secs(1);
        let terminal = Self::setup_terminal()?;

        let screen_manager = ScreenManager {
            screens: Vec::new(),
            popups: Vec::new(),
            input_manager: InputManager::new(tick_rate, tick_timeout),
            terminal: terminal,
            should_quit: false
        };

        Ok(screen_manager)
    }

    /// Sets up stdout and creates a terminal using it
    fn setup_terminal() -> Result<Terminal> {
        let mut stdout = io::stdout();

        enable_raw_mode()?;
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;

        Ok(terminal)
    }

    pub fn start_main_loop(&mut self) -> Result<()> {
        while !self.should_quit {
            let tick = self.input_manager.tick()?;

            let screens = &self.screens;
            let popups = &self.popups;
            self.terminal.draw(move |f| {
                render(f, screens, popups, tick);
            })?;
        }

        Ok(())
    }

}

/// Renders the screen (popups and screen stack)
fn render(f: &mut Frame, screens: &Vec<Box<dyn Screen>>, popups: &Vec<Box<dyn Popup>>, tick: GameTick) {
    let tick = render_popups(f, popups, tick);
    render_screens(f, screens, tick);
}

/// Renders the topmost screen in the screen stack
fn render_screens(f: &mut Frame, screens: &Vec<Box<dyn Screen>>, tick: GameTick) {
    if let Some(top_screen) = screens.first() {
        top_screen.render(f, tick);
    }
}

/// Renders each of the popups, only allowing the topmost popup to get commands
fn render_popups(f: &mut Frame, popups: &Vec<Box<dyn Popup>>, tick: GameTick) -> GameTick {
    let mut tick = tick;

    for i in (0..popups.len()).rev() {
        let popup_screen = &popups[i];

        if i == 0 {
            // Render topmost popup with the full tick + command
            popup_screen.render(f, tick);
            // Downgrade tick as the command has been "used"
            tick = remove_input_from_tick(tick);
        } else {
            // render with just deltatime
            let deltatime_tick = remove_input_from_tick(tick);
            popup_screen.render(f, deltatime_tick);
        }
    }

    tick
}

/// "Downgrades" a `GameTick` from `GameTick::Command` to `GameTick::Tick`, maintaining the deltatime.
/// If it is already a `Tick`, just returns a copy
/// Mainly to give deltatime updates but avoid giving command updates to some screens.
///
fn remove_input_from_tick(tick: GameTick) -> GameTick {
    match tick {
        GameTick::Command(deltatime, _) => GameTick::Tick(deltatime),
        tick => tick,
    }
}
