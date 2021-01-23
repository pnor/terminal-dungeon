mod world;
mod entities;
mod utility;
mod systems;
mod game;
mod views;

use crate::views::MapView;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;

use crate::map::Map;
use cursive::Cursive;
use specs::Dispatcher;
use cursive::theme::*;
use cursive::traits::*;
use cursive::views::*;

use specs::{DispatcherBuilder, World, WorldExt};

use world::map;

use entities::{component, factory};
use component::*;
use systems::*;
use utility::text_canvas::TextCanvas;
use game::Command;


fn main() {
    test();
    // let mut world = World::new();

    // register_components(&mut world);
    // add_resources(&mut world);
    // make_player(&mut world);
    // let mut dispatch = setup_dispatch();
    // let mut siv = setup_cursive();
    // setup_display(&mut siv, &world);
    // setup_callbacks(
    //     &mut siv,
    //     Rc::new(RefCell::new(world)),
    //     Rc::new(RefCell::new(dispatch))
    // );

    // siv.run();
}

fn test() {
    let mut siv = cursive::default();

    let (sx, rx): (Sender<TextCanvas>, Receiver<TextCanvas>) = channel();

    let map = map::test_room();

    let _dont_care_color = Color::Rgb(0, 0, 0); // color i don't think will be seen

    let map_view = MapView::new(&map, rx);

    siv.add_layer(
        map_view
    );

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('a', move |_| {
        let canvas = TextCanvas::for_map(&map);
        sx.send(canvas).unwrap();
    });

    siv.run();
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

    world.insert(map);
    world.insert(canvas);

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

fn setup_cursive() -> Cursive {
    let mut siv = cursive::default();

    let mut theme = siv.current_theme().clone();
    theme.shadow = true;
    siv.set_theme(theme);

    siv
}

// fn setup_display(siv: &mut Cursive, world: &World) {
//     let canvas = world.read_resource::<TextCanvas>();
//     let (canvas_width, canvas_height) = canvas.dimensions();
//
//     let dont_care_color = Color::Rgb(0, 0, 0); // color i don't think will be seen
//
//     let map_text_view = TextView::new(canvas.as_styled_string())
//                 .with_name("canvas")
//                 .fixed_size((canvas_width - 1, canvas_height - 1));
//
//     let game_map_view = Panel::new(map_text_view);
//
//     let main_view =
//         LinearLayout::vertical()
//         .child(Canvas::new("").fixed_size((1, 2)))
//         .child(
//             ShadowView::new(
//                 Panel::new(
//                     Layer::with_color(
//                         TextView::new("hello (:")
//                     , ColorStyle::new(dont_care_color.clone(), Color::Rgb(206, 206, 206)))
//                 )
//             )
//         )
//         .child(Canvas::new("").fixed_size((1, 2)))
//         .child(
//             ShadowView::new(
//                 game_map_view
//             ).left_padding(true)
//         )
//         .child(Canvas::new("").fixed_size((1, 2)));
//
//     let side_padding =
//         LinearLayout::horizontal()
//         .child(Canvas::new("").fixed_size((5, 1)))
//         .child(main_view)
//         .child(Canvas::new("").fixed_size((5, 1)));
//
//     siv.add_layer(
//         Layer::with_color(
//             side_padding
//             , ColorStyle::new(dont_care_color.clone(), Color::Rgb(100, 100, 100))
//         )
//     );
// }

fn setup_callbacks(
    siv: &mut Cursive,
    world_pointer: Rc<RefCell<World>>,
    dispatch_pointer: Rc<RefCell<Dispatcher<'static, 'static>>>
) {
    let command_keybinds = vec![
        ('j', Command::Down),
        ('k', Command::Up),
        ('h', Command::Left),
        ('l', Command::Right)
    ];


    for (input, command) in command_keybinds {
        let world_pointer = world_pointer.clone();
        let dispatch_pointer = dispatch_pointer.clone();
        siv.add_global_callback(input, move |s| {
            let world_refcell = &*world_pointer;
            let mut world = world_refcell.borrow_mut();

            let dispatch_refcell = &*dispatch_pointer;
            let mut dispatcher = dispatch_refcell.borrow_mut();

            update_command(&mut world, command);
            run_world(&mut world, &mut dispatcher);
        });
    }

    siv.add_global_callback('q', |s| s.quit());
}

fn update_command(world: &mut World, command: Command) {
    let mut resource = world.write_resource::<Command>();
    *resource = command;
}

fn run_world(world: &mut World, dispatcher: &mut Dispatcher) {
    dispatcher.dispatch(&world);
    world.maintain();
}
