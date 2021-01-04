use crate::entities::component::*;
use specs::{World, WorldExt, Builder, Entity};
use cursive::theme::{Color, BaseColor};

extern crate nalgebra as na;
use na::{Vector2};

pub fn make_player(world: &mut World) -> Entity {
    let starting_position = Position { vec2: Vector2::new(4, 4) };
    let appearence = Appearance {
        icon: '@',
        color: Color::Light(BaseColor::Cyan)
    };
    world.create_entity().with(starting_position).with(appearence).build()
}

pub fn make_camera(target: Entity, world: &mut World) -> Entity {
    let camera = Camera { target: target, offset: Vector2::new(0, 0) };
    world.create_entity()
         .with(camera)
        .with(Position { vec2: Vector2::new(0, 0) })
        .build()
}
