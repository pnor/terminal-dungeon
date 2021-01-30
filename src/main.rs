mod world;
mod entities;
mod utility;
mod systems;
mod game;
mod views;

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

use std::rc::Rc;

use std::thread;
use std::time::Duration;

use std::io::{self, Write};
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph};
use tui::layout::{Layout, Constraint, Direction};

use crossterm::terminal::{Clear, ClearType};

extern crate nalgebra;
use nalgebra::Vector2;

fn main() {
    test().unwrap();
}

fn test() -> Result<(), io::Error> {
    let mut stdout = io::stdout();

    // clear the screen
    write!(stdout, "{}", Clear(ClearType::All))?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let map = map::test_room();
    let mut canvas = TextCanvas::for_map(&map);
    let character = CanvasSymbol {
        character: '!',
        foreground: Color::Red,
        background: Color::Blue,
        modifiers: vec!(),
    };

    canvas.set_symbol(Vector2::new(3, 3), character);

    let canvas_ptr = Rc::new(canvas);

    for _ in 0..10 {
        let canvas_ptr_clone = Rc::clone(&canvas_ptr);

        terminal.draw(move |f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10)
                    ].as_ref())
                .split(f.size());

            let block = Block::default()
                .title("Block")
                .borders(Borders::ALL);

            f.render_widget(block, chunks[0]);

            let map_text = (*canvas_ptr_clone).as_styled_text();
            let map_display = Paragraph::new(map_text)
                .block(
                    Block::default()
                        .title("map!")
                        .borders(Borders::ALL)
                );

            f.render_widget(map_display, chunks[1]);
        })?;

        thread::sleep(Duration::from_millis(1000));
    }

    Ok(())
}

fn init_game() {
    let mut world = World::new();

    register_components(&mut world);
    add_resources(&mut world);
    make_player(&mut world);
    let mut dispatch = setup_dispatch();
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
