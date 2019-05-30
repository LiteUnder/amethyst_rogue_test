use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle,
    Texture, TextureMetadata, 
};

mod bullet;
use bullet::*;

pub struct Rogue;

pub const ROOM_WIDTH: f32 = 160.0;
pub const ROOM_HEIGHT: f32 = 90.0;

pub const PLAYER_SIZE: f32 = 16.0;

impl SimpleState for Rogue {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        // no longer needed
        // world.register::<Player>();
        //
        world.register::<Bullet>();

        init_bullet(world, sprite_sheet_handle.clone());
        init_player(world, sprite_sheet_handle);
        init_camera(world);
    }
}

pub struct Player {
    pub width: f32,
    pub height: f32,
}

impl Player {
    fn new() -> Player {
        Player {
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ROOM_WIDTH,
            0.0,
            ROOM_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn init_player(world: &mut World, sprite_sheet: SpriteSheetHandle) {
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

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // load in sprite sheet
    // // texture_handle is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "texture/player_spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/player_spritesheet.ron", // load associated RON file
        SpriteSheetFormat,
        texture_handle, // pass it the handle of the texture
        (),
        &sprite_sheet_store,
    )
}
