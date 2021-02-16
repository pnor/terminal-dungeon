use std::error::Error;
use crate::game::GameTick;
use crate::game::input_manager::InputManager;
use crate::game::input_manager::InputManagerError;
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::enable_raw_mode;
use std::io::{Stdout, Write};
use std::io;
use std::time::Duration;
use std::fmt;
use super::Popup;
use super::Screen;
use tui::Terminal as TuiTerminal;
use tui::Frame as TuiFrame;
use tui::backend::CrosstermBackend;

type Terminal = TuiTerminal<CrosstermBackend<Stdout>>;
type Frame<'a> = TuiFrame<'a, CrosstermBackend<Stdout>>;
type Result<T> = std::result::Result<T, ScreenManagerError>;

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

    /// Starts the main render loop, which polls for input and sends the deltatime and user input to the
    /// screens/popups
    pub fn start_main_loop(&mut self) -> Result<()> {
        while !self.should_quit {
            let tick = self.input_manager.tick()?;
            let screens = &mut self.screens;
            let popups = &mut self.popups;

            self.terminal.draw(move |f| {
                render(f, screens, popups, tick);
            })?;
        }

        Ok(())
    }

    /// Push a new `Screen` to the top of the Screen stack
    pub fn push_screen(&mut self, screen: Box<dyn Screen>) {
        self.screens.push(screen);
    }

    /// Pops a `Screen` from the Screen stack
    pub fn pop_screen(&mut self) -> Option<Box<dyn Screen>> {
        self.screens.pop()
    }

    /// Push a new `Popup` to the top of the Popup stack
    pub fn push_popup(&mut self, popup: Box<dyn Popup>) {
        self.popups.push(popup);
    }

    /// Pops a `Popup` from the Popup stack
    pub fn pop_popup(&mut self) -> Option<Box<dyn Popup>> {
        self.popups.pop()
    }

}

impl Drop for ScreenManager {

    /// Calls tear_down on all screens and popups before they are dropped
    fn drop(&mut self) {
        for screen in &mut self.screens {
            screen.tear_down();
        }

        for popup in &mut self.popups {
            popup.tear_down();
        }
    }

}

/// Renders the screen (popups and screen stack)
fn render(f: &mut Frame, screens: &mut Vec<Box<dyn Screen>>, popups: &mut Vec<Box<dyn Popup>>, tick: GameTick) {
    let tick = render_popups(f, popups, tick);
    render_screens(f, screens, tick);
}

/// Renders the topmost screen in the screen stack
fn render_screens(f: &mut Frame, screens: &mut Vec<Box<dyn Screen>>, tick: GameTick) {
    if let Some(top_screen) = screens.first_mut() {
        top_screen.render(f, tick);
    }
}

