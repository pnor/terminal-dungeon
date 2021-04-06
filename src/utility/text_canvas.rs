use tui::text::{Span, Spans, Text};
use tui::style::{Color, Style, Modifier};
use crate::world::map::Map;
use crate::utility::{color_util, conversions};

extern crate nalgebra as na;
use na::{clamp, Vector2};

/// Canvas that is used to generate a string to represent the portion of the map on camera
pub struct TextCanvas {
    symbols: Vec<Vec<CanvasSymbol>>
}

impl Default for TextCanvas {

    fn default() -> TextCanvas {
        TextCanvas { symbols: vec![vec![CanvasSymbol::default(); 0]; 0] }
    }

}

impl TextCanvas {

    /// Returns `TextCanvas` with default symbols of size `width` x `height`
    pub fn with_size(width: usize, height: usize) -> Self {
        TextCanvas { symbols: vec![vec![CanvasSymbol::default(); height]; width] }
    }

    /// Returns `TextCanvas` with default symbols of the same dimensions as the map
    pub fn for_map(map: &Map) -> Self {
        let (map_width, map_height) = map.dimensions();
        TextCanvas { symbols: vec![vec![CanvasSymbol::default(); map_height]; map_width] }
    }

    /// Returns representation of the canvas as a multiline `Text`
    pub fn as_styled_text(&self) -> Text {
        let mut map_lines: Vec<Spans> = vec!();
        let (width, height) = self.dimensions();

        for i in 0..width {
            let mut line: Vec<Span> = vec!();

            for j in 0..height {
                let symbol = &self.symbols[i][j];
                line.push(symbol.span());
            }
            line.push(Span::raw("\n"));

            map_lines.push(Spans::from(line));
        }

        Text::from(map_lines)
    }

    /// Returns `(width, height)` of the canvas
    pub fn dimensions(&self) -> (usize, usize) {
        if self.symbols.len() > 0 {
            (self.symbols.len(), self.symbols[0].len())
        } else {
            (0, 0)
        }
    }

    /// Returns `true` if (x, y) is in bounds of `self.symbols`
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        if self.symbols.len() > 0 {
            let width = conversions::as_i32(self.symbols.len());
            let height = conversions::as_i32(self.symbols[0].len());

            (x >= 0 && x < width) && (y >= 0 && y < height)
        } else {
            false
        }
    }

    /// Sets the `CanvasSymbol` at the location specified by `vec2`
    pub fn set_symbol(&mut self, vec2: Vector2<usize>, symbol: CanvasSymbol) {
        self.symbols[vec2[0]][vec2[1]] = symbol;
    }

    /// Changes the character at the location specified by `vec2`
    pub fn set_character(&mut self, vec2: Vector2<usize>, character: char) {
        self.symbols[vec2[0]][vec2[1]].character = character;
    }

    /// Adds a modifier to the location specified by `vec2`
    pub fn add_modifier(&mut self, vec2: Vector2<usize>, modifier: Modifier) {
        self.symbols[vec2[0]][vec2[1]].modifiers.push(modifier)
    }

    /// Removes all modifiers to the location specified by `vec2`
    pub fn clear_modifiers(&mut self, vec2: Vector2<usize>) {
        self.symbols[vec2[0]][vec2[1]].modifiers = vec!();
    }

    /// Changes the foreground color of the symbol at the location specified by `vec2`
    pub fn set_fg_color(&mut self, vec2: Vector2<usize>, color: Color) {
        self.symbols[vec2[0]][vec2[1]].foreground = color;
    }

    /// Changes the background color of the symbol at the location specified by `vec2`
    pub fn set_bg_color(&mut self, vec2: Vector2<usize>, color: Color) {
        self.symbols[vec2[0]][vec2[1]].background = color;
    }

    /// Alters the foreground color by applying `color` with an alpha value
    pub fn apply_fg_color(&mut self, vec2: Vector2<usize>, color: Color, alpha: f64) {
        let base_color = self.symbols[vec2[0]][vec2[1]].foreground;
        self.symbols[vec2[0]][vec2[1]].foreground = Self::apply_color(base_color, color, alpha);
    }

    /// Alters the background color by applying `color` with an alpha value
    pub fn apply_bg_color(&mut self, vec2: Vector2<usize>, color: Color, alpha: f64) {
        let base_color = self.symbols[vec2[0]][vec2[1]].background;
        self.symbols[vec2[0]][vec2[1]].background = Self::apply_color(base_color, color, alpha);
    }

    fn apply_color(base_color: Color, color: Color, alpha: f64) -> Color {
        let base_color = color_util::color_to_rgb(base_color);
        let applied_color = color_util::color_to_rgb(color);

        if let (Color::Rgb(r, g, b), Color::Rgb(r_a, g_a, b_a)) = (base_color, applied_color) {
            let (r, g, b) = (r as f64, g as f64, b as f64);
            let (r_a, g_a, b_a) = (r_a as f64, g_a as f64, b_a as f64);

            let new_r = clamp(r + ((r_a - r) * alpha), 0.0, 255.0) as u8;
            let new_g = clamp(g + ((g_a - g) * alpha), 0.0, 255.0) as u8;
            let new_b = clamp(b + ((b_a - b) * alpha), 0.0, 255.0) as u8;

            return Color::Rgb(new_r, new_g, new_b);
        } else {
            return base_color;
        }
    }

}

#[derive(Clone)]
pub struct CanvasSymbol {
    pub character: char,
    pub foreground: Color,
    pub background: Color,
    pub modifiers: Vec<Modifier>
}

impl CanvasSymbol {

    fn span(&self) -> Span {
        let style = Style::default().fg(self.foreground).bg(self.background);
        for modifier in &self.modifiers {
            style.add_modifier(*modifier);
        }

        Span::styled(self.character.to_string(), style)
    }

}

impl Default for CanvasSymbol {

    fn default() -> CanvasSymbol {
        CanvasSymbol {
            character: ' ',
            foreground: Color::Rgb(0, 0, 0),
            background: Color::Rgb(0, 0, 0),
            modifiers: vec![]
        }
    }

}

