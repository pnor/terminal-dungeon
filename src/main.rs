mod world;
mod entities;
mod utility;
mod systems;
mod game;

use std::rc::Rc;
use std::cell::RefCell;

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
use systems::{CameraSystem, TextRenderSystem, CommandSystem};
use utility::text_canvas::{TextCanvas, create_canvas};
use game::Command;

fn main() {
    let mut world = World::new();

    register_components(&mut world);
    add_resources(&mut world);
    make_player(&mut world);
    let mut dispatch = setup_dispatch();
    let mut siv = setup_cursive();
    setup_display(&mut siv, &world);
    setup_callbacks(
        &mut siv,
        Rc::new(RefCell::new(world)),
        Rc::new(RefCell::new(dispatch))
    );

    siv.run();
}

fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Appearance>();
    world.register::<Camera>();
    world.register::<CommandResponse>();
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
        .with(CameraSystem, "Camera", &[])
        .with(CommandSystem, "Command", &[])
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

fn setup_display(siv: &mut Cursive, world: &World) {
    let canvas = world.read_resource::<TextCanvas>();
    let (canvas_width, canvas_height) = canvas.dimensions();

    let dont_care_color = Color::Rgb(0, 0, 0); // color i don't think will be seen

    let map_text_view = TextView::new(canvas.as_styled_string())
                .with_name("canvas")
                .fixed_size((canvas_width - 1, canvas_height - 1));

    let game_map_view = Panel::new(map_text_view);

    let main_view =
        LinearLayout::vertical()
        .child(Canvas::new("").fixed_size((1, 2)))
        .child(
            ShadowView::new(
                Panel::new(
                    Layer::with_color(
                        TextView::new("hello (:")
                    , ColorStyle::new(dont_care_color.clone(), Color::Rgb(206, 206, 206)))
                )
            )
        )
        .child(Canvas::new("").fixed_size((1, 2)))
        .child(
            ShadowView::new(
                game_map_view
            ).left_padding(true)
        )
        .child(Canvas::new("").fixed_size((1, 2)));

    let side_padding =
        LinearLayout::horizontal()
        .child(Canvas::new("").fixed_size((5, 1)))
        .child(main_view)
        .child(Canvas::new("").fixed_size((5, 1)));

    siv.add_layer(
        Layer::with_color(
            side_padding
            , ColorStyle::new(dont_care_color.clone(), Color::Rgb(100, 100, 100))
        )
    );
}

fn setup_callbacks<'a>(
    siv: &mut Cursive,
    world_pointer: Rc<RefCell<World>>,
    dispatch_pointer: Rc<RefCell<Dispatcher<'static, 'static>>>
) {
    let world_pointer = world_pointer.clone();
    let dispatch_pointer = dispatch_pointer.clone();

    siv.add_global_callback('j', move |s| {
        let world_refcell = &*world_pointer;
        let mut world = world_refcell.borrow_mut();

        let dispatch_refcell = &*dispatch_pointer;
        let mut dispatcher = dispatch_refcell.borrow_mut();

        {
            let mut command = world.write_resource::<Command>();
            *command = Command::Down;
        }

        dispatcher.dispatch(&world);
        world.maintain();

        update_display(s, &mut world);
    });

    siv.add_global_callback('q', |s| s.quit());
}

fn update_display(siv: &mut Cursive, world: &mut World) {
    let canvas = world.read_resource::<TextCanvas>();

    match siv.find_name::<TextView>("canvas") {
        Some(mut text_view) => {
            text_view.set_content(canvas.as_styled_string())
        },
        None => ()
    }
}
