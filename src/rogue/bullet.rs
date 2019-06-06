use amethyst::ecs::prelude::{Component, DenseVecStorage};

// sprite size * 0.25
pub const BULLET_WIDTH: f32 = 2.0;
pub const BULLET_HEIGHT: f32 = 2.0;

pub struct Bullet {
    pub velocity: f32,
}

impl Bullet {
    pub fn new(vel: f32) -> Bullet {
        Bullet { velocity: vel }
    }
}

impl Component for Bullet {
    type Storage = DenseVecStorage<Self>;
}
