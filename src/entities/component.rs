use specs::{HashMapStorage, VecStorage, Component, Entity};
use cursive::theme::{Color, Effect};

extern crate nalgebra as na;
use na::{Vector2};


/// An Entity's absolute position in the world
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub vec2: Vector2<i32>
}

/// How an entitiy appears in the character-based world
/// Are fully opaque
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Appearance {
    pub icon: char,
    pub color: Color,
    pub face: Effect
}

/// How an entity affects entities below it when rendering
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Shade {
    pub color: Color,
    /// 0.0 .. 1.0
    pub alpha: f32
}

/// Camera that draws to a canvas
#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Camera {
    pub target: Entity,
    pub offset: Vector2<i32>,
}
