mod world;
mod entities;

use cursive::Cursive;
use cursive::theme::*;
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::*;

use specs::{Builder, Component, DispatcherBuilder,
            VecStorage, World, WorldExt};

use world::map;

use entities::{entity, component, text_render_system, factory};
use component::{Position, Appearance};
use text_render_system::TextRenderSystem;

fn main() {
    demo_ecs();
}

fn demo_ecs() {
    let mut world = World::new();

    // Register Components
    world.register::<Position>();
    world.register::<Appearance>();

    // Add resources
    let mut map = map::test_room();
    world.insert(map);

    // Make a player
    let player = factory::make_player(&mut world);

    // Setup Dispatch
    let mut dispatcher = DispatcherBuilder::new()
        .with(TextRenderSystem, "text_render_system", &[])
        .build();

    // Run world once
    dispatcher.dispatch(&mut world);
    world.maintain();
}

fn draw_room() -> String {
    let room = map::test_room();
    room.as_string()
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
