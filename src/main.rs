mod world;
mod entities;
mod utility;
mod systems;
mod game;
mod views;

use std::error::Error;
use specs::Dispatcher;
use specs::{DispatcherBuilder, World, WorldExt};

use tui::style::Color;

use crate::map::Map;
use world::map;

use entities::{component, factory};
use component::*;
use systems::*;
use utility::text_canvas::{TextCanvas, CanvasSymbol};
use game::Command;
use game::InputManager;

use std::rc::Rc;

use std::thread;
use std::time::Duration;

use std::io::{self, Write, Stdout};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph};
use tui::layout::{Rect, Layout, Constraint, Direction};

use crossterm::terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::EnableMouseCapture;

extern crate nalgebra;
use nalgebra::Vector2;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut terminal = setup_ui()?;

    let (mut world, mut dispatcher) = init_game();
    register_components(&mut world);
    add_resources(&mut world);
    make_player(&mut world);

    let input_manager = InputManager::new(Duration::from_millis(16));

    for _ in 1..400 {
        let command = input_manager.tick()?;
        {
            let mut command_res = world.write_resource::<Command>();

            match command {
                Command::Tick(delta) => {
                    *command_res = Command::None;
                },
                command => {
                    *command_res = command;
                }
            }
        }

        run_world(&mut world, &mut dispatcher);
        draw_ui(&mut world, &mut terminal);

        // thread::sleep(Duration::from_millis(50));
    }

    disable_raw_mode()?;

    Ok(())
}

fn setup_ui() -> Result<Terminal<CrosstermBackend<Stdout>>, io::Error> {
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear();

    Ok(terminal)
}

fn draw_ui(world: &mut World, terminal: &mut Terminal<CrosstermBackend<Stdout>>) {
    let canvas = world.read_resource::<TextCanvas>();

    terminal.draw(move |f| {
        let map_text = (*canvas).as_styled_text();
        let map_display = Paragraph::new(map_text)
            .block(
                Block::default()
                    .title("map!")
                    .borders(Borders::ALL)
            );

        let (width, height) = (*canvas).dimensions();
        let rec = Rect::new(0, 0, width as u16, height as u16);

        f.render_widget(map_display, rec);
    });
}

fn clear_screen(stdout: &mut Stdout) {
    write!(stdout, "{}", Clear(ClearType::All));
}

fn init_game<'a>() -> (World, Dispatcher<'a, 'a>) {
    let mut world = World::new();

    register_components(&mut world);
    add_resources(&mut world);
    make_player(&mut world);
    let mut dispatch = setup_dispatch();

    (world, dispatch)
}

fn register_components(world: &mut World) {
    world.register::<Appearance>();
    world.register::<Camera>();
    world.register::<CommandResponse>();
    world.register::<Follow>();
    world.register::<Position>();
}

fn add_resources(world: &mut World) {
    let map = initialize_map();

    let canvas = TextCanvas::for_map(&map);
    world.insert(canvas);

    world.insert(map);

    world.insert(Command::None);
}

fn initialize_map() -> Map {
    map::test_room()
}

fn make_player(world: &mut World) {
    let player = factory::make_player(world);
    let _ = factory::make_camera(player, world);
}

fn setup_dispatch<'a>() -> Dispatcher<'a, 'a> {
    DispatcherBuilder::new()
        .with(CommandSystem, "Command", &[])
        .with(FollowSystem, "Follow", &["Command"])
        .with_thread_local(TextRenderSystem)
        .build()
}

fn update_command(world: &mut World, command: Command) {
    let mut resource = world.write_resource::<Command>();
    *resource = command;
}

fn run_world(world: &mut World, dispatcher: &mut Dispatcher) {
    dispatcher.dispatch(&world);
    world.maintain();
}

fn setup_input() {}
