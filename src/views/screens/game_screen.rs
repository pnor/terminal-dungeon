use super::Frame;
use crate::views::ScreenManager;
use crate::utility::conversions;
use tui::layout::Rect;
use tui::widgets::Block;
use tui::widgets::Paragraph;
use tui::widgets::Borders;

use crate::game::Command;
use crate::game::GameTick;
use crate::views::Screen;
use crate::views::screen_manager::BoxedCallback;

use std::collections::VecDeque;

use specs::Dispatcher;
use specs::{DispatcherBuilder, World, WorldExt};

use crate::entities::factory;
use crate::systems::*;
use crate::entities::component::*;
use crate::world::map::{self, Map};
use crate::utility::text_canvas::TextCanvas;

/// Primary screen where game is played
/// Handles the boiler palte for setting up a Specs World
pub struct GameScreen<'a> {
    world: World,
    dispatcher: Dispatcher<'a, 'a>,
    callbacks: VecDeque<BoxedCallback>,
}

impl Screen for GameScreen<'_> {

    fn new() -> Self {
        let (mut world, dispatcher) = init_game();
        register_components(&mut world);
        add_resources(&mut world);
        make_player(&mut world);

        factory::make_dummy(&mut world);

        GameScreen {
            world,
            dispatcher,
            callbacks: VecDeque::new(),
        }
    }

    fn render(&mut self, mut frame: &mut Frame, tick: GameTick) {
        update_world_tick(&mut self.world, tick);

        if check_time_to_quit(tick) {
            self.add_screen_manager_callback(Box::new(|s: &mut ScreenManager| {
                s.should_quit = true;
            }));
        }

        run_world(&mut self.world, &mut self.dispatcher);
        draw_ui(&mut self.world, &mut frame);
    }

    fn tear_down(&mut self) {
    }

    fn add_screen_manager_callback(&mut self, callback: BoxedCallback) {
        self.callbacks.push_front(callback)
    }

    fn get_screen_manager_callbacks(&mut self) -> VecDeque<BoxedCallback> {
        self.callbacks.drain(0..).collect()
    }

}

fn check_time_to_quit(tick: GameTick) -> bool {
    match tick {
        GameTick::Command(_, Command::Quit) => true,
        _ => false
    }
}

/// Handling specs ECS
fn init_game<'a>() -> (World, Dispatcher<'a, 'a>) {
    let mut world = World::new();

    register_components(&mut world);
    add_resources(&mut world);
    make_player(&mut world);
    let dispatch = setup_dispatch();

    (world, dispatch)
}


fn register_components(world: &mut World) {
    world.register::<Appearance>();
    world.register::<Camera>();
    world.register::<CommandResponse>();
    world.register::<Follow>();
    world.register::<Position>();
    world.register::<Collision>();
}

fn add_resources(world: &mut World) {
    let map = initialize_map();

    let canvas = create_canvas(&map);
    world.insert(canvas);

    world.insert(map);

    world.insert(GameTick::default());
}

fn create_canvas(map: &Map) -> TextCanvas {
    let (terminal_width, terminal_height) = match crossterm::terminal::size() {
        Ok((width, height)) => (conversions::u16_to_usize(width), conversions::u16_to_usize(height)),
        _ => (0, 0)
    };
    let (map_width, map_height) = map.dimensions();

    let canvas_width = std::cmp::min(terminal_width, map_width);
    let canvas_height = std::cmp::min(terminal_height, map_height);

    TextCanvas::with_size(canvas_width, canvas_height)
}

fn initialize_map() -> Map {
    map::test_big_room()
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

fn update_world_tick(world: &mut World, tick: GameTick) {
    let mut resource = world.write_resource::<GameTick>();
    *resource = tick;
}

fn run_world(world: &mut World, dispatcher: &mut Dispatcher) {
    dispatcher.dispatch(&world);
    world.maintain();
}

fn draw_ui(world: &mut World, frame: &mut Frame) {
    let canvas = world.read_resource::<TextCanvas>();

    let map_text = (*canvas).as_styled_text();
    let map_display = Paragraph::new(map_text)
        .block(
            Block::default()
                .title("map!")
                .borders(Borders::ALL)
        );

    let (width, height) = (*canvas).dimensions();
    let rec = Rect::new(0, 0, width as u16, height as u16);

    frame.render_widget(map_display, rec);
}
