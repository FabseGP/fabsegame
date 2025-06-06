use bevy::prelude::*;

use crate::{
	components::player::{Player, Position, Rotation, Velocity},
	constants::{
		BOUNDS, SPACESHIP_ROTATION_BOUND, SPACESHIP_ROTATION_FACTOR, SPACESHIP_SPEED_FACTOR,
	},
};

pub fn player_movement_system(
	time: Res<Time>,
	mut movement: Query<
		(
			&mut Transform,
			(&mut Velocity, &mut Rotation, &mut Position),
		),
		With<Player>,
	>,
) {
	for (mut transform, (mut velocity, mut rotation, mut position)) in &mut movement {
		transform.rotate_z(
			rotation
				.0
				.algebraic_mul(SPACESHIP_ROTATION_FACTOR)
				.algebraic_mul(time.delta_secs())
				.clamp(-SPACESHIP_ROTATION_BOUND, SPACESHIP_ROTATION_BOUND),
		);

		let movement_direction = transform.rotation.mul_vec3(Vec3::Y);

		let movement_distance = velocity
			.0
			.y
			.algebraic_mul(SPACESHIP_SPEED_FACTOR)
			.algebraic_mul(time.delta_secs());

		transform.translation += movement_direction * movement_distance;

		let extents = Vec3::from((BOUNDS / 2.0, 0.0));
		let translation_bounded = transform.translation.min(extents).max(-extents);

		transform.translation = translation_bounded;
		position.0 = translation_bounded;

		velocity.0.y = 0.0;
		rotation.0 = 0.0;
	}
}
