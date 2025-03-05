use bevy::prelude::*;
use rand::rng;

use crate::components::{bird::*, game_manager::GameManager, obstacle::Obstacle};
use crate::systems::obstacle_systems::{OBSTACLE_HEIGHT, PIXEL_RATIO};

use super::obstacle_systems::{OBSTACLE_WIDTH, spawn_obstacles}; // Import the constants

const FLAP_FORCE: f32 = 500.0;
const GRAVITY: f32 = 2000.0;
const VELOCITY_TO_ROTATION_RATIO: f32 = 7.5;

pub fn update_bird(
    mut commands: Commands,
    mut bird_query: Query<(&mut Bird, &mut Transform), Without<Obstacle>>,
    mut obstacle_query: Query<(&Transform, Entity), With<Obstacle>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    game_manager: Res<GameManager>,
) {
    if let Ok((mut bird, mut transform)) = bird_query.get_single_mut() {
        if keys.just_pressed(KeyCode::Space) {
            bird.velocity = FLAP_FORCE;
        }

        bird.velocity -= time.delta_secs() * GRAVITY;
        transform.translation.y += time.delta_secs() * bird.velocity;

        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity / VELOCITY_TO_ROTATION_RATIO, -90.0, 90.0).to_radians(),
        );

        let mut dead = false;
        if transform.translation.y <= -game_manager.window_dimensions.y / 2. {
            dead = true;
        } else {
            for (pipe_transform, _entity) in obstacle_query.iter() {
                if (pipe_transform.translation.y - transform.translation.y).abs()
                    < OBSTACLE_HEIGHT * PIXEL_RATIO / 2.
                    && (pipe_transform.translation.x - transform.translation.x).abs()
                        < OBSTACLE_WIDTH * PIXEL_RATIO / 2.
                {
                    dead = true;
                    break;
                }
            }
        }

        if dead {
            transform.translation = Vec3::ZERO;
            bird.velocity = 0.;
            for (pipe_transform, entity) in obstacle_query.iter_mut() {
                commands.entity(entity).despawn();
            }
            let mut rand = rng();
            spawn_obstacles(
                &mut commands,
                &mut rand,
                game_manager.window_dimensions.x,
                &game_manager.pipe_image,
            );
        }
    }
}
