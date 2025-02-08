use bevy::{
    prelude::*,
    window::{Window, WindowPlugin},
};

use food::FoodPlugin;
use snake::SnakePlugin;

mod snake;
mod arena;
mod food;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04))) // Dark gray background
        .add_systems(Startup, setup_camera)
        .add_plugins((SnakePlugin, FoodPlugin))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake Game".into(),
                resolution: (500., 500.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(PostUpdate, (arena::position_translation, arena::scale_size))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
