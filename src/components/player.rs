use bevy::prelude::*;

use super::common::{
	AmmoDamage, AmmoDirection, AmmoDistanceTraveled, AmmoRange, AmmoSize, AmmoSpeed, BoundsDamage,
	MaxRotation, MaxSpeed, MovementRotation, MovementSpeed, ShipHealth, ShipSize, WeaponCooldown,
};
use crate::constants::{
	SPACESHIP_AMMO_DAMAGE, SPACESHIP_AMMO_RANGE, SPACESHIP_AMMO_SIZE, SPACESHIP_AMMO_SPEED,
	SPACESHIP_HEALTH, SPACESHIP_ROTATION, SPACESHIP_SIZE, SPACESHIP_SPEED,
	SPACESHIP_WEAPON_COOLDOWN, WINDOW_BOUNDS_DAMAGE,
};

const fn default_player_ammo_speed() -> f32 {
	SPACESHIP_AMMO_SPEED
}

const fn default_player_ammo_size() -> Vec2 {
	SPACESHIP_AMMO_SIZE
}

const fn default_player_ammo_range() -> f32 {
	SPACESHIP_AMMO_RANGE
}

const fn default_player_ammo_damage() -> f32 {
	SPACESHIP_AMMO_DAMAGE
}

const fn default_player_ammo_distance_traveled() -> f32 {
	0.0
}

const fn default_player_ammo_direction() -> Vec3 {
	Vec3::new(0.0, 0.0, 0.0)
}

const fn default_player_health() -> f32 {
	SPACESHIP_HEALTH
}

const fn default_player_bounds_damage() -> f32 {
	WINDOW_BOUNDS_DAMAGE
}

const fn default_player_size() -> Vec2 {
	SPACESHIP_SIZE
}

const fn default_player_max_speed() -> f32 {
	SPACESHIP_SPEED
}

const fn default_player_max_rotation() -> f32 {
	SPACESHIP_ROTATION
}

const fn default_player_speed() -> f32 {
	0.0
}

const fn default_player_rotation() -> f32 {
	0.0
}

fn default_player_weapon_cooldown() -> Timer {
	Timer::from_seconds(SPACESHIP_WEAPON_COOLDOWN, TimerMode::Once)
}

#[derive(Message, Default)]
pub struct PlayerFireWeapon;

#[derive(Message, Default)]
pub struct PlayerWeaponImpact;

#[derive(Message, Default)]
pub struct PlayerBoundsImpact;

#[derive(Component, Default)]
#[require(
	AmmoSize(default_player_ammo_size()),
	AmmoSpeed(default_player_ammo_speed()),
	AmmoRange(default_player_ammo_range()),
	AmmoDistanceTraveled(default_player_ammo_distance_traveled()),
	AmmoDirection(default_player_ammo_direction()),
	AmmoDamage(default_player_ammo_damage())
)]
pub struct PlayerAmmo;

#[derive(Component, Default)]
#[require(PlayerAmmo, WeaponCooldown(default_player_weapon_cooldown()))]
pub struct PlayerWeapon;

#[derive(Component)]
pub struct PlayerHealthBar;

#[derive(Component, Default)]
#[require(
	MovementRotation(default_player_rotation()),
	MaxRotation(default_player_max_rotation()),
	MovementSpeed(default_player_speed()),
	MaxSpeed(default_player_max_speed()),
	ShipHealth(default_player_health()),
	ShipSize(default_player_size()),
	BoundsDamage(default_player_bounds_damage()),
	PlayerWeapon
)]
pub struct Player;
