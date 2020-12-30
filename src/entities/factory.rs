use crate::entities::component::*;
use specs::{World, WorldExt, Builder, Entity};
use cursive::theme::{Color, BaseColor};

pub fn make_player(world: &mut World) -> Entity {
    let starting_position = Position { x: 4, y: 4 };
    let appearence = Appearance {
        icon: '@',
        color: Color::Light(BaseColor::Cyan)
    };
    world.create_entity().with(starting_position).with(appearence).build()
}
