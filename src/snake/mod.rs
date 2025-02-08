//! Snake game module
//!
//! This module contains all components and systems related to the playable snake in the game.
//! The snake is made up of segments that follow each other, growing longer when food is eaten.
//!
//! Key components:
//! - Snake head component for tracking the lead segment
//! - Snake segment component for body parts
//! - Movement and growth systems
//! - Collision detection with food and self

use bevy::{
    color::Color,
    ecs::{
        component::Component, query::With, system::{Commands, Query, Res}
    },
    input::{keyboard::KeyCode, ButtonInput},
    sprite::Sprite,
    transform::components::Transform,
};

use crate::arena::{Position, Size};

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

#[derive(Component)]
pub struct SnakeHead;

pub fn spawn(mut commands: Commands) {
    commands
        .spawn(Sprite {
            color: SNAKE_HEAD_COLOR,
            ..Default::default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

pub fn movement(input: Res<ButtonInput<KeyCode>>, mut positions: Query<&mut Position, With<SnakeHead>>) {
    for mut position in positions.iter_mut() {
        if let Some(key) = [
            KeyCode::ArrowLeft, KeyCode::ArrowRight, KeyCode::ArrowDown, KeyCode::ArrowUp,
            KeyCode::KeyA, KeyCode::KeyD, KeyCode::KeyS, KeyCode::KeyW,
        ]
        .into_iter()
        .find(|key| input.pressed(*key))
        {
            match key {
                KeyCode::ArrowLeft | KeyCode::KeyA => position.x -= 1,
                KeyCode::ArrowRight | KeyCode::KeyD => position.x += 1,
                KeyCode::ArrowDown | KeyCode::KeyS => position.y -= 1,
                KeyCode::ArrowUp | KeyCode::KeyW => position.y += 1,
                _ => (),
            }
        }
    }
}
