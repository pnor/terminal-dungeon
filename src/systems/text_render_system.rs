use std::convert::TryInto;
use specs::{System, SystemData, ReadStorage, Read, Write, Join};
use crate::entities::component::{Position, Appearance, Camera};
use crate::utility::text_canvas::{TextCanvas, CanvasSymbol};
use crate::world::map::{Map, Tile};
use cursive::theme::{Color, Effect};

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

        // Use the camera
        let mut camera_position: Option<&Position> = None;
        for (_, position) in (&cam, &pos).join() {
            camera_position = Some(position);
            break;
        }

        let camera_position = match camera_position {
            Some(pos) => pos,
            None => return // If there is no camera, there is nothing to render
        };

        // Draw the map
        let (start_x, end_x, start_y, end_y) = get_camera_corners(&camera_position, canvas.dimensions());

        for i in start_x..end_x {
            for j in start_y..end_y {
                let symbol: CanvasSymbol;
                let (map_width, map_height) = map.dimensions();

                // OOB
                // TODO avoid `as`
                if (i < 0 || j < 0) || (i as usize >= map_width || j as usize >= map_height) {
                    symbol = CanvasSymbol {
                        character: '?',
                        color: Color::Rgb(255, 50, 0),
                        face: Effect::Simple
                    };
                } else {
                    symbol = map_tile_to_canvas_symbol(&map.tiles[i as usize][j as usize]);
                }

                match world_to_canvas((i, j), &camera_position, &canvas) {
                    Some((x, y)) => canvas.set_symbol(Vector2::new(x, y), symbol),
                    None => ()
                };
            }
        }

        // Draw Entities
        for (position, appearence) in (&pos, &app).join() {
            let position = (as_i64(position.vec2[0]), as_i64(position.vec2[1]));

            match world_to_canvas(position, &camera_position, &canvas) {
                Some((x, y)) => {
                    let symbol = map_appearence_to_canvas_symbol(&appearence);
                    canvas.set_symbol(Vector2::new(x, y), symbol);
                }
                None => ()
            }
        }
    }

}

fn map_tile_to_canvas_symbol(tile: &Tile) -> CanvasSymbol {
    match tile {
        Tile::Blank => CanvasSymbol {
            character: ' ',
            color: Color::Rgb(0, 0, 0),
            face: Effect::Simple
        },
        Tile::Wall => CanvasSymbol {
            character: '#',
            color: Color::Rgb(20, 20, 20),
            face: Effect::Simple
        }
    }
}

fn map_appearence_to_canvas_symbol(appearence: &Appearance) -> CanvasSymbol {
    CanvasSymbol {
        character: appearence.icon,
        color: appearence.color,
        face: Effect::Simple
    }
}

/// Converts (x, y) in world space to (x, y) in the text_canvas_space.
/// Returns optional (x, y) tuple in text_canvas_space space, where (0, 0) is the top-left of the canvas.
/// Is None if the world space coordinate is not on the canvas (negative or OOB)
///
/// # Arguements
/// * `xy` - (x, y) tuple in world space
/// * `camera_center` - location in world space of where the camera is centered
/// * `canvas_dimension` - (width, height) of the canvas
fn world_to_canvas(
    xy: (i64, i64),
    camera_center: &Position,
    canvas: &TextCanvas
) -> Option<(usize, usize)> {
    let (canvas_width, canvas_height) = match canvas.dimensions() {
        (w, h) => (as_i64(w), as_i64(h))
    };
    let (center_x, center_y) = (as_i64(camera_center.vec2[0]), as_i64(camera_center.vec2[1]));
    let (x, y) = xy;

    let canvas_x = x - center_x + (canvas_width / 2);
    let canvas_y = y - center_y + (canvas_height / 2);

    if canvas.in_bounds(canvas_x, canvas_y) {
        Some((as_usize(canvas_x), as_usize(canvas_y)))
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
fn get_camera_corners(camera_position: &Position, canvas_dimensions: (usize, usize)) -> (i64, i64, i64, i64) {
    let (width, height) = canvas_dimensions;

    let start_x: i64 = as_i64(camera_position.vec2[0]) - as_i64(width / 2);
    let end_x: i64 = as_i64(camera_position.vec2[0]) + as_i64(width / 2);
    let start_y: i64 =  as_i64(camera_position.vec2[1]) - as_i64(height / 2);
    let end_y: i64 =  as_i64(camera_position.vec2[1]) + as_i64(height / 2);

    (start_x, end_x, start_y, end_y)
}

/// Converts `num` to `i64`, clamping it to `i64`'s max value if `num` is too big
fn as_i64(num: usize) -> i64 {
    match num.try_into() {
        Ok(result) => result,
        Err(_) => i64::MAX
    }
}

/// Converts `num` to `usize`, clamping it between 0 and `usize::MAX` if `num` is OOB
fn as_usize(num: i64) -> usize {
    match num.try_into() {
        Ok(result) => result,
        Err(_) => {
            if num < 0 {
                0
            } else {
                usize::MAX
            }
        }
    }
}
