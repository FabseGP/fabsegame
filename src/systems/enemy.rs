use bevy::prelude::*;

use crate::components::player::{Enemy, Player};

pub fn snap_to_player_system(
	mut query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
	player_transform: Single<&mut Transform, With<Player>>,
) {
	let player_translation = player_transform.translation.xy();

	for mut enemy_transform in &mut query {
		let enemy_translation = enemy_transform.translation.xy();
		let to_player = (player_translation - enemy_translation).normalize();
		let rotate_to_player = Quat::from_rotation_arc(Vec3::Y, to_player.extend(0.));
		enemy_transform.rotation = rotate_to_player;

		if player_translation == enemy_translation {
			info!("IMPACT!!!! \n");
		}
	}
}
