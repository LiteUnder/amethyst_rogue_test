use amethyst::core::timing::Time;
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::rogue::player::{Player, PLAYER_SIZE};
use crate::rogue::{ROOM_HEIGHT, ROOM_WIDTH};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut player, input, time): Self::SystemData) {
        for (play, transform) in (&mut player, &mut transforms).join() {
            let movement_x = input.axis_value("player_x");
            let movement_y = input.axis_value("player_y");

            if let Some(mv_x) = movement_x {
                let player_x = transform.translation().x;

                let scaled_x = mv_x as f32 * time.time_scale();

                transform.set_x(
                    (player_x + scaled_x)
                        .max(PLAYER_SIZE * 0.5)
                        .min(ROOM_WIDTH - PLAYER_SIZE * 0.5),
                );

                // used by bullet for relative velocity
                play.velocity[0] = mv_x as f32;
            } else {
                play.velocity[0] = 0.0;
            }

            if let Some(mv_y) = movement_y {
                let scaled_y = mv_y as f32 * time.time_scale();

                let player_y = transform.translation().y;
                transform.set_y(
                    (player_y + scaled_y)
                        .max(PLAYER_SIZE * 0.5)
                        .min(ROOM_HEIGHT - PLAYER_SIZE * 0.5),
                );

                play.velocity[1] = mv_y as f32;
            } else {
                play.velocity[1] = 0.0;
            }
        }
    }
}
