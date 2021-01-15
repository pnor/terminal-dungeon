use specs::{System, Entities, ReadStorage, WriteStorage, Join};
use crate::entities::component::{Position, Follow};

/// System for controlling camera movement
pub struct FollowSystem;

impl<'a> System<'a> for FollowSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Follow>,
    );

    fn run(&mut self, (entities, mut position, follow): Self::SystemData) {
        // Update camera so it follows its target
        for (entity, follow) in (&*entities, &follow).join() {
            if let Some(target_position) = position.get(follow.target) {
                let new_position = target_position.vec2 + follow.offset;

                if let Some(camera_position_comp) = position.get_mut(entity) {
                    camera_position_comp.vec2 = new_position;
                }
            }
        }
    }
}
