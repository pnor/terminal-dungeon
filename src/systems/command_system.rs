use specs::{Read, System, ReadStorage, WriteStorage, Join};
use crate::entities::component::{CommandResponse, Position};
use crate::game::Command;

/// System for procesing commands
pub struct CommandSystem;

impl <'a> System<'a> for CommandSystem {
    type SystemData = (
        Read<'a, Command>,
        ReadStorage<'a, CommandResponse>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (command, command_response, mut position) = data;

        for (_, position) in (&command_response, &mut position).join() {
            match *command {
                Command::Up => (*position).vec2[0] -= 1,
                Command::Down => (*position).vec2[0] += 1,
                Command::Left => (*position).vec2[1] -= 1,
                Command::Right => (*position).vec2[1] += 1,
                _ => ()
            };
        }
    }
}
