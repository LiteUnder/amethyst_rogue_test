use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    SpriteRender, SpriteSheetHandle, 
};

use crate::rogue::{ROOM_WIDTH, ROOM_HEIGHT};

pub const BULLET_WIDTH: f32 = 4.25;
pub const BULLET_HEIGHT: f32 = 2.0;

pub struct Bullet {
    pub velocity: [f32; 2],
}

impl Bullet {
    pub fn new(vel: [f32; 2]) -> Bullet {
        Bullet {
            velocity: vel, 
        }
    }
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}

pub fn spawn_bullet(world: &mut World, sprite_sheet_handle: SpriteSheetHandle, vel: [f32; 2]) {    
    let mut transform = Transform::default(); 
    transform.set_xyz(ROOM_WIDTH / 2.0, ROOM_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    transform.set_scale(0.25, 0.25, 0.0);
    
    //let mut rotation: f32 = 0.0;

    // if vel[0] == -2.0 {
    //    rotation = 180.0;
    //

    // if vel[1] == 2.0 {
    //    rotation = 90.0;
    // } else if vel[1] == -2.0 {
    //    rotation = 270.0;
    //}


    world
        .create_entity()
        .with(sprite_render)
        .with(Bullet::new(vel))
        .with(transform)
        .build();
}
