use std::time::Duration;

use bevy::{
	input::gamepad::{GamepadRumbleIntensity, GamepadRumbleRequest},
	prelude::*,
};

use crate::{
	components::{
		common::{
			AmmoDamage, AmmoDirection, AmmoDistanceTraveled, AmmoRange, AmmoSize, AmmoSpeed,
			BoundsDamage, GameBounds, ShipHealth,
		},
		enemy::{Enemy, EnemyAmmo, EnemyWeapon},
		player::{Player, PlayerAmmo, PlayerBoundsImpact, PlayerWeapon, PlayerWeaponImpact},
	},
	constants::CONTROLLER_RUMBLE_DURATION_MS,
	traits::{F32Component as _, Vec2Component as _, Vec3Component as _},
};

pub fn ammo_collision(
	mut commands: Commands,
	mut weapon_hit_event: EventWriter<PlayerWeaponImpact>,
	player_query: Single<(&Transform, &mut ShipHealth), (With<Player>, Without<Enemy>)>,
	mut enemy_query: Query<(Entity, &Transform, &mut ShipHealth), (With<Enemy>, Without<Player>)>,
	ammo_query: Query<
		(Entity, &Transform, &AmmoSize, &AmmoDamage),
		(
			Or<(With<EnemyAmmo>, With<PlayerAmmo>)>,
			(Without<Player>, Without<Enemy>),
		),
	>,
) {
	let (player_transform, mut player_health) = player_query.into_inner();
	let player_pos = player_transform.translation;

	for (ammo_entity, ammo_transform, ammo_size, ammo_damage) in ammo_query {
		if (player_pos.x.algebraic_sub(ammo_transform.translation.x)).abs() < ammo_size.get_x()
			&& (player_pos.y.algebraic_sub(ammo_transform.translation.y)).abs() < ammo_size.get_y()
		{
			player_health.sub(ammo_damage.get());
			weapon_hit_event.write_default();
			commands.entity(ammo_entity).despawn();
		} else {
			for (enemy_entity, enemy_transform, mut enemy_health) in &mut enemy_query {
				let enemy_pos = enemy_transform.translation;
				if (enemy_pos.x.algebraic_sub(ammo_transform.translation.x)).abs()
					< ammo_size.get_x()
					&& (enemy_pos.y.algebraic_sub(ammo_transform.translation.y)).abs()
						< ammo_size.get_y()
				{
					enemy_health.sub(ammo_damage.get());
					if enemy_health.get() <= 0.0 {
						commands.entity(enemy_entity).despawn();
					}
					commands.entity(ammo_entity).despawn();
					break;
				}
			}
		}
	}
}

pub fn bounds_collision(
	mut bounds_impact_event: EventWriter<PlayerBoundsImpact>,
	bounds: Res<GameBounds>,
	player_query: Single<(&Transform, &mut ShipHealth, &BoundsDamage), With<Player>>,
) {
	let (player_transform, mut player_health, player_bounds_damage) = player_query.into_inner();
	if (player_transform
		.translation
		.x
		.abs()
		.algebraic_sub(bounds.extents.x))
	.abs() < bounds.minimum_impact
		|| (player_transform
			.translation
			.y
			.abs()
			.algebraic_sub(bounds.extents.y))
		.abs() < bounds.minimum_impact
	{
		player_health.sub(player_bounds_damage.get());
		bounds_impact_event.write_default();
	}
}

pub fn bounds_impact(
	mut events: EventReader<PlayerBoundsImpact>,
	gamepad_query_opt: Option<Single<Entity, With<Gamepad>>>,
	mut rumble_requests: EventWriter<GamepadRumbleRequest>,
) {
	if let Some(gamepad_entity) = gamepad_query_opt {
		for _event in events.read() {
			rumble_requests.write(GamepadRumbleRequest::Add {
				duration: Duration::from_millis(CONTROLLER_RUMBLE_DURATION_MS),
				intensity: GamepadRumbleIntensity::WEAK_MAX,
				gamepad: *gamepad_entity,
			});
		}
	}
}

pub fn weapon_impact(
	mut events: EventReader<PlayerWeaponImpact>,
	gamepad_query_opt: Option<Single<Entity, With<Gamepad>>>,
	mut rumble_requests: EventWriter<GamepadRumbleRequest>,
) {
	if let Some(gamepad_entity) = gamepad_query_opt {
		for _event in events.read() {
			rumble_requests.write(GamepadRumbleRequest::Add {
				duration: Duration::from_millis(CONTROLLER_RUMBLE_DURATION_MS),
				intensity: GamepadRumbleIntensity::MAX,
				gamepad: *gamepad_entity,
			});
		}
	}
}

pub fn ammo_translation(
	time: Res<Time>,
	bounds: Res<GameBounds>,

	mut commands: Commands,
	ammo_query: Query<
		(
			Entity,
			&mut Transform,
			&AmmoSpeed,
			&AmmoRange,
			&mut AmmoDistanceTraveled,
			&AmmoDirection,
		),
		(
			Or<(With<EnemyAmmo>, With<PlayerAmmo>)>,
			(Without<EnemyWeapon>, Without<PlayerWeapon>),
		),
	>,
) {
	for (
		ammo_entity,
		mut ammo_transform,
		ammo_speed,
		ammo_range,
		mut ammo_distance_traveled,
		ammo_direction,
	) in ammo_query
	{
		let movement_distance = ammo_speed.get().algebraic_mul(time.delta_secs());

		ammo_transform.translation += ammo_direction.value() * movement_distance;

		ammo_distance_traveled.add(movement_distance);

		let translation_cmp = ammo_transform.translation.xy().abs().cmpge(bounds.extents);

		if translation_cmp.x
			|| translation_cmp.y
			|| ammo_distance_traveled.get() >= ammo_range.get()
		{
			commands.entity(ammo_entity).despawn();
		}
	}
}
