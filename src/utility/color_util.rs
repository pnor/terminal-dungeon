use cursive::theme::Color;
use cursive::theme::BaseColor;

/// Returns `color` as an `Color::Rgb` enum.
/// `Color::TerminalDefault` is converted to `Color::Rgb(255, 255, 255)`
pub fn to_rgb(color: Color) -> Color {
    match color {
        Color::Rgb(r, g, b) => Color::Rgb(r, g, b),
        Color::Light(base_color) => light_base_color_rgb(base_color),
        Color::Dark(base_color) => dark_base_color_rgb(base_color),
        Color::RgbLowRes(r, g, b) => Color::Rgb(r, g, b),
        Color::TerminalDefault => Color::Rgb(255, 255, 255)
    }
}

fn light_base_color_rgb(base_color: BaseColor) -> Color {
    match base_color {
        BaseColor::Black => Color::Rgb(0, 0, 0),
        BaseColor::Red => Color::Rgb(201, 27, 0),
        BaseColor::Green => Color::Rgb(0, 194, 0),
        BaseColor::Yellow => Color::Rgb(199, 196, 0),
        BaseColor::Blue => Color::Rgb(2, 37, 199),
        BaseColor::Magenta => Color::Rgb(201, 48, 199),
        BaseColor::Cyan => Color::Rgb(0, 197, 199),
        BaseColor::White => Color::Rgb(199, 199, 199)
    }
}

fn dark_base_color_rgb(base_color: BaseColor) -> Color {
    match base_color {
        BaseColor::Black => Color::Rgb(103, 103, 103),
        BaseColor::Red => Color::Rgb(255, 109, 103),
        BaseColor::Green => Color::Rgb(95, 249, 103),
        BaseColor::Yellow => Color::Rgb(254, 251, 103),
        BaseColor::Blue => Color::Rgb(104, 113, 255),
        BaseColor::Magenta => Color::Rgb(255, 118, 255),
        BaseColor::Cyan => Color::Rgb(95, 253, 255),
        BaseColor::White => Color::Rgb(254, 255, 255)
    }
}
