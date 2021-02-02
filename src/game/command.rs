use std::time::Duration;

/// Possible actions the player gives the games
#[derive(Copy, Clone, PartialEq)]
pub enum Command {
    None,
    Tick(Duration),
    Up,
    Down,
    Left,
    Right
}

impl Default for Command {

    fn default() -> Self {
        Self::None
    }

}
