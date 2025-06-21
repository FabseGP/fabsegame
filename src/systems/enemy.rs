use bevy::prelude::*;

use crate::{
	components::{
		common::{AmmoDirection, AmmoSize, GameBounds, MaxSpeed, ShipSize, WeaponCooldown},
		enemy::{Enemy, EnemyAmmo},
		player::Player,
	},
	constants::{AMMO_PATH, ENEMY_COUNT, ENEMY_FILENAME, ENEMY_SIZE_MAX},
	traits::{F32Component as _, Vec2Component as _},
};

pub fn spawn_enemy(
	mut commands: Commands,
	bounds: Res<GameBounds>,
	asset_server: Res<AssetServer>,
) {
	let enemy_handle = asset_server.load(ENEMY_FILENAME);

	let enemy_size = vec2(
		ENEMY_SIZE_MAX.algebraic_div(f32::from(ENEMY_COUNT)),
		ENEMY_SIZE_MAX.algebraic_div(f32::from(ENEMY_COUNT)),
	);

	for i in 0..ENEMY_COUNT {
		let x_pos = bounds.extents.x.algebraic_sub(f32::max(
			enemy_size.x.algebraic_mul(f32::from(i)),
			bounds
				.extents
				.x
				.algebraic_div(f32::from(ENEMY_COUNT))
				.algebraic_mul(f32::from(i))
				.algebraic_mul(2.0),
		));

		commands.spawn((
			Sprite {
				image: enemy_handle.clone(),
				custom_size: Some(enemy_size),
				..Default::default()
			},
			Transform::from_xyz(x_pos, bounds.extents.y, 0.0),
			Enemy,
			ShipSize(enemy_size),
		));
	}
}

pub fn enemy_movement(
	time: Res<Time>,
	mut enemy_query: Query<(&mut Transform, &MaxSpeed), (With<Enemy>, Without<Player>)>,
	player_query: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
	let player_translation = player_query.translation;

	for (mut enemy_transform, enemy_max_velocity) in &mut enemy_query {
		let enemy_translation = enemy_transform.translation;
		let to_player = (player_translation - enemy_translation).normalize();
		let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player);
		enemy_transform.rotation = rotate_to_player;

		let movement_direction = enemy_transform.rotation.mul_vec3(Vec3::Y);
		let movement_distance = enemy_max_velocity.get().algebraic_mul(time.delta_secs());

		enemy_transform.translation += movement_direction * movement_distance;
	}
}

pub fn spawn_ammo(
	mut commands: Commands,
	assets_server: Res<AssetServer>,
	enemy_query: Query<(&Transform, &ShipSize, &AmmoSize, &mut WeaponCooldown), With<Enemy>>,
) {
	let ammo_handle = assets_server.load(AMMO_PATH);
	for (enemy_transform, enemy_size, ammo_size, mut weapon_cooldown) in enemy_query {
		if weapon_cooldown.finished() {
			weapon_cooldown.reset();
			let enemy_rotation = enemy_transform.rotation.mul_vec3(Vec3::Y);

			let spawn_offset_distance = enemy_size
				.get_y()
				.algebraic_div(2.0)
				.algebraic_add(ammo_size.get_y().algebraic_div(2.0));
			let spawn_offset = enemy_rotation * spawn_offset_distance;

			let spawn_position = enemy_transform.translation + spawn_offset;

			commands.spawn((
				Sprite {
					image: ammo_handle.clone(),
					custom_size: Some(ammo_size.value()),
					..Default::default()
				},
				Transform::from_translation(spawn_position),
				EnemyAmmo,
				AmmoDirection(enemy_rotation),
			));
		}
	}
}
