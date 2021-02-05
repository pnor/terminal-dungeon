use specs::{Read, System, ReadStorage, WriteStorage, Join};
use crate::entities::component::{CommandResponse, Position};
use crate::game::{Command, GameTick};

/// System for procesing commands
pub struct CommandSystem;

impl <'a> System<'a> for CommandSystem {
    type SystemData = (
        Read<'a, GameTick>,
        ReadStorage<'a, CommandResponse>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (game_tick, command_response, mut position) = data;

        for (_, position) in (&command_response, &mut position).join() {
            match *game_tick {
                GameTick::Command(_, command) => handle_motion(position, &command),
                GameTick::Tick(_) => ()
            }
        }
    }
}

fn handle_motion(position: &mut Position, command: &Command) {
    match *command {
        Command::Up => (*position).vec2[0] -= 1,
        Command::Down => (*position).vec2[0] += 1,
        Command::Left => (*position).vec2[1] -= 1,
        Command::Right => (*position).vec2[1] += 1,
        _ => ()
    };
}
