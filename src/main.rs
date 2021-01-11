mod world;
mod entities;
mod utility;
mod systems;
mod game;

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
    setup_word();
}

fn setup_word() {
    let mut world = World::new();
    world.insert(3);

    // Register Components
    world.register::<Position>();
    world.register::<Appearance>();
    world.register::<Camera>();
    world.register::<CommandResponse>();

    // Add resources
    let map = map::test_room();
    world.insert(map);
    let canvas = create_canvas(20, 20);
    world.insert(canvas);

    world.insert(Command::None);

    // Make a player
    let player = factory::make_player(&mut world);
    let _ = factory::make_camera(player, &mut world);

    // Setup Dispatch
    let mut dispatcher = DispatcherBuilder::new()
        .with(CameraSystem, "Camera", &[])
        .with(CommandSystem, "Command", &[])
        .with_thread_local(TextRenderSystem)
        .build();

    // Run world once
    dispatcher.dispatch(&mut world);
    world.maintain();

    // update display
    let mutated_canvas = world.read_resource::<TextCanvas>();
    draw_room(&mutated_canvas);
}

fn draw_room(canvas: &TextCanvas) {
    let mut siv = cursive::default();

    let mut theme = siv.current_theme().clone();
    theme.shadow = true;
    siv.set_theme(theme);

    let (canvas_width, canvas_height) = canvas.dimensions();

    let dont_care_color = Color::Rgb(0, 0, 0); // color i don't think will be seen

    let game_map_view =
        Panel::new(
            TextView::new(canvas.as_styled_string())
                .fixed_size((canvas_width - 1, canvas_height - 1))
            );

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

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}
