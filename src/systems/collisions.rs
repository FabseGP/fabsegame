use std::time::Duration;

use bevy::{
	input::gamepad::{GamepadRumbleIntensity, GamepadRumbleRequest},
	prelude::*,
};

use crate::{
	components::player::{BoundsCollisionEvent, Player, Position},
	constants::{BOUNDS, CONTROLLER_RUMBLE_DURATION_MS},
};

pub fn bounds_collision_detection_system(
	mut collision_events: EventWriter<BoundsCollisionEvent>,
	mut query: Query<(&Transform, Option<&mut Position>), With<Player>>,
) {
	let extents = BOUNDS / 2.0;

	for (transform, mut previous_pos) in &mut query {
		let current_pos = transform.translation;

		let prev_pos = match previous_pos.as_mut() {
			Some(prev) => {
				let old_pos = prev.0;
				prev.0 = current_pos;
				old_pos
			}
			None => {
				continue;
			}
		};

		let hit_x_boundary = (prev_pos.x <= extents.x && prev_pos.x >= -extents.x)
			&& current_pos.x.abs() == extents.x;
		let hit_y_boundary = (prev_pos.y <= extents.y && prev_pos.y >= -extents.y)
			&& current_pos.y.abs() == extents.y;

		if hit_x_boundary || hit_y_boundary {
			collision_events.write_default();
		}
	}
}

pub fn bounds_collision_rumble_system(
	mut collision_events: EventReader<BoundsCollisionEvent>,
	gamepads: Query<Entity, With<Gamepad>>,
	mut rumble_requests: EventWriter<GamepadRumbleRequest>,
) {
	for _event in collision_events.read() {
		for gamepad_entity in &gamepads {
			rumble_requests.write(GamepadRumbleRequest::Add {
				duration: Duration::from_millis(CONTROLLER_RUMBLE_DURATION_MS),
				intensity: GamepadRumbleIntensity::MAX,
				gamepad: gamepad_entity,
			});
		}
	}
}
