use tui::backend::CrosstermBackend;
use std::error::Error;
use std::collections::VecDeque;
use crate::game::GameTick;
use crate::game::input_manager::InputManager;
use crate::game::input_manager::InputManagerError;
use crate::game::source::{Source, FakeSource, EventSource};
use crossterm::event::{EnableMouseCapture, Event};
use std::io::{Stdout, Write};
use std::io;
use std::time::Duration;
use std::fmt;
use super::Popup;
use super::Screen;
use super::ScreenManagerCallback;
use super::{Terminal, Frame};

type Result<T> = std::result::Result<T, ScreenManagerError>;
type Callback = Box<ScreenManagerCallback>;

/// Manages screens and popups in the game, and controls which views get inputs
/// To properly draw, should enable raw mode on terminal before use (and clear screen)
pub struct ScreenManager {
    screens: Vec<Box<dyn Screen>>,
    popups: Vec<Box<dyn Popup>>,
    input_manager: InputManager,
    terminal: Terminal,
    callback_queue: VecDeque<Callback>,
    pub should_quit: bool,
}

impl ScreenManager {

    pub fn new() -> Result<ScreenManager> {
        ScreenManager::init(EventSource::new())
    }

    pub fn debug_new(events: Vec<Event>) -> Result<ScreenManager> {
        ScreenManager::init(FakeSource::new(events))
    }

    /// Initializes a `ScreenManager` with no screens or popups
    fn init(source: impl Source + Send + 'static) -> Result<ScreenManager> {
        let tick_rate = Duration::from_millis(16);
        let tick_timeout = Duration::from_secs(1);

        let terminal = Self::setup_terminal()?;

        let screen_manager = ScreenManager {
            screens: Vec::new(),
            popups: Vec::new(),
            input_manager: InputManager::new(source, tick_rate, tick_timeout),
            terminal: terminal,
            callback_queue: VecDeque::<Callback>::new(),
            should_quit: false
        };

        Ok(screen_manager)
    }

    /// Creates terminal using Crossterm Backend for drawing
    fn setup_terminal() -> Result<Terminal> {
        let mut stdout = io::stdout();

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;

        Ok(terminal)
    }

    /// Starts the main render loop, which polls for input and sends the deltatime and user input to the
    /// screens/popups.
    /// After each render loop, calls any added callbacks from screens and popups
    ///
    /// Callbacks are called in order of the Popup stack, then the Screen stack, both from top to bottom. As each is
    /// called, it is removed from the queue
    pub fn start_main_loop(&mut self) -> Result<()> {
        while !self.should_quit {
            let tick = self.input_manager.tick()?;
            let screens = &mut self.screens;
            let popups = &mut self.popups;

            self.terminal.draw(move |f| {
                render(f, screens, popups, tick);
            })?;

            self.handle_callbacks();
        }

        Ok(())
    }

    /// Updates the queue using the callbacks of the screens and popups, and calls each in turn
    fn handle_callbacks(&mut self) {
        self.update_callback_queue();

        let callbacks: Vec<Box<ScreenManagerCallback>> = self.callback_queue.drain(0..).collect();
        for mut callback in callbacks {
            callback(self);
        }
    }

