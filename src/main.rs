mod world;
mod entities;

use std::rc::Rc;

use cursive::Cursive;
use cursive::theme::*;
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::*;

use specs::{Builder, Component, DispatcherBuilder,
            VecStorage, World, WorldExt};

use world::map;

use entities::{entity, component, text_render_system, factory, text_canvas};
use component::{Position, Appearance};
use text_canvas::TextCanvas;
use text_render_system::TextRenderSystem;

fn main() {
    setup_word();
}

fn setup_word() {
    let mut world = World::new();
    world.insert(3);

    // Register Components
    world.register::<Position>();
    world.register::<Appearance>();

    // Add resources
    let map = map::test_room();
    world.insert(map);
    let canvas = text_canvas::create_canvas(20, 20);
    world.insert(canvas);

    // Make a player
    let _ = factory::make_player(&mut world);

    // Setup Dispatch
    let mut dispatcher = DispatcherBuilder::new()
        .with(TextRenderSystem, "text_render_system", &[])
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

    let (canvas_width, canvas_height) = canvas.dimensions();

    let game_map_view =
        LinearLayout::vertical()
        .child(TextView::new(canvas.as_styled_string()))
        .fixed_size((canvas_width, canvas_height));

    siv.add_layer(
        Layer::with_color(
            LinearLayout::vertical()
                .child(game_map_view)
                .fixed_size((30, 30))
            , ColorStyle::new(Color::Rgb(200, 200, 200), Color::Rgb(100, 100, 100))
        )
    );


    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

fn demo() {
    let mut siv = cursive::default();

    let mut styled = StyledString::plain("I'm ");
    styled.append(StyledString::styled("floating", Color::Rgb(255, 100, 100)));
    styled.append(StyledString::styled(
            "!",
            Style::from(Color::Rgb(50, 50, 255)).combine(Effect::Bold)));

    let mainBody =
        LinearLayout::vertical()
        .child(TextView::new(styled))
        .fixed_size((10, 10))
        ;

    let above_view =
        LinearLayout::vertical()
        .child(Canvas::new("").fixed_size((1, 1)))
        .child(
            LinearLayout::horizontal()
            .child(Canvas::new("").fixed_size((1, 1)))
            .child(ShadowView::new(
                    mainBody
                    ))
            .child(Canvas::new("").fixed_size((1, 1)))
            )
        .child(Canvas::new("").fixed_size((1, 1)));

    siv.add_layer(
        Layer::with_color(
        LinearLayout::vertical()
        .child(above_view)
        .fixed_size((40, 40))
        ,ColorStyle::new(Color::Rgb(200, 200, 200), Color::Rgb(100, 100, 100)))
    );

    siv.add_global_callback('q', |s| s.quit());

    siv.run();

}
