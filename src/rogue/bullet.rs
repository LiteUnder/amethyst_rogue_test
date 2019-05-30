use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
    Texture, TextureMetadata, 
};

use crate::rogue::{ROOM_WIDTH, ROOM_HEIGHT};

pub struct Bullet {
    velocity: [f32; 2],
}

impl Bullet {
    fn new() -> Bullet {
        Bullet {
            velocity: [0.0, 0.0],
        }
    }
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_bullet(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) {    
    let mut transform = Transform::default(); 
    transform.set_xyz(ROOM_WIDTH / 2.0, ROOM_HEIGHT / 2.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 1,
    };

    transform.set_scale(0.5, 0.5, 0.0);
    
    world
        .create_entity()
        .with(sprite_render)
        .with(Bullet::new())
        .with(transform)
        .build();
}
