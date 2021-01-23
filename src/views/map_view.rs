use cursive::theme::Style;
use cursive::utils::span::SpannedStr;
use cursive::utils::span;
use cursive::Printer;
use cursive::View;
use cursive::Vec2;
use crate::utility::text_canvas::TextCanvas;
use cursive::utils::markup::StyledString;
use std::sync::mpsc::Receiver;
use crate::world::map::Map;

/// View that updates based on the contents of the map from the main render loop
pub struct MapView {
    buffer: TextCanvas,
    rx: Receiver<TextCanvas>
}

impl MapView {

    pub fn new(map: &Map, rx: Receiver<TextCanvas>) -> Self {
        MapView {
            buffer: TextCanvas::for_map(map),
            rx
        }
    }

    fn update(&mut self) {
        while let Ok(canvas) = self.rx.try_recv() {
            self.buffer = canvas;
        }
    }

}

impl View for MapView {

    fn layout(&mut self, _: Vec2) {
        self.update()
    }

    fn draw(&self, printer: &Printer) {
        let mut canvas_lines = self.buffer.as_styled_strings();
        let lines_enumerated = canvas_lines.drain((printer.size.y)..).enumerate();

        for (i, line) in lines_enumerated {
            let line_as_str: SpannedStr<Style> = span::SpannedStr::from(&line);
            printer.print_styled((0, i), line_as_str);
        }
    }

}
