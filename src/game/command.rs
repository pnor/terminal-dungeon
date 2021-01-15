/// Possible actions the player gives the games
#[derive(Copy, Clone)]
pub enum Command {
    None,
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
