use bevy::prelude::*;

use super::common::{
	AmmoDamage, AmmoDirection, AmmoDistanceTraveled, AmmoRange, AmmoSize, AmmoSpeed, ShipHealth,
	ShipSize, WeaponCooldown,
};
use crate::constants::{
	BOUNDS_DAMAGE, ENEMY_AMMO_DAMAGE, ENEMY_AMMO_RANGE, ENEMY_AMMO_SIZE, ENEMY_AMMO_SPEED,
	ENEMY_HEALTH, ENEMY_WEAPON_COOLDOWN,
};

const fn default_enemy_ammo_speed() -> f32 {
	ENEMY_AMMO_SPEED
}

const fn default_enemy_ammo_size() -> Vec2 {
	ENEMY_AMMO_SIZE
}

const fn default_enemy_ammo_range() -> f32 {
	ENEMY_AMMO_RANGE
}

const fn default_enemy_ammo_damage() -> f32 {
	ENEMY_AMMO_DAMAGE
}

const fn default_enemy_ammo_distance_traveled() -> f32 {
	0.0
}

const fn default_enemy_ammo_direction() -> Vec3 {
	Vec3::new(0.0, 0.0, 0.0)
}

const fn default_enemy_bounds_damage() -> f32 {
	BOUNDS_DAMAGE
}

const fn default_enemy_health() -> f32 {
	ENEMY_HEALTH
}

fn default_enemy_weapon_cooldown() -> Timer {
	Timer::from_seconds(ENEMY_WEAPON_COOLDOWN, TimerMode::Once)
}

#[derive(Event, Default)]
pub struct EnemyFireWeapon;

#[derive(Event, Default)]
pub struct EnemyWeaponImpact;

#[derive(Component, Default)]
#[require(
	AmmoSize(default_enemy_ammo_size()),
	AmmoSpeed(default_enemy_ammo_speed()),
	AmmoRange(default_enemy_ammo_range()),
	AmmoDistanceTraveled(default_enemy_ammo_distance_traveled()),
	AmmoDirection(default_enemy_ammo_direction()),
	AmmoDamage(default_enemy_ammo_damage())
)]
pub struct EnemyAmmo;

#[derive(Component, Default)]
#[require(EnemyAmmo, WeaponCooldown(default_enemy_weapon_cooldown()))]
pub struct EnemyWeapon;

#[derive(Component, Default)]
#[require(ShipHealth(default_enemy_health()), ShipSize, EnemyWeapon)]
pub struct Enemy;
