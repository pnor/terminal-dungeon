use specs::{HashMapStorage, VecStorage, Component, Entity};
use splines::{Spline, Interpolation, Key};
use std::time::Duration;
use tui::style::{Color, Modifier};

extern crate nalgebra as na;
use na::Vector2;


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
    pub foreground: Color,
    pub background: Color,
    pub modifiers: Vec<Modifier>
}

/// How an entity affects entities below it when rendering
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Shade {
    pub color: Color,
    /// 0.0 .. 1.0
    pub alpha: f32
}

/// Follows `target` entity with an offset
#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Follow {
    pub target: Entity,
    pub offset: Vector2<i32>,
}

/// Camera that draws to a canvas
#[derive(Component, Debug)]
#[storage(HashMapStorage)]
pub struct Camera;

/// Entity Movement through `Command`
#[derive(Component)]
#[storage(HashMapStorage)]
pub struct CommandResponse;

/// Entities that can collide with Walls or other entities with `Collision`
#[derive(Component)]
#[storage(VecStorage)]
pub struct Collision;

/// Component that stores animation progress
#[derive(Component)]
#[storage(VecStorage)]
pub struct AnimationProgress {
    pub current: Duration,
    pub total: Duration,
}

/// Animate `Position` using splines
#[derive(Component)]
#[storage(VecStorage)]
pub struct PositionAnimation {
    /// Spline used to sample for x position
    pub x_spline: Spline<f32, i32>,
    /// Spline used to sample for y position
    pub y_spline: Spline<f32, i32>,
}

impl PositionAnimation {
    /// Create `PositionAnimation` that uses `interpolation` for entirety of both the x spline and the y spline
    fn new(start: Vector2<i32>, end: Vector2<i32>, interpolation: Interpolation<f32, i32>) -> Self {
        let x_start = Key::new(0., start[0], interpolation);
        let x_end = Key::new(1., end[0], interpolation);
        let x_spline = Spline::from_vec(vec![x_start, x_end]);

        let y_start = Key::new(0., start[1], interpolation);
        let y_end = Key::new(1., end[1], interpolation);
        let y_spline = Spline::from_vec(vec![y_start, y_end]);

        PositionAnimation { x_spline, y_spline }
    }

    /// Create `PositionAnimation` with custom splines for both x and y
    fn with_splines(x_spline: Spline<f32, i32>, y_spline: Spline<f32, i32>) -> Self {
        PositionAnimation { x_spline, y_spline }
    }
}

/// Animate `Appearance` using splines
#[derive(Component)]
#[storage(VecStorage)]
pub struct AppearanceAnimation {
    pub foreground_spline: Option<Spline<f32, (u8, u8, u8)>>,
    pub background_spline: Option<Spline<f32, (u8, u8, u8)>>,
    // TODO step function for icon and modifiers...
}
