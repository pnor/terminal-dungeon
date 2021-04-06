use specs::{Read, System, ReadStorage, WriteStorage, Join};
use crate::entities::component::{CommandResponse, Position, Collision};
use crate::game::{Command, GameTick};
use crate::world::map::Map;

extern crate nalgebra as na;
use na::Vector2;

/// System for processing commands
pub struct CommandSystem;

impl <'a> System<'a> for CommandSystem {
    type SystemData = (
        Read<'a, GameTick>,
        Read<'a, Map>,
        ReadStorage<'a, CommandResponse>,
        ReadStorage<'a, Collision>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (game_tick, map, command_response, collision, mut position_storage) = data;

        let mut collidable_entity_locations = vec!();
        for (pos, _) in (&mut position_storage, &collision).join() {
            collidable_entity_locations.push(pos.vec2);
        }

        for (_, position) in (&command_response, &mut position_storage).join() {
            match *game_tick {
                GameTick::Command(_, command) => {
                    let new_position = get_target_position(position, &command);

                    if can_move_onto(new_position, &map, &collidable_entity_locations) {
                        (*position).vec2 = new_position
                    }
                },
                GameTick::Tick(_) => ()
            }
        }
    }
}

/// Returns coordinate that `command` would move `Position` to
fn get_target_position(position: &mut Position, command: &Command) -> Vector2<i32> {
    let mut new_position = (*position).vec2;

    match *command {
        Command::Up => new_position[0] -= 1,
        Command::Down => new_position[0] += 1,
        Command::Left => new_position[1] -= 1,
        Command::Right => new_position[1] += 1,
        _ => ()
    };

    new_position
}

/// Returns true if there is nothing obstructing movement at `location` on `map`
fn can_move_onto(location: Vector2<i32>, map: &Map, collidable_locations: &Vec<Vector2<i32>>) -> bool {
    map.is_open(location[0], location[1]) &&
        !collidable_locations.iter().any(|&v| v[0] == location[0] && v[1] == location[1])
}
