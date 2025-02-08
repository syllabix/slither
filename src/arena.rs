//! Arena module
//!
//! This module handles the game arena/playing field where the snake moves around.
//! The arena defines the boundaries and constraints of the game space.
//!
//! Key responsibilities:
//! - Defining the arena dimensions and boundaries
//! - Handling collision detection with arena walls
//! - Managing the coordinate system for game entities
//! - Providing utilities for position validation

use bevy::{
    ecs::{component::Component, query::With, system::Query}, math::Vec3, transform::components::Transform, window::{PrimaryWindow, Window}
};

pub const WIDTH: f32 = 10.;
pub const HEIGHT: f32 = 10.;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

/// Scales the size of game entities based on the window dimensions.
///
/// This system adjusts the scale of entities with `Size` and `Transform` components to maintain
/// consistent proportions relative to the window size. The scaling ensures that game elements
/// appear at the correct size regardless of window dimensions.
///
/// # Arguments
/// * `window` - Query for the primary window to get current dimensions
/// * `size_transform` - Query for entities with both Size and Transform components
///
/// The scaling is calculated by:
/// 1. Getting the current window dimensions
/// 2. For each entity, computing scale factors based on:
///    - The entity's defined size (width/height)
///    - The game arena dimensions (WIDTH/HEIGHT constants)
///    - The current window dimensions
///
/// This maintains consistent relative sizes as the window is resized.
pub fn scale_size(window: Query<&Window, With<PrimaryWindow>>, mut size_transform: Query<(&Size, &mut Transform)>) {
    let window = window.single();
    for (size, mut transform) in size_transform.iter_mut() {
        transform.scale = Vec3::new(
            size.width / WIDTH * window.width(),
            size.height / WIDTH * window.height(),
            1.0
        )
    }
}

/// Converts a position from the game arena dimensions to the window dimensions.
///
/// This function converts a position from the game arena's coordinate system to the window's
/// coordinate system. It takes into account the size of the game arena and the window dimensions
/// to ensure that the position is correctly mapped.
///
fn convert(pos: f32, window_bounds: f32, game_bounds: f32) -> f32 {
    let tile_size = window_bounds / game_bounds;
    pos / game_bounds * window_bounds - (window_bounds / 2.) + (tile_size / 2.)
}

/// Translates the position of game entities based on the window dimensions.
///
/// This system adjusts the position of entities with `Position` and `Transform` components
/// to ensure they are correctly positioned relative to the window. The translation ensures
/// that game elements appear at the correct location regardless of window dimensions.
///
/// # Arguments
/// * `window` - Query for the primary window to get current dimensions
/// * `position_transform` - Query for entities with both Position and Transform components
///
/// The translation is calculated by:
/// 1. Getting the current window dimensions
/// 2. For each entity, converting the position to the correct location based on:
///    - The entity's position (x/y)
///    - The game arena dimensions (WIDTH/HEIGHT constants) 
pub fn position_translation(window: Query<&Window, With<PrimaryWindow>>, mut position_transform: Query<(&Position, &mut Transform)>) {
    let window = window.single();
    for (pos, mut transform) in position_transform.iter_mut() {
        let x = convert(pos.x as f32, window.width(), WIDTH);
        let y = convert(pos.y as f32, window.height(), HEIGHT);
        transform.translation = Vec3::new(x, y, 0.0);
    }
}