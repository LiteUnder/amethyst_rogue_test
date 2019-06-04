use amethyst::core::Transform;
use amethyst::ecs::Entities;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, ReadExpect};
use amethyst::input::InputHandler;
use amethyst::renderer::{SpriteRender};

use crate::rogue::bullet::Bullet;
use crate::rogue::player::Player;
use crate::rogue::BulletSprite;

pub struct ShootSystem {
    pub delay: u32,
}


impl<'s> System<'s> for ShootSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Bullet>,
        Read<'s, InputHandler<String, String>>,
        Entities<'s>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, BulletSprite>,
    );


    fn run(&mut self, (mut transforms, mut bullets, input, entities, player, mut sprites, bullet_sprite): Self::SystemData) {
        if self.delay != 0 {
            self.delay -= 1;
        }

        let mut player_trans = Transform::default();
        let mut player_vel: [f32; 2] = [0.0, 0.0];

        for (player, transform) in (&player, &transforms).join() {
            player_trans = transform.clone();
            player_vel = player.velocity;
        }

        let shoot_x = input.axis_value("shoot_x").unwrap_or(0.0) as f32;
        let shoot_y = input.axis_value("shoot_y").unwrap_or(0.0) as f32;

        if (shoot_x != 0.0 || shoot_y != 0.0) && self.delay <= 0 { 
            
            
            entities
                .build_entity()
                .with(player_trans, &mut transforms)
                .with(Bullet::new([shoot_x * 2.0 + player_vel[0], shoot_y * 2.0 + player_vel[1]]), &mut bullets)
                .with(bullet_sprite.sprite.clone(), &mut sprites)
                .build();
            self.delay = 30;
        }
    }
}
