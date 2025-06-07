use bevy::prelude::*;

pub const BOUNDS: Vec2 = Vec2::new(1920.0, 1080.0);

pub const CONTROLLER_DEADZONE: f32 = 0.01;
pub const CONTROLLER_RUMBLE_DURATION_MS: u64 = 50;

pub const ENEMY_FILENAME: &str = "flopa.png";
pub const ENEMY_SIZE_MAX: f32 = 1024.0;
pub const ENEMY_COUNT: u16 = 8;

pub const SPACESHIP_FILENAME: &str = "spaceship.png";
pub const SPACESHIP_SIZE: Vec2 = vec2(128.0, 128.0);
pub const SPACESHIP_POSITION: Vec2 = vec2(0.0, -BOUNDS.y);

pub const SPACESHIP_SPEED_STEP: f32 = 1.0;
pub const SPACESHIP_SPEED_FACTOR: f32 = 500.0;

pub const SPACESHIP_ROTATION_STEP: f32 = 1.0;
pub const SPACESHIP_ROTATION_FACTOR: f32 = f32::to_radians(360.0);
pub const SPACESHIP_ROTATION_BOUND: f32 = f32::to_radians(180.0);
