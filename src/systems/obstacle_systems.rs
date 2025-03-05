use bevy::prelude::*;
use rand::{Rng, rng, rngs::ThreadRng};

use crate::components::{game_manager::GameManager, obstacle::*};

pub const PIXEL_RATIO: f32 = 4.0;

pub const OBSTACLE_AMOUNT: i32 = 5;
pub const OBSTACLE_WIDTH: f32 = 32.;
pub const OBSTACLE_HEIGHT: f32 = 144.;
pub const OBSTACLE_VERTICAL_OFFSET: f32 = 30.;
pub const OBSTACLE_GAP_SIZE: f32 = 15.;
pub const OBSTACLE_SPACING: f32 = 60.;
pub const OBSTACLE_SCROLL_SPEED: f32 = 150.;

fn get_centered_pipe_position() -> f32 {
    return (OBSTACLE_HEIGHT / 2. + OBSTACLE_GAP_SIZE) * PIXEL_RATIO;
}

pub fn spawn_obstacles(
    mut commands: &mut Commands,
    mut rand: &mut ThreadRng,
    window_width: f32,
    pipe_image: &Handle<Image>,
) {
    for i in 0..OBSTACLE_AMOUNT {
        let y_offset = generate_offset(rand);
        let x_pos = window_width / 2. + (OBSTACLE_SPACING * PIXEL_RATIO * i as f32);

        spawn_obstacle(
            Vec3::X * x_pos + Vec3::Y * (get_centered_pipe_position() + y_offset),
            1.,
            commands,
            pipe_image,
        );

        spawn_obstacle(
            Vec3::X * x_pos + Vec3::Y * (-get_centered_pipe_position() + y_offset),
            -1.,
            commands,
            pipe_image,
        );
    }
}

fn spawn_obstacle(
    translation: Vec3,
    //bottom or top of screen
    pipe_direction: f32,
    commands: &mut Commands,
    pipe_image: &Handle<Image>,
) {
    commands.spawn((
        Sprite {
            image: pipe_image.clone(),
            ..Default::default()
        },
        Transform::from_translation(translation).with_scale(Vec3::new(
            PIXEL_RATIO,
            PIXEL_RATIO * -pipe_direction,
            PIXEL_RATIO,
        )),
        Obstacle { pipe_direction },
    ));
}

pub fn update_obstacle(
    time: Res<Time>,
    game_manager: Res<GameManager>,
    mut obstacle_query: Query<(&mut Obstacle, &mut Transform)>,
) {
    let mut rand = rng();
    let y_offset = generate_offset(&mut rand);

    for (obstacle, mut transform) in obstacle_query.iter_mut() {
        transform.translation.x -= time.delta_secs() * OBSTACLE_SCROLL_SPEED;

        if transform.translation.x + OBSTACLE_WIDTH * PIXEL_RATIO / 2.
            < -game_manager.window_dimensions.x / 2.
        {
            transform.translation.x += OBSTACLE_AMOUNT as f32 * OBSTACLE_SPACING * PIXEL_RATIO;
            transform.translation.y =
                get_centered_pipe_position() * obstacle.pipe_direction + y_offset;
        }
    }
}

fn generate_offset(rand: &mut ThreadRng) -> f32 {
    return rand.random_range(-OBSTACLE_VERTICAL_OFFSET..OBSTACLE_VERTICAL_OFFSET) * PIXEL_RATIO;
}