/// Renders each of the popups, only allowing the topmost popup to get commands
fn render_popups(f: &mut Frame, popups: &mut Vec<Box<dyn Popup>>, tick: GameTick) -> GameTick {
    let mut tick = tick;

    for i in (0..popups.len()).rev() {
        let popup_screen = &mut popups[i];

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

/// Screen Manager general error
#[derive(Debug)]
pub enum ScreenManagerError {
    IoError(io::Error),
    CrosstermError(crossterm::ErrorKind),
    InputManagerError(InputManagerError)
}

impl Error for ScreenManagerError {}

impl fmt::Display for ScreenManagerError {

    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        write!(
            f,
            "Encountered error when using the Screen Manager: {:?}",
            self
        )
    }

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

#[cfg(test)]
mod test {
    use std::error::Error;
    use std::sync::mpsc;
    use std::rc::Rc;
    use crate::game::Command;
    use super::*;

    static DUMMY_TICK: GameTick = GameTick::Tick(Duration::from_secs(1));

    type TestResult = std::result::Result<(), Box<dyn Error>>;

    struct TestScreen {
        sx: mpsc::Sender<GameTick>,
        pub rx_rc: Rc<mpsc::Receiver<GameTick>>
    }

    impl Screen for TestScreen {

        fn new() -> Self {
            let (sx, rx) = mpsc::channel();
            let rx_rc = Rc::new(rx);

            TestScreen {
                sx,
                rx_rc
            }
        }

        fn render(&mut self, _: &mut Frame, tick: GameTick) {
            self.sx.send(tick).unwrap();
        }

        fn tear_down(&mut self) {
            self.sx.send(DUMMY_TICK).unwrap();
        }

    }

    struct TestPopup {
        sx: mpsc::Sender<GameTick>,
        pub rx_rc: Rc<mpsc::Receiver<GameTick>>
    }

    impl Popup for TestPopup {

        fn new() -> Self {
            let (sx, rx) = mpsc::channel();
            let rx_rc = Rc::new(rx);

            TestPopup {
                sx,
                rx_rc
            }
        }

        fn render(&mut self, frame: &mut Frame, tick: GameTick) {
            self.sx.send(tick).unwrap();
        }

        fn draw_location(&self) -> tui::layout::Rect {
            tui::layout::Rect::new(0, 0, 10, 10)
        }

        fn tear_down(&mut self) {
            self.sx.send(DUMMY_TICK).unwrap();
        }

    }

    #[test]
    fn test_new() -> TestResult {
        let _ = ScreenManager::new()?;
        Ok(())
    }

    #[test]
    fn test_push_screen() -> TestResult {
        let mut screen_manager = ScreenManager::new()?;

        let test_screen = TestScreen::new();
        screen_manager.push_screen(Box::new(test_screen));

        assert_eq!(screen_manager.screens.len(), 1);

        Ok(())
    }

    #[test]
    fn test_pop_screen() -> TestResult {
        let mut screen_manager = ScreenManager::new()?;

        let test_screen_1 = TestScreen::new();
        let test_screen_2 = TestScreen::new();

        let screen_1_rx_rc = test_screen_1.rx_rc.clone();

        screen_manager.push_screen(Box::new(test_screen_2));
        screen_manager.push_screen(Box::new(test_screen_1));

        let mut popped = screen_manager.pop_screen().unwrap();

        // Test pop changes length
        assert_eq!(screen_manager.screens.len(), 1);

        // Test popped thing is the top of stack (by checking the write popup changed)
        popped.as_mut().tear_down();
        assert_eq!(screen_1_rx_rc.try_recv(), Ok(DUMMY_TICK));

        Ok(())
    }

    #[test]
    fn test_push_popup() -> TestResult {
        let mut screen_manager = ScreenManager::new()?;

        let test_popup = TestPopup::new();
        screen_manager.push_popup(Box::new(test_popup));

        assert_eq!(screen_manager.popups.len(), 1);

        Ok(())
    }

    #[test]
    fn test_pop_popup() -> TestResult {
        let mut screen_manager = ScreenManager::new()?;

        let test_popup_1 = TestPopup::new();
        let test_popup_2 = TestPopup::new();

        let popup_1_rx_rc = test_popup_1.rx_rc.clone();

        screen_manager.push_popup(Box::new(test_popup_2));
        screen_manager.push_popup(Box::new(test_popup_1));

        let mut popped = screen_manager.pop_popup().unwrap();

        // Test pop changes length
        assert_eq!(screen_manager.popups.len(), 1);

        // Test popped thing is the top of stack (by checking the write popup changed)
        popped.as_mut().tear_down();
        assert_eq!(popup_1_rx_rc.try_recv(), Ok(DUMMY_TICK));

        Ok(())
    }


    #[test]
    fn test_tear_down_called() -> TestResult {
        let test_screen = TestScreen::new();
        let test_screen_rx = test_screen.rx_rc.clone();

        let test_popup = TestPopup::new();
        let test_popup_rx = test_popup.rx_rc.clone();

        {
            let mut screen_manager = ScreenManager::new()?;

            screen_manager.push_screen(Box::new(test_screen));
            screen_manager.push_popup(Box::new(test_popup));
        }

        // Screen manager is dropped
        assert_eq!(test_screen_rx.try_recv(), Ok(DUMMY_TICK));
        assert_eq!(test_popup_rx.try_recv(), Ok(DUMMY_TICK));

        Ok(())
    }

    /// Test commands handled correctly with screens
    #[test]
    fn test_render_screens() -> TestResult {
        let mut screen_manager = ScreenManager::new()?;

        // Create screens
        let screen_top = TestScreen::new();
        let screen_bottom = TestScreen::new();
        let rc_rx_top = screen_top.rx_rc.clone();
        let rc_rx_bottom = screen_bottom.rx_rc.clone();

        // topdd it to manager
        screen_manager.push_screen(Box::new(screen_bottom));
        screen_manager.push_screen(Box::new(screen_top));

        // Create the test tick in question
        let test_tick = GameTick::Command(Duration::from_millis(100), Command::Up);

        // call render once
        let screens = &mut screen_manager.screens;
        let popups = &mut screen_manager.popups;
        screen_manager.terminal.draw(move |f| {
            render(f, screens, popups, test_tick);
        })?;

        // Test the current things got what they needed
        assert_eq!(rc_rx_top.try_recv(), Ok(test_tick));
        assert_eq!(rc_rx_bottom.try_recv().err(), None);

        Ok(())
    }

    /// Test commands handled correctly with screens and popups
    // #[test] TODO
    fn test_render_popup_and_screen() -> TestResult {
        todo!()

    }

}
