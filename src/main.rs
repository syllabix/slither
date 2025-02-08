use bevy::{
    prelude::*,
    window::{Window, WindowPlugin},
};

mod snake;
mod arena;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04))) // Dark gray background
        .add_systems(Startup, (setup_camera, snake::spawn).chain())
        .add_systems(Update, snake::movement)
        .add_systems(PostUpdate, (arena::position_translation, arena::scale_size))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake Game".into(),
                resolution: (500., 500.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
