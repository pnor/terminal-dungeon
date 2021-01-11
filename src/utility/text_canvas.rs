use cursive::theme::{Color, Style, Effect};
use cursive::utils::markup::StyledString;
use crate::utility::{color_util, conversions};

extern crate nalgebra as na;
use na::{clamp, Vector2};

/// Canvas that is used to generate a string to represent the portion of the map on camera
pub struct TextCanvas {
    symbols: Vec<Vec<CanvasSymbol>>
}

pub fn create_canvas(width: usize, height: usize) -> TextCanvas {
    let symbols = vec![vec![CanvasSymbol::default(); height]; width];
    TextCanvas { symbols }
}

impl Default for TextCanvas {

    fn default() -> TextCanvas {
        TextCanvas { symbols: vec![vec![CanvasSymbol::default(); 0]; 0] }
    }

}

impl TextCanvas {

    pub fn as_styled_string(&self) -> StyledString {
        let mut string = StyledString::default();

        for i in 0..(self.symbols.len() - 1) {
            for j in 0..(self.symbols[i].len() - 1) {
                let symbol = &self.symbols[i][j];
                string.append(symbol.styled_string());
            }
            string.append(StyledString::plain("\n"));
        }

        string
    }

    /// Reurns `(widht, height)` of the canvas
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

    // Mutating the board
    pub fn set_symbol(&mut self, vec2: Vector2<usize>, symbol: CanvasSymbol) {
        self.symbols[vec2[0]][vec2[1]] = symbol;
    }

    pub fn set_character(&mut self, vec2: Vector2<usize>, character: char) {
        self.symbols[vec2[0]][vec2[1]].character = character;
    }

    pub fn set_face(&mut self, vec2: Vector2<usize>, face: Effect) {
        self.symbols[vec2[0]][vec2[1]].face = face;
    }

    pub fn set_color(&mut self, vec2: Vector2<usize>, color: Color) {
        self.symbols[vec2[0]][vec2[1]].color = color;
    }

    pub fn apply_color(&mut self, vec2: Vector2<usize>, color: Color, alpha: f64) {
        let base_color = color_util::to_rgb(self.symbols[vec2[0]][vec2[1]].color);
        let applied_color = color_util::to_rgb(color);

        if let (Color::Rgb(r, g, b), Color::Rgb(r_a, g_a, b_a)) = (base_color, applied_color) {
            let (r, g, b) = (r as f64, g as f64, b as f64);
            let (r_a, g_a, b_a) = (r_a as f64, g_a as f64, b_a as f64);

            let new_r = clamp(r + ((r_a - r) * alpha), 0.0, 255.0) as u8;
            let new_g = clamp(g + ((g_a - g) * alpha), 0.0, 255.0) as u8;
            let new_b = clamp(b + ((b_a - b) * alpha), 0.0, 255.0) as u8;

            self.symbols[vec2[0]][vec2[1]].color = Color::Rgb(new_r, new_g, new_b);
        }
    }

}

#[derive(Clone)]
pub struct CanvasSymbol {
    pub character: char,
    pub color: Color,
    pub face: Effect
}

impl CanvasSymbol {

    fn styled_string(&self) -> StyledString {
        StyledString::styled(
            self.character.to_string(),
            Style::from(self.color).combine(self.face)
        )
    }

}

impl Default for CanvasSymbol {

    fn default() -> CanvasSymbol {
        CanvasSymbol {
            character: ' ',
            color: Color::Rgb(0, 0, 0),
            face: Effect::Simple
        }
    }

}

