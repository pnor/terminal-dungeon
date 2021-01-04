use std::convert::TryInto;
use specs::{System, SystemData, ReadStorage, Read, Write, Join};
use crate::entities::component::{Position, Appearance, Camera};
use crate::utility::text_canvas::{TextCanvas, CanvasSymbol};
use crate::world::map::{Map, Tile};
use cursive::theme::{Color, Effect};

extern crate nalgebra as na;
use na::{Vector2, clamp};

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
        let (width, height) = canvas.dimensions();

        let start_x: i64 = as_i64(camera_position.vec2[0]) - as_i64(width / 2);
        let end_x: i64 = as_i64(camera_position.vec2[0]) + as_i64(width / 2);
        let start_y: i64 =  as_i64(camera_position.vec2[1]) - as_i64(height / 2);
        let end_y: i64 =  as_i64(camera_position.vec2[1]) + as_i64(height / 2);

        let mut canvas_row: usize = 0;
        let mut canvas_col: usize = 0;

        for i in start_x..end_x {
            for j in start_y..end_y {
                let mut symbol: CanvasSymbol;

                // OOB
                let (map_width, map_height) = map.dimensions();
                if (i < 0 || j < 0) || (i as usize >= map_width || j as usize >= map_height) {
                    symbol = CanvasSymbol {
                        character: '?',
                        color: Color::Rgb(255, 100, 0),
                        face: Effect::Simple
                    };
                } else {
                    symbol = map_tile_to_canvas_symbol(&map.tiles[i as usize][j as usize]);
                }

                canvas.set_symbol(Vector2::new(canvas_row, canvas_col), symbol);

                canvas_row += 1;
            }
            canvas_row = 0;
            canvas_col += 1;
        }


        // Draw entities
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
            color: Color::Rgb(200, 200, 200),
            face: Effect::Bold
        }
    }
}

fn as_i64(num: usize) -> i64 {
    num.try_into().unwrap()
}
