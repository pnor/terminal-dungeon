use tui::style::{Color, Modifier};
use specs::{System, ReadStorage, Read, Write, Join};

use crate::entities::component::{Position, Appearance, Camera};
use crate::utility::text_canvas::{TextCanvas, CanvasSymbol};
use crate::utility::conversions;
use crate::world::map::{Map, Tile};

extern crate nalgebra as na;
use na::Vector2;

/// System that renders the area near a camera onto a TextCanvas
pub struct TextRenderSystem;

impl<'a> System<'a> for TextRenderSystem {
    type SystemData = (
        Read<'a, Map>,
        Write<'a, TextCanvas>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Appearance>,
        ReadStorage<'a, Camera>
    );


    fn run(&mut self, data: Self::SystemData) {
        let (map, mut canvas, pos, app, cam) = data;

        let camera_position = match get_camera_position(&cam, &pos) {
            Some(camera_position) => camera_position,
            None => return // Can't render without a camera...
        };

        draw_map(camera_position, &mut canvas, &map);

        draw_entities(&pos, &app, camera_position, &mut canvas);
    }

}

/// Returns reference to the `Position` of a camera in the world
/// The Camera chosen is essentially random
fn get_camera_position<'a>(
    cam: &'a ReadStorage<Camera>,
    pos: &'a ReadStorage<Position>
) -> Option<&'a Position> {
    // Use the camera
    let mut camera_position: Option<&Position> = None;
    for (_, position) in (&*cam, &*pos).join() {
        camera_position = Some(position);
        break;
    }
    camera_position
}

fn draw_map(camera_position: &Position, canvas: &mut TextCanvas, map: &Map) {
    let (start_x, end_x, start_y, end_y) = get_camera_corners(&camera_position, canvas.dimensions());

    for i in start_x..end_x {
        for j in start_y..end_y {
            let symbol = get_symbol_for_location(i, j, map);

            match world_to_canvas((i, j), camera_position, &canvas) {
                Some((x, y)) => canvas.set_symbol(Vector2::new(x, y), symbol),
                None => ()
            };
        }
    }
}


/// Gets the `CanvasSymbol` for the `(x, y)` pair on the screen. `(x, y)` are
/// the coordinates in world space
fn get_symbol_for_location(x: i32, y: i32, map: &Map) -> CanvasSymbol {
    if map.in_bounds(x, y) {
        get_canvas_symbol_for_tile(&map[x as usize][y as usize])
    } else {
        CanvasSymbol {
            character: '?',
            foreground: Color::Rgb(30, 30, 30),
            background: Color::Rgb(0, 0, 0),
            modifiers: vec!()
        }
    }
}

fn draw_entities(
    pos: &ReadStorage<Position>,
    app: &ReadStorage<Appearance>,
    camera_position: &Position,
    canvas: &mut TextCanvas
) {
        for (position, appearence) in (&*pos, &*app).join() {
            let position = (position.vec2[0], position.vec2[1]);

            if let Some((x, y)) = world_to_canvas(position, &camera_position, &canvas) {
                let symbol = get_canvas_symbol_for_appearence(appearence);

                canvas.set_symbol(Vector2::new(x, y), symbol);
            }
        }
}

/// Returns `CanvasSymbol` that corresponds to the `Tile` of a tilemap
/// TODO should move to another module?
fn get_canvas_symbol_for_tile(tile: &Tile) -> CanvasSymbol {
    match tile {
        Tile::Blank => CanvasSymbol {
            character: ' ',
            foreground: Color::Black,
            background: Color::Black,
            modifiers: vec!()
        },
        Tile::Wall => CanvasSymbol {
            character: '#',
            foreground: Color::Black,
            background: Color::White,
            modifiers: vec!()
        }
    }
}

fn get_canvas_symbol_for_appearence(appearence: &Appearance) -> CanvasSymbol {
    CanvasSymbol {
        character: appearence.icon,
        foreground: appearence.foreground,
        background: appearence.background,
        modifiers: appearence.modifiers.clone()
    }
}

/// Converts (x, y) in world space to (x, y) in the text_canvas_space.
/// Returns optional (x, y) tuple in text_canvas_space space, where (0, 0) is the top-left of the canvas.
/// Is None if the world space coordinate is not on the canvas (negative or OOB)
///
/// # Arguements
///
/// * `xy` - (x, y) tuple in world space
/// * `camera_center` - location in world space of where the camera is centered
/// * `canvas_dimension` - (width, height) of the canvas
fn world_to_canvas(
    xy: (i32, i32),
    camera_center: &Position,
    canvas: &TextCanvas
) -> Option<(usize, usize)> {
    let (canvas_width, canvas_height) = match canvas.dimensions() {
        (w, h) => (conversions::as_i32(w), conversions::as_i32(h))
    };
    let (center_x, center_y) = (camera_center.vec2[0], camera_center.vec2[1]);
    let (x, y) = xy;

    let canvas_x = x - center_x + (canvas_width / 2);
    let canvas_y = y - center_y + (canvas_height / 2);

    if canvas.in_bounds(canvas_x, canvas_y) {
        Some((conversions::as_usize(canvas_x), conversions::as_usize(canvas_y)))
    } else {
        None
    }
}

/// Returns tuple of the four corners/extreeme x,y values of the area a camera sees
/// Returns in form: (min x, max x, min y, max y)
///
/// # Arguements
/// * `camera_position` - positon the camera is centered at
/// * `canvas_dimension` - (width, height) of canvas
fn get_camera_corners(camera_position: &Position, canvas_dimensions: (usize, usize)) -> (i32, i32, i32, i32) {
    let (width, height) = canvas_dimensions;

    let start_x: i32 = camera_position.vec2[0] - conversions::as_i32(width / 2);
    let end_x: i32 = camera_position.vec2[0] + conversions::as_i32(width / 2);
    let start_y: i32 =  camera_position.vec2[1] - conversions::as_i32(height / 2);
    let end_y: i32 =  camera_position.vec2[1] + conversions::as_i32(height / 2);

    (start_x, end_x, start_y, end_y)
}
