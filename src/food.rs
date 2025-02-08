//! Food module for the snake game
//! 
//! This module handles the food mechanics in the snake game, including:
//! - Spawning food at random positions in the arena
//! - Handling food collection when the snake collides with it
//! - Managing food appearance and visual representation
//! 
//! Food appears as collectible items that the snake can eat to grow longer.
//! When collected, new food spawns at a random unoccupied position.
//! 
//! # Components
//! - `Food` - Marks an entity as food that can be collected by the snake
//! 
//! # Systems
//! - `spawn_food` - Spawns initial food and respawns food when collected
//! - `food_collection` - Detects snake collision with food and handles collection
//!
//! Food positions are constrained to the game arena grid to maintain consistent
//! gameplay mechanics with the snake's movement.

use core::f32;

use bevy::prelude::*;
use rand::random;

use crate::arena::{self, Position, Size};

const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

#[derive(Resource)]
struct FoodTimer {
    clock: Timer
}

impl FoodTimer {
    fn from_seconds(secs: f32) -> Self {
        Self { clock: Timer::from_seconds(secs, TimerMode::Repeating) }
    }
}

/// Component that marks an entity as collectible food
#[derive(Component)]
pub struct Food;

/// Spawns initial food and respawns food when collected
fn spawn(time: Res<Time>, mut timer: ResMut<FoodTimer>, mut commands: Commands) {
    if timer.clock.tick(time.delta()).just_finished() {
        let x = (random::<f32>() * arena::WIDTH) as i32;
        let y = (random::<f32>() * arena::HEIGHT) as i32;
        commands.spawn(Sprite {
            color: FOOD_COLOR,
            ..Default::default()
        })
        .insert(Food)
        .insert(Position { x, y })
        .insert(Size::square(0.8));
    }
}

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        let timer = FoodTimer::from_seconds(2.0);
        app.insert_resource(timer);
        app.add_systems(Update, spawn);
    }
}
