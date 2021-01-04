use specs::{System, SystemData, Entities, ReadStorage, WriteStorage, Join};
use crate::entities::component::{Position, Camera};

extern crate nalgebra as na;
use na::{Vector2};

/// System for controlling camera movement
pub struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Camera>,
    );

    fn run(&mut self, (entities, mut position, camera): Self::SystemData) {
        // Update camera so it follows its target
        for (entity, camera) in (&*entities, &camera).join() {
            if let Some(target_position) = position.get(camera.target) {
                let new_position = target_position.vec2 + camera.offset;

                if let Some(camera_position_comp) = position.get_mut(entity) {
                    camera_position_comp.vec2 = new_position;
                }
            }
        }
    }
}
