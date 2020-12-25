mod world;

use cursive::Cursive;
use cursive::theme::*;
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::*;

use world::map;

fn main() {
    println!("{}", draw_room());
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
