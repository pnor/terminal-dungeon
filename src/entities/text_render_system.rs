use specs::{System, SystemData, ReadStorage, Read, Join};
use super::component::{Position, Appearance};
use super::super::world::map::Map;

pub struct TextRenderSystem;

impl<'a> System<'a> for TextRenderSystem {
    type SystemData = (
        Read<'a, Map>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Appearance>
        );

    fn run(&mut self, data: Self::SystemData) {
        let (map, pos, app) = data;

        for (pos, app) in (&pos, &app).join() {
            println!("{}", map.as_string());
            println!("----------");
            println!("pos is {:?}", pos);
            println!("app is {:?}", app);
        }
    }

}

struct Camera {
    center: i32,
    width: i32,
    height: i32
}

impl Camera {

    fn create_canvas() {
        // string it'll use to draw on
    }

    fn draw_map() {
        // draw the base map tiles
    }

}
