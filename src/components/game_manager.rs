use bevy::prelude::*;

#[derive(Resource)]
pub struct GameManager {
    pub pipe_image: Handle<Image>,
    pub window_dimensions: Vec2,
}
