use tui::style::Color;

/// Returns `color` as an `Color::Rgb` enum.
pub fn color_to_rgb(color: Color) -> Color {
    match color {
        // Dark colors
        Color::Black => Color::Rgb(0, 0, 0),
        Color::Red => Color::Rgb(201, 27, 0),
        Color::Green => Color::Rgb(0, 194, 0),
        Color::Yellow => Color::Rgb(199, 196, 0),
        Color::Blue => Color::Rgb(2, 37, 199),
        Color::Magenta => Color::Rgb(201, 48, 199),
        Color::Cyan => Color::Rgb(0, 197, 199),
        Color::White => Color::Rgb(254, 255, 255),
        // Light colors
        Color::LightRed => Color::Rgb(255, 109, 103),
        Color::LightGreen => Color::Rgb(95, 249, 103),
        Color::LightYellow => Color::Rgb(254, 251, 103),
        Color::LightBlue => Color::Rgb(104, 113, 255),
        Color::LightMagenta => Color::Rgb(255, 118, 255),
        Color::LightCyan => Color::Rgb(95, 253, 255),
        // Grays
        Color::Gray => todo!(),
        Color::DarkGray => todo!(),
        // Non color constants
        Color::Rgb(r, g, b) => Color::Rgb(r, g, b),
        Color::Reset => todo!(),
        Color::Indexed(_) => {
                todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_colors() {
        let black = Color::Black;
        assert_eq!(color_to_rgb(black), Color::Rgb(0, 0, 0));

        let red = Color::Red;
        assert_eq!(color_to_rgb(red), Color::Rgb(201, 27, 0));

        let green = Color::Green;
        assert_eq!(color_to_rgb(green), Color::Rgb(0, 194, 0));

        let yellow = Color::Yellow;
        assert_eq!(color_to_rgb(yellow), Color::Rgb(199, 196, 0));

        let blue = Color::Blue;
        assert_eq!(color_to_rgb(blue), Color::Rgb(2, 37, 199));

        let magenta = Color::Magenta;
        assert_eq!(color_to_rgb(magenta), Color::Rgb(201, 48, 199));

        let cyan = Color::Cyan;
        assert_eq!(color_to_rgb(cyan), Color::Rgb(0, 197, 199));

        let white = Color::White;
        assert_eq!(color_to_rgb(white), Color::Rgb(254, 255, 255));
    }

}
