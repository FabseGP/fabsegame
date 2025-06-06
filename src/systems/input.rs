use bevy::prelude::*;

use crate::{
	components::player::{Player, Rotation, Velocity},
	constants::{SPACESHIP_ROTATION_STEP, SPACESHIP_SPEED_STEP},
};

pub fn input_events(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	gamepads: Query<&Gamepad>,
	mut ship: Single<(&mut Velocity, &mut Rotation), With<Player>>,
) {
	if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
		ship.0.0.y = SPACESHIP_SPEED_STEP;
	} else if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
		ship.0.0.y = -SPACESHIP_SPEED_STEP;
	}

	if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
		ship.1.0 = SPACESHIP_ROTATION_STEP;
	} else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
		ship.1.0 = -SPACESHIP_ROTATION_STEP;
	}

	for gamepad in gamepads {
		if gamepad.pressed(GamepadButton::DPadUp) {
			ship.0.0.y = SPACESHIP_SPEED_STEP;
		} else if gamepad.pressed(GamepadButton::DPadDown) {
			ship.0.0.y = -SPACESHIP_SPEED_STEP;
		}
		if gamepad.pressed(GamepadButton::DPadLeft) {
			ship.1.0 = SPACESHIP_ROTATION_STEP;
		} else if gamepad.pressed(GamepadButton::DPadRight) {
			ship.1.0 = -SPACESHIP_ROTATION_STEP;
		}

		if let Some(left_stick_y) = gamepad.get(GamepadAxis::LeftStickY)
			&& left_stick_y.abs() > 0.1
		{
			ship.0.0.y = left_stick_y.algebraic_mul(SPACESHIP_SPEED_STEP);
		}

		if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX)
			&& left_stick_x.abs() > 0.1
		{
			ship.1.0 = -left_stick_x.algebraic_mul(SPACESHIP_ROTATION_STEP);
		}
	}
}
