use cursive::theme::{Color, Style, Effect};
use cursive::utils::markup::StyledString;

#[derive(Clone)]
struct CanvasSymbol {
    character: char,
    color: Color,
    face: Effect
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

pub struct TextCanvas {
    symbols: Vec<Vec<CanvasSymbol>>
}

pub fn create_canvas(width: usize, height: usize) -> TextCanvas {
    let mut symbols = vec![vec![CanvasSymbol::default(); height]; width];
    TextCanvas { symbols }
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
        (self.symbols.len(), self.symbols[0].len())
    }

}
