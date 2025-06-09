use bevy::prelude::*;

use crate::{
	components::{
		common::{AmmoSize, ShipSize},
		enemy::{Enemy, EnemyAmmo, EnemyFireWeapon},
		player::Player,
	},
	constants::{AMMO_PATH, BOUNDS, BOUNDS_EXTENTS, ENEMY_COUNT, ENEMY_FILENAME, ENEMY_SIZE_MAX},
	traits::Vec2Component as _,
};

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
	let enemy_handle = asset_server.load(ENEMY_FILENAME);

	let enemy_size = vec2(
		ENEMY_SIZE_MAX.algebraic_div(f32::from(ENEMY_COUNT)),
		ENEMY_SIZE_MAX.algebraic_div(f32::from(ENEMY_COUNT)),
	);

	let y_pos = BOUNDS.y.algebraic_div(2.0);

	for i in 0..ENEMY_COUNT {
		let x_pos = BOUNDS_EXTENTS.x.algebraic_sub(f32::max(
			enemy_size.x.algebraic_mul(f32::from(i)),
			BOUNDS_EXTENTS
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
			Transform::from_xyz(x_pos, y_pos, 0.0),
			Enemy,
		));
	}
}

pub fn enemy_movement(
	mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
	player_query: Single<&Transform, (With<Player>, Without<Enemy>)>,
) {
	let player_translation = player_query.translation;

	for mut enemy_transform in &mut enemy_query {
		let enemy_translation = enemy_transform.translation;
		let to_player = (player_translation - enemy_translation).normalize();
		let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player);
		enemy_transform.rotation = rotate_to_player;
	}
}

pub fn spawn_ammo(
	mut commands: Commands,
	mut ammo_event: EventReader<EnemyFireWeapon>,
	assets_server: Res<AssetServer>,
	enemy_query: Query<(&Transform, &ShipSize, &AmmoSize), With<Enemy>>,
) {
	let ammo_handle = assets_server.load(AMMO_PATH);
	for _event in ammo_event.read() {
		for (enemy_transform, enemy_size, ammo_size) in enemy_query {
			let enemy_rotation = enemy_transform.rotation.mul_vec3(Vec3::Y);

			let spawn_offset_distance = enemy_size
				.get_y()
				.algebraic_div(2.0)
				.algebraic_add(ammo_size.0.y.algebraic_div(2.0));
			let spawn_offset = enemy_rotation * spawn_offset_distance;

			let spawn_position = enemy_transform.translation + spawn_offset;

			commands.spawn((
				Sprite {
					image: ammo_handle.clone(),
					custom_size: Some(ammo_size.0),
					..Default::default()
				},
				Transform::from_translation(spawn_position),
				EnemyAmmo,
			));
		}
	}
}
