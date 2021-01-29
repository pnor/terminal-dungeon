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
use utility::text_canvas::TextCanvas;
use game::Command;

use std::thread;
use std::time::Duration;

fn main() {
    test();
}

fn test() {
    let map = map::test_room();

    let _dont_care_color = Color::Rgb(0, 0, 0); // color i don't think will be seen

    // let map_view = MapView::new(&map, rx);
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