    /// Udates the queue with callbacks of screens and popups
    /// The order is popup callbacks first then screeen popups, both in order of their stacks
    fn update_callback_queue(&mut self) {
        self.callback_queue.clear();

        for popup in &mut self.popups {
            self.callback_queue.append(&mut popup.as_mut().get_screen_manager_callbacks());
        }

        for screen in &mut self.screens {
            self.callback_queue.append(&mut screen.as_mut().get_screen_manager_callbacks());
        }
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
///
/// If there are both popups and screens, then if the `tick` was a Command, the topmost Popup will receive
/// the Command, while the rest of the popups just receive the deltatime Tick.
/// Then the topmost screen will also receive the deltatime Tick, while the rest of the screens don't receive
/// anything
fn render(f: &mut Frame, screens: &mut Vec<Box<dyn Screen>>, popups: &mut Vec<Box<dyn Popup>>, tick: GameTick) {
    let tick = render_popups(f, popups, tick);
    render_screens(f, screens, tick);
}

/// Renders the topmost screen in the screen stack (not giving any other screens the deltatime Tick)
fn render_screens(f: &mut Frame, screens: &mut Vec<Box<dyn Screen>>, tick: GameTick) {
    if let Some(top_screen) = screens.last_mut() {
        top_screen.render(f, tick);
    }
}

/// Renders each of the popups, only allowing the topmost popup to get commands, while giving popups below it just
/// the deltatime Tick
fn render_popups(f: &mut Frame, popups: &mut Vec<Box<dyn Popup>>, tick: GameTick) -> GameTick {
    let mut tick = tick;

    let popups_length = popups.len();

    for i in 0..popups_length {
        let popup_screen = &mut popups[i];

        if i == popups_length - 1 {
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
    use ntest::timeout;
    use crate::utility::test_util::crossterm_key;
    use std::error::Error;
    use std::sync::mpsc;
    use std::rc::Rc;
    use crate::game::Command;
    use std::collections::VecDeque;
    use super::*;

    static DUMMY_TICK: GameTick = GameTick::Tick(Duration::from_secs(1));

    type TestResult = std::result::Result<(), Box<dyn Error>>;

    /// Testing Screen that quits after a set amount of renders
    struct TestScreen {
        sx: mpsc::Sender<GameTick>,
        pub rx_rc: Rc<mpsc::Receiver<GameTick>>,
        callbacks: VecDeque<Box<ScreenManagerCallback>>,
        pub render_counter: i32,
    }

    impl Screen for TestScreen {

        fn new() -> Self {
            let (sx, rx) = mpsc::channel();
            let rx_rc = Rc::new(rx);

            TestScreen {
                sx,
                rx_rc,
                callbacks: VecDeque::<Callback>::new(),
                render_counter: 0,
            }
        }

        fn render(&mut self, _: &mut Frame, tick: GameTick) {
            self.sx.send(tick).unwrap();
            self.render_counter += 1;

            // After 10 ticks, tell the screen manager to quit
            if self.render_counter > 10 {
                self.add_screen_manager_callback(Box::new(|s: &mut ScreenManager| {
                    s.should_quit = true;
                }));
            }
        }

        fn tear_down(&mut self) {
            self.sx.send(DUMMY_TICK).unwrap();
        }

        fn add_screen_manager_callback(&mut self, callback: Box<ScreenManagerCallback>) {
            self.callbacks.push_front(callback)
        }

        fn get_screen_manager_callbacks(&mut self) -> VecDeque<Box<ScreenManagerCallback>> {
            self.callbacks.drain(0..).collect()
        }

    }

    /// Testing Popup that quits after a set amount of renders
    struct TestPopup {
        sx: mpsc::Sender<GameTick>,
        pub rx_rc: Rc<mpsc::Receiver<GameTick>>,
        callbacks: VecDeque<Box<ScreenManagerCallback>>,
        pub render_counter: i32,
    }

    impl Popup for TestPopup {

        fn new() -> Self {
            let (sx, rx) = mpsc::channel();
            let rx_rc = Rc::new(rx);

            TestPopup {
                sx,
                rx_rc,
                callbacks: VecDeque::<Callback>::new(),
                render_counter: 0
            }
        }

        fn render(&mut self, frame: &mut Frame, tick: GameTick) {
            self.sx.send(tick).unwrap();
            self.render_counter += 1;

            // After 10 ticks, tell screen manager to quit
            if self.render_counter > 10 {
                self.add_screen_manager_callback(Box::new(|s: &mut ScreenManager| {
                    s.should_quit = true;
                }));
            }
        }

        fn draw_location(&self) -> tui::layout::Rect {
            tui::layout::Rect::new(0, 0, 10, 10)
        }

        fn tear_down(&mut self) {
            self.sx.send(DUMMY_TICK).unwrap();
        }

        fn add_screen_manager_callback(&mut self, callback: Box<ScreenManagerCallback>) {
            self.callbacks.push_front(callback)
        }

        fn get_screen_manager_callbacks(&mut self) -> VecDeque<Box<ScreenManagerCallback>> {
            self.callbacks.drain(0..).collect()
        }

    }

    /// Gets all ticks from a Receiver
    fn get_ticks_from_rx(rx: &mpsc::Receiver<GameTick>) -> Vec<GameTick> {
        let mut ticks = Vec::new();

        while let Some(tick) = rx.try_recv().ok() {
            ticks.push(tick);
        }

        ticks
    }


    /// Returns `true` if the `tick` is `GameTick::Command` and has a command that is the same
    /// as `command`
    fn tick_has_command(tick: GameTick, command: Command) -> bool {
        if let GameTick::Command(_, tick_command) = tick {
            tick_command == command
        } else {
            false
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

        // create screens
        let screen_top = TestScreen::new();
        let screen_bottom = TestScreen::new();
        let top_rx = screen_top.rx_rc.clone();
        let bottom_rx = screen_bottom.rx_rc.clone();

        // add it to manager
        screen_manager.push_screen(Box::new(screen_bottom));
        screen_manager.push_screen(Box::new(screen_top));

        // create the test tick in question
        let test_tick = GameTick::Command(Duration::from_millis(100), Command::Up);

        // call render once
        let screens = &mut screen_manager.screens;
        let popups = &mut screen_manager.popups;
        screen_manager.terminal.draw(move |f| {
            render(f, screens, popups, test_tick);
        })?;

        // test the current things got what they needed
        assert_eq!(top_rx.try_recv().ok(), Some(test_tick));
        assert_eq!(bottom_rx.try_recv().ok(), None);

        Ok(())
    }

    /// Test commands handled correctly with screens and popups
    #[test]
    fn test_render_popup_and_screen() -> TestResult {
        let mut screen_manager = ScreenManager::new()?;

        // create screens
        let screen_top = TestScreen::new();
        let screen_bottom = TestScreen::new();
        let popup_top = TestPopup::new();
        let popup_bottom = TestPopup::new();

        let screen_top_rx = screen_top.rx_rc.clone();
        let screen_bottom_rx = screen_bottom.rx_rc.clone();
        let popup_top_rx = popup_top.rx_rc.clone();
        let popup_bottom_rx = popup_bottom.rx_rc.clone();

        // add it to manager
        screen_manager.push_popup(Box::new(popup_bottom));
        screen_manager.push_popup(Box::new(popup_top));

        screen_manager.push_screen(Box::new(screen_bottom));
        screen_manager.push_screen(Box::new(screen_top));

        // create the test tick in question
        let test_tick = GameTick::Command(Duration::from_millis(100), Command::Up);
        let deltatime_tick = remove_input_from_tick(test_tick);

        // call render once
        let screens = &mut screen_manager.screens;
        let popups = &mut screen_manager.popups;
        screen_manager.terminal.draw(move |f| {
            render(f, screens, popups, test_tick);
        })?;

        // test the current things got what they needed
        assert_eq!(popup_top_rx.try_recv().ok(), Some(test_tick));
        assert_eq!(popup_bottom_rx.try_recv().ok(), Some(deltatime_tick));

        assert_eq!(screen_top_rx.try_recv().ok(), Some(deltatime_tick));
        assert_eq!(screen_bottom_rx.try_recv().ok(), None);

        Ok(())
    }

    /// Test keyboard input handled correctly with just screens
    #[test]
    #[timeout(2000)]
    fn test_main_loop_screen() {
        let mut screen_manager = ScreenManager::debug_new(vec!(
            crossterm_key('k')
        )).unwrap();

        let screen = TestScreen::new();

        let screen_rx = screen.rx_rc.clone();

        screen_manager.push_screen(Box::new(screen));

        screen_manager.start_main_loop().unwrap();

        // After 10 render, it should have quit
        let ticks = get_ticks_from_rx(&screen_rx);

        let an_input = ticks.iter().find(|&tick| {
            tick_has_command(*tick, Command::Up)
        });

        assert_ne!(an_input, None);
    }

    /// Test keyboard input handled correctly with just popups
    #[test]
    #[timeout(2000)]
    fn test_main_loop_popup() {
        let mut screen_manager = ScreenManager::debug_new(vec!(
            crossterm_key('k')
        )).unwrap();

        let popup = TestPopup::new();

        let popup_rx = popup.rx_rc.clone();

        screen_manager.push_popup(Box::new(popup));

        screen_manager.start_main_loop().unwrap();

        // After 10 render, it should have quit
        let ticks = get_ticks_from_rx(&popup_rx);

        let an_input = ticks.iter().find(|&tick| {
            tick_has_command(*tick, Command::Up)
        });

        assert_ne!(an_input, None);
    }

    /// Test keyboard input handled corrrectly with both screens and popups
    #[test]
    #[timeout(2000)]
    fn test_main_loop_screens_and_popups() {
        let mut screen_manager = ScreenManager::debug_new(vec!(
            crossterm_key('k')
        )).unwrap();

        let screen = TestScreen::new();
        let popup = TestPopup::new();

        let screen_rx = screen.rx_rc.clone();
        let popup_rx = popup.rx_rc.clone();

        screen_manager.push_screen(Box::new(screen));
        screen_manager.push_popup(Box::new(popup));

        screen_manager.start_main_loop().unwrap();

        // After 10 render, it should have quit
        let screen_ticks = get_ticks_from_rx(&screen_rx);
        let popup_ticks = get_ticks_from_rx(&popup_rx);

        let popup_input = popup_ticks.iter().find(|&tick| {
            tick_has_command(*tick, Command::Up)
        });
        let screen_input = screen_ticks.iter().find(|&tick| {
            tick_has_command(*tick, Command::Up)
        });

        assert_ne!(popup_input, None); // Popup should of received input
        assert_eq!(screen_input, None); // and not the screen
    }
}
