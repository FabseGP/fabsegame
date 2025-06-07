use bevy::prelude::*;

use crate::{
	components::player::{InputData, Player},
	constants::{CONTROLLER_DEADZONE, SPACESHIP_ROTATION_STEP, SPACESHIP_SPEED_STEP},
};

pub fn input_events(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	gamepads: Query<&Gamepad>,
	mut ship: Single<InputData, With<Player>>,
) {
	if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
		ship.velocity.0 = SPACESHIP_SPEED_STEP;
	} else if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
		ship.velocity.0 = -SPACESHIP_SPEED_STEP;
	}

	if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
		ship.rotation.0 = SPACESHIP_ROTATION_STEP;
	} else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
		ship.rotation.0 = -SPACESHIP_ROTATION_STEP;
	}

	for gamepad in gamepads {
		if gamepad.pressed(GamepadButton::DPadUp) {
			ship.velocity.0 = SPACESHIP_SPEED_STEP;
		} else if gamepad.pressed(GamepadButton::DPadDown) {
			ship.velocity.0 = -SPACESHIP_SPEED_STEP;
		}
		if gamepad.pressed(GamepadButton::DPadLeft) {
			ship.rotation.0 = SPACESHIP_ROTATION_STEP;
		} else if gamepad.pressed(GamepadButton::DPadRight) {
			ship.rotation.0 = -SPACESHIP_ROTATION_STEP;
		}

		if let Some(left_stick_y) = gamepad.get(GamepadAxis::LeftStickY)
			&& left_stick_y.abs() > CONTROLLER_DEADZONE
		{
			ship.velocity.0 = left_stick_y
				.algebraic_mul(SPACESHIP_SPEED_STEP)
				.algebraic_mul(2.0);
		}
		if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX)
			&& left_stick_x.abs() > CONTROLLER_DEADZONE
		{
			ship.rotation.0 = -left_stick_x
				.algebraic_mul(SPACESHIP_ROTATION_STEP)
				.algebraic_mul(2.0);
		}
	}
}
