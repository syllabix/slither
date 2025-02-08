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

use std::slice::Iter;

use bevy::{
    app::{FixedUpdate, Plugin, Startup},
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    input::{keyboard::KeyCode, ButtonInput},
    sprite::Sprite,
    time::{Time, Timer, TimerMode},
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
const SNAKE_SEGMENT_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);

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

#[derive(Component)]
struct SnakeSegment;

#[derive(Resource, Default)]
struct SnakeSegments(Vec<Entity>);

impl SnakeSegments {
    fn iter(&self) -> Iter<Entity> {
        self.0.iter()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(Sprite {
            color: SNAKE_SEGMENT_COLOR,
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    *segments = SnakeSegments(vec![
        commands
            .spawn(Sprite {
                color: SNAKE_HEAD_COLOR,
                ..Default::default()
            })
            .insert(SnakeHead::default())
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ]);
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
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if !timer.clock.tick(time.delta()).just_finished() {
        return;
    }
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions: Vec<Position> = segments
            .iter()
            .filter_map(|e| positions.get_mut(*e).ok().map(|p| *p))
            .collect();
        if segment_positions.len() != segments.len() {
            // Some segments were missing positions, exit early
            return;
        }
        if let Ok(mut head_pos) = positions.get_mut(head_entity) {
            match &head.direction {
                Direction::Left => head_pos.x -= 1,
                Direction::Up => head_pos.y += 1,
                Direction::Right => head_pos.x += 1,
                Direction::Down => head_pos.y -= 1,
            }
        }
        segment_positions
            .iter()
            .zip(segments.iter().skip(1))
            .for_each(|(pos, segment)| {
                if let Ok(mut position) = positions.get_mut(*segment) {
                    *position = *pos
                }
            });
    }
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let timer = MovementTimer::from_seconds(0.150);
        app.insert_resource(timer);
        app.insert_resource(SnakeSegments::default());
        app.add_systems(Startup, spawn_snake);
        app.add_systems(FixedUpdate, (handle_input, movement).chain());
    }
}
