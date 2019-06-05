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

        let mut bullet_trans = Transform::default();
        let mut player_vel: [f32; 2] = [0.0, 0.0];

        for (player, transform) in (&player, &transforms).join() {
            bullet_trans = transform.clone();
            player_vel = player.velocity;
        }

        let shoot_x = input.axis_value("shoot_x").unwrap_or(0.0) as f32;
        let shoot_y = input.axis_value("shoot_y").unwrap_or(0.0) as f32;


        if (shoot_x != 0.0 || shoot_y != 0.0) && self.delay <= 0 { 
            let scaled_vel = [shoot_x * 2.0 + player_vel[0], shoot_y * 2.0 + player_vel[1]];

            let vel = (scaled_vel[0].powi(2) + scaled_vel[1].powi(2)).sqrt(); 
            let rotation = scaled_vel[1].atan2(scaled_vel[0]);

            println!("rotation: {}\nvelocity: {}", rotation, vel);
            bullet_trans.set_rotation_euler(0.0, 0.0, rotation);

            entities
                .build_entity()
                .with(bullet_trans, &mut transforms)
                .with(Bullet::new(vel), &mut bullets)
                .with(bullet_sprite.sprite.clone(), &mut sprites)
                .build();
            self.delay = 30;
        }
    }
}
