use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities};

use crate::rogue::{ROOM_HEIGHT, ROOM_WIDTH};
use crate::rogue::bullet::{BULLET_WIDTH, BULLET_HEIGHT, Bullet};

pub struct BulletSystem;

impl<'s> System<'s> for BulletSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Bullet>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut transforms, bullets, time): Self::SystemData) {
        for (e, local, bullet) in (&entities, &mut transforms, &bullets).join() {
            local.translate_x(bullet.velocity[0] * time.time_scale());
            local.translate_y(bullet.velocity[1] * time.time_scale());

            if local.translation().x < BULLET_WIDTH * -0.5 ||
                local.translation().x > ROOM_WIDTH + BULLET_WIDTH * 0.5 {
                    entities.delete(e).unwrap();
                }
        }
    }
}

