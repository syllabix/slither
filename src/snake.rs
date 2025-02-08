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

use std::time::Duration;

use bevy::{
    app::{FixedUpdate, Plugin, Startup, Update},
    color::Color,
    ecs::{
        component::Component,
        query::With,
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    input::{keyboard::KeyCode, ButtonInput},
    sprite::Sprite,
    time::{common_conditions::on_timer, Time, Timer, TimerMode},
};

use crate::arena::{Position, Size};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Direction::Left => Self::Right,
            Direction::Up => Self::Down,
            Direction::Right => Self::Left,
            Direction::Down => Self::Up,
        }
    }
}

const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

impl Default for SnakeHead {
    fn default() -> Self {
        Self {
            direction: Direction::Up,
        }
    }
}

fn spawn(mut commands: Commands) {
    commands
        .spawn(Sprite {
            color: SNAKE_HEAD_COLOR,
            ..Default::default()
        })
        .insert(SnakeHead::default())
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn handle_input(input: Res<ButtonInput<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    for mut head in heads.iter_mut() {
        if let Some(key) = [
            KeyCode::ArrowLeft,
            KeyCode::ArrowRight,
            KeyCode::ArrowDown,
            KeyCode::ArrowUp,
            KeyCode::KeyA,
            KeyCode::KeyD,
            KeyCode::KeyS,
            KeyCode::KeyW,
        ]
        .into_iter()
        .find(|key| input.pressed(*key))
        {
            let dir = match key {
                KeyCode::ArrowLeft | KeyCode::KeyA => Direction::Left,
                KeyCode::ArrowRight | KeyCode::KeyD => Direction::Right,
                KeyCode::ArrowDown | KeyCode::KeyS => Direction::Down,
                KeyCode::ArrowUp | KeyCode::KeyW => Direction::Up,
                _ => head.direction,
            };
            if dir != head.direction.opposite() {
                head.direction = dir
            }
        }
    }
}

#[derive(Resource)]
struct MovementTimer {
    clock: Timer,
}

impl MovementTimer {
    fn from_seconds(secs: f32) -> Self {
        Self {
            clock: Timer::from_seconds(secs, TimerMode::Repeating),
        }
    }
}

fn movement(
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
    mut heads: Query<(&mut Position, &SnakeHead)>,
) {
    if !timer.clock.tick(time.delta()).just_finished() {
        return;
    }
    if let Some((mut pos, head)) = heads.iter_mut().next() {
        match &head.direction {
            Direction::Left => pos.x -= 1,
            Direction::Up => pos.y += 1,
            Direction::Right => pos.x += 1,
            Direction::Down => pos.y -= 1,
        }
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let timer = MovementTimer::from_seconds(0.150);
        app.insert_resource(timer);
        app.add_systems(Startup, spawn);
        app.add_systems(FixedUpdate, (handle_input, movement).chain());
    }
}
