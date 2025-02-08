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
    app::{Plugin, Startup, Update},
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    input::{keyboard::KeyCode, ButtonInput},
    sprite::Sprite,
    time::{Time, Timer, TimerMode},
};

use crate::{
    arena::{Position, Size, HEIGHT, WIDTH},
    food::Food,
};

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

    fn push(&mut self, e: Entity) {
        self.0.push(e);
    }
}

#[derive(Resource, Default)]
struct LastTailPosition(Option<Position>);

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
    mut last_tail_position: ResMut<LastTailPosition>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut game_over: EventWriter<GameOverEvent>,
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

            if head_pos.x < 0
                || head_pos.y < 0
                || head_pos.x as f32 >= WIDTH
                || head_pos.y as f32 >= HEIGHT
            {
                game_over.send(GameOverEvent);
            }

            if segment_positions.contains(&head_pos) {
                game_over.send(GameOverEvent);
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

        if let Some(last_segment) = segment_positions.last() {
            *last_tail_position = LastTailPosition(Some(*last_segment));
        }
    }
}

fn grow(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.read().next().is_some() {
        if let Some(last_position) = last_tail_position.0 {
            let segment = spawn_segment(commands, last_position);
            segments.push(segment);
        }
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segment_resource: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
    heads: Query<Entity, With<SnakeHead>>,
) {
    if reader.read().next().is_some() {
        for ent in food.iter().chain(heads.iter()).chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segment_resource);
    }
}

#[derive(Event)]
struct GrowthEvent;

fn eater(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

#[derive(Event)]
struct GameOverEvent;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        let timer = MovementTimer::from_seconds(0.150);
        app.insert_resource(timer);
        app.insert_resource(SnakeSegments::default());
        app.insert_resource(LastTailPosition::default());
        app.add_event::<GrowthEvent>();
        app.add_event::<GameOverEvent>();
        app.add_systems(Startup, spawn_snake);
        app.add_systems(
            Update,
            (handle_input, movement, game_over, eater, grow).chain(),
        );
    }
}
