use bevy::prelude::*;

use crate::{
	components::{
		common::{
			AmmoDirection, AmmoSize, HealthBar, MaxRotation, MaxSpeed, MovementRotation,
			MovementSpeed, ShipHealth, ShipSize,
		},
		player::{Player, PlayerAmmo, PlayerFireWeapon},
	},
	constants::{
		AMMO_PATH, BOUNDS_EXTENTS, SPACESHIP_FILENAME, SPACESHIP_HEALTH, SPACESHIP_POSITION,
		SPACESHIP_ROTATION_BOUND, SPACESHIP_SIZE,
	},
	traits::{F32Component as _, Vec2Component as _},
};

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn((
		Sprite {
			image: asset_server.load(SPACESHIP_FILENAME),
			custom_size: Some(SPACESHIP_SIZE),
			..Default::default()
		},
		Transform::from_xyz(SPACESHIP_POSITION.x, SPACESHIP_POSITION.y, 0.0),
		Player,
	));
}

pub fn player_movement(
	time: Res<Time>,
	player_query: Single<
		(
			&mut Transform,
			&mut MovementSpeed,
			&MaxSpeed,
			&mut MovementRotation,
			&MaxRotation,
		),
		With<Player>,
	>,
) {
	let (
		mut player_transform,
		mut player_velocity,
		player_max_velocity,
		mut player_rotation,
		player_max_rotation,
	) = player_query.into_inner();

	let rotation_value = player_rotation
		.get()
		.algebraic_mul(player_max_rotation.0)
		.algebraic_mul(time.delta_secs())
		.clamp(-SPACESHIP_ROTATION_BOUND, SPACESHIP_ROTATION_BOUND);

	player_transform.rotate_z(rotation_value);

	player_rotation.reset();

	let movement_direction = player_transform.rotation.mul_vec3(Vec3::Y);

	let movement_distance = player_velocity
		.get()
		.algebraic_mul(player_max_velocity.0)
		.algebraic_mul(time.delta_secs());

	player_transform.translation += movement_direction * movement_distance;

	let extents = Vec3::from((BOUNDS_EXTENTS, 0.0));
	let translation_bounded = player_transform.translation.clamp(-extents, extents);

	player_transform.translation = translation_bounded;

	player_velocity.reset();
}

pub fn player_ammo(
	mut commands: Commands,
	mut ammo_event: EventReader<PlayerFireWeapon>,
	assets_server: Res<AssetServer>,
	player_query: Single<(&Transform, &ShipSize, &AmmoSize), With<Player>>,
) {
	let ammo_handle = assets_server.load(AMMO_PATH);
	let (player_transform, player_size, ammo_size) = player_query.into_inner();
	let player_rotation = player_transform.rotation.mul_vec3(Vec3::Y);

	let spawn_offset_distance = player_size
		.get_y()
		.algebraic_div(2.0)
		.algebraic_add(ammo_size.0.y.algebraic_div(2.0));
	let spawn_offset = player_rotation * spawn_offset_distance;

	let spawn_position = player_transform.translation + spawn_offset;

	for _event in ammo_event.read() {
		commands.spawn((
			Sprite {
				image: ammo_handle.clone(),
				custom_size: Some(ammo_size.0),
				..Default::default()
			},
			Transform::from_translation(spawn_position),
			PlayerAmmo,
			AmmoDirection(player_rotation),
		));
	}
}

pub fn player_health(
	player_query: Single<&ShipHealth, With<Player>>,
	mut exit: EventWriter<AppExit>,
) {
	if player_query.0 <= 0.0 {
		exit.write(AppExit::Success);
	}
}

pub fn update_health_bar(
	player_query: Single<(&ShipHealth, &Transform, &ShipSize), With<Player>>,
	health_bar_query: Single<&mut Transform, (With<HealthBar>, Without<Player>)>,
) {
	let (player_health, player_transform, player_size) = player_query.into_inner();
	let health_percentage = (player_health.value().algebraic_div(SPACESHIP_HEALTH)).clamp(0.0, 1.0);

	let mut health_bar_transform = health_bar_query.into_inner();

	let offset_y = player_size.get_y().algebraic_div(2.0).algebraic_add(30.0);
	health_bar_transform.translation.x = player_transform.translation.x;
	health_bar_transform.translation.y = player_transform.translation.y.algebraic_add(offset_y);

	health_bar_transform.scale.x = health_percentage;
}
