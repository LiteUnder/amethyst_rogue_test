use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata, SpriteRender, 
};

pub mod bullet;

pub mod player;
use player::*; // re-export for system

pub struct Rogue;
pub struct BulletSprite {
    pub sprite: SpriteRender,
}

pub const ROOM_WIDTH: f32 = 160.0;
pub const ROOM_HEIGHT: f32 = 90.0;

impl SimpleState for Rogue {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        let bullet_sprite = BulletSprite {
            sprite: SpriteRender {
                sprite_sheet: sprite_sheet_handle.clone(),
                sprite_number: 1,
            }
        };

        world.add_resource(bullet_sprite);
            
        // no longer needed
        // world.register::<Player>();
        //
        //world.register::<Bullet>();
        init_player(world, sprite_sheet_handle);
        init_camera(world);
    }
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
