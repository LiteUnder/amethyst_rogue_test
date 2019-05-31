use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::rogue::{ROOM_HEIGHT, ROOM_WIDTH};
use crate::rogue::player::{Player, PLAYER_SIZE};
use crate::rogue::bullet::{BULLET_WIDTH, BULLET_HEIGHT, Bullet};

pub struct BulletSystem;

impl<'s> System<'s> for BulletSystem {
   type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Bullet>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, bullets, time): Self::SystemData) {
        for (local, bullet) in (&mut transforms, &bullets).join() {
            local.translate_x(bullet.velocity[0] * time.time_scale());
            local.translate_x(bullet.velocity[1] * time.time_scale());
        }
    }
}

