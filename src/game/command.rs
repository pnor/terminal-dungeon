/// Possible actions the player gives the games
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
