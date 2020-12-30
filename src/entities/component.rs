use specs::{Component, VecStorage};
use cursive::theme::Color;

/// An Entity's absolute position in the world
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

/// How an entitiy appears in the character-based world
/// Are fully opaque
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Appearance {
    pub icon: char,
    pub color: Color
}

/// How an entity affects entities below it when rendering
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Shade {
    pub color: Color,
    /// 0.0 .. 1.0
    pub alpha: f32
}
