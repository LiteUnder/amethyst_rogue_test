use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    SpriteRender, SpriteSheetHandle, 
};

use crate::rogue::{ROOM_WIDTH, ROOM_HEIGHT};

pub const PLAYER_SIZE: f32 = 16.0;

pub struct Player {
    pub width: f32,
    pub height: f32,
    pub velocity: [f32; 2],
}

impl Player {
    fn new() -> Player {
        Player {
            width: 1.0,
            height: 1.0,
            velocity: [0.0, 0.0],
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_player(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut transform = Transform::default();
    
    transform.set_xyz(ROOM_WIDTH * 0.5, ROOM_HEIGHT * 0.5, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // player is the first sprite in sprite_sheet
    };

    transform.set_scale(0.5, 0.5, 0.0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Player::new())
        .with(transform)
        .build();
}
