mod components;
mod systems;

use bevy::prelude::*;
use bevy::window;
use bevy::window::PrimaryWindow;
use components::bird::*;
use components::game_manager::*;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use systems::bird_systems::*;
use systems::obstacle_systems::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Bevy Flappy The Bird"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(512., 512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup_level)
        .add_systems(Update, update_bird)
        .run();
}

const PIXEL_RATIO: f32 = 4.0;

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let pipe_image = asset_server.load("pipe.png");
    let window = window_query.get_single().unwrap();

    commands.insert_resource(GameManager {
        pipe_image: pipe_image.clone(),
        window_dimensions: Vec2::new(window.width(), window.height()),
    });

    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));

    commands.spawn(Camera2d::default());

    commands.spawn((
        Sprite {
            image: asset_server.load("bird.png"),
            ..Default::default()
        },
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
        Bird { velocity: 0.0 },
    ));

    let mut rand = thread_rng();

    spawn_obstacles(&mut commands, &mut rand, window.width(), &pipe_image);
}
