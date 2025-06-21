use bevy::prelude::*;

pub const WINDOW_TITLE: &str = "Space Invaders: Remastered";
pub const WINDOW_BOUNDS: f32 = 100.0;
pub const WINDOW_BOUNDS_DAMAGE: f32 = 2.0;
pub const WINDOW_BOUNDS_MINIMUM_IMPACT: f32 = 0.1;

pub const INFO_TEXT: &str = "Up / Down Arrow: Move\nLeft / Right Arrow: Turn\nSpacebar: Fire";

pub const AMMO_PATH: &str = "embedded://fabsegame_v2/assets/ammo.webp";
pub const MUSIC_FILENAME: &str = "eye_of_the_tiger.flac";

pub const CONTROLLER_DEADZONE: f32 = 0.02;
pub const CONTROLLER_RUMBLE_DURATION_MS: u64 = 50;

pub const ENEMY_FILENAME: &str = "flopa.webp";
pub const ENEMY_SIZE_MAX: f32 = 1024.0;
pub const ENEMY_COUNT: u16 = 8;
pub const ENEMY_HEALTH: f32 = 20.0;
pub const ENEMY_SPEED: f32 = 100.0;

pub const ENEMY_WEAPON_COOLDOWN: f32 = 0.25;
pub const ENEMY_AMMO_DAMAGE: f32 = 5.0;
pub const ENEMY_AMMO_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const ENEMY_AMMO_SPEED: f32 = 1000.0;
pub const ENEMY_AMMO_RANGE: f32 = 400.0;

pub const SPACESHIP_FILENAME: &str = "spaceship.webp";
pub const SPACESHIP_SIZE: Vec2 = Vec2::new(128.0, 128.0);

pub const SPACESHIP_SPEED: f32 = 500.0;
pub const SPACESHIP_SPEED_INCREMENT: f32 = 2.0;

pub const SPACESHIP_ROTATION: f32 = f32::to_radians(360.0);
pub const SPACESHIP_ROTATION_INCREMENT: f32 = 1.5;
pub const SPACESHIP_ROTATION_BOUND: f32 = f32::to_radians(180.0);

pub const SPACESHIP_HEALTH: f32 = 1000.0;
pub const SPACESHIP_WEAPON_COOLDOWN: f32 = 0.1;
pub const SPACESHIP_AMMO_DAMAGE: f32 = 10.0;
pub const SPACESHIP_AMMO_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const SPACESHIP_AMMO_SPEED: f32 = 1500.0;
pub const SPACESHIP_AMMO_RANGE: f32 = 750.0;
