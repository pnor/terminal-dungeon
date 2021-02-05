use std::time::Duration;

/// Possible actions the player gives the games
#[derive(Copy, PartialEq)]
pub enum GameTick {
    Tick(Duration),
    Command(Duration, Command)
}

impl Default for GameTick {

    fn default() -> Self {
        Self::Tick(Duration::from_millis(0))
    }

}

impl Clone for GameTick {

    fn clone(&self) -> Self {
        match self {
            Self::Tick(dur) => GameTick::Tick(*dur),
            Self::Command(dur, command) => GameTick::Command(*dur, *command)
        }
    }

}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Command {
     Up,
     Down,
     Left,
     Right
}
