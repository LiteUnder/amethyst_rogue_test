use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use crate::rogue::{Player, PLAYER_SIZE, ROOM_HEIGHT, ROOM_WIDTH};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, player, input): Self::SystemData) {
        for (_, transform) in (&player, &mut transforms).join() {
            let movement_x = input.axis_value("player_x");
            let movement_y = input.axis_value("player_y");

            if let Some(mv_x) = movement_x {
                // transform.translate_x(mv_x as f32);

                let player_x = transform.translation().x;
                transform.set_x(
                    (player_x + mv_x as f32)
                        .max(PLAYER_SIZE * 0.5)
                        .min(ROOM_WIDTH - PLAYER_SIZE * 0.5),
                );
            }

            if let Some(mv_y) = movement_y {
                let player_y = transform.translation().y;
                transform.set_y(
                    (player_y + mv_y as f32)
                        .max(PLAYER_SIZE * 0.5)
                        .min(ROOM_HEIGHT - PLAYER_SIZE * 0.5),
                );
            }
        }
    }
}
