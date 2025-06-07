use bevy::prelude::*;

use crate::{
	components::player::{Enemy, MovementData, Player},
	constants::{
		BOUNDS, SPACESHIP_ROTATION_BOUND, SPACESHIP_ROTATION_FACTOR, SPACESHIP_SPEED_FACTOR,
	},
};

pub fn player_movement_system(
	time: Res<Time>,
	mut movement: Single<MovementData, (With<Player>, Without<Enemy>)>,
) {
	let rotation_value = movement
		.rotation
		.0
		.algebraic_mul(SPACESHIP_ROTATION_FACTOR)
		.algebraic_mul(time.delta_secs())
		.clamp(-SPACESHIP_ROTATION_BOUND, SPACESHIP_ROTATION_BOUND);

	movement.transform.rotate_z(rotation_value);

	movement.rotation.0 = 0.;

	let movement_direction = movement.transform.rotation.mul_vec3(Vec3::Y);

	let movement_distance = movement
		.velocity
		.0
		.algebraic_mul(SPACESHIP_SPEED_FACTOR)
		.algebraic_mul(time.delta_secs());

	movement.transform.translation += movement_direction * movement_distance;

	let extents = Vec3::from((BOUNDS / 2.0, 0.0));
	let translation_bounded = movement.transform.translation.min(extents).max(-extents);

	movement.transform.translation = translation_bounded;
	movement.position.0 = translation_bounded;

	movement.velocity.0 = 0.;
}
