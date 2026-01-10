use bevy::prelude::*;

use crate::{
	components::{
		common::{MovementRotation, MovementSpeed, WeaponCooldown},
		player::{Player, PlayerFireWeapon},
	},
	constants::{CONTROLLER_DEADZONE, SPACESHIP_ROTATION_INCREMENT, SPACESHIP_SPEED_INCREMENT},
	traits::F32Component as _,
};

pub fn input_events(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	gamepad_query_opt: Option<Single<&Gamepad>>,
	player_query: Single<
		(
			&mut MovementSpeed,
			&mut MovementRotation,
			&mut WeaponCooldown,
		),
		With<Player>,
	>,
	mut weapon_event: MessageWriter<PlayerFireWeapon>,
	mut exit: MessageWriter<AppExit>,
) {
	let (mut player_velocity, mut player_rotation, mut player_weapon_cooldown) =
		player_query.into_inner();

	if keyboard_input.pressed(KeyCode::Escape) {
		exit.write(AppExit::Success);
	}

	if keyboard_input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) {
		player_velocity.set(SPACESHIP_SPEED_INCREMENT);
	} else if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) {
		player_velocity.set(-SPACESHIP_SPEED_INCREMENT);
	}

	if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
		player_rotation.set(SPACESHIP_ROTATION_INCREMENT);
	} else if keyboard_input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
		player_rotation.set(-SPACESHIP_ROTATION_INCREMENT);
	}

	if keyboard_input.pressed(KeyCode::Space) && player_weapon_cooldown.finished() {
		weapon_event.write_default();
		player_weapon_cooldown.reset();
	}

	if let Some(gamepad) = gamepad_query_opt {
		if let Some(left_stick_y) = gamepad.get(GamepadAxis::LeftStickY)
			&& left_stick_y.abs() > CONTROLLER_DEADZONE
		{
			player_velocity.set(left_stick_y.algebraic_mul(SPACESHIP_SPEED_INCREMENT));
		} else if gamepad.pressed(GamepadButton::DPadUp) {
			player_velocity.set(SPACESHIP_SPEED_INCREMENT);
		} else if gamepad.pressed(GamepadButton::DPadDown) {
			player_velocity.set(-SPACESHIP_SPEED_INCREMENT);
		}

		if let Some(left_stick_x) = gamepad.get(GamepadAxis::LeftStickX)
			&& left_stick_x.abs() > CONTROLLER_DEADZONE
		{
			player_rotation.set(-left_stick_x.algebraic_mul(SPACESHIP_ROTATION_INCREMENT));
		} else if gamepad.pressed(GamepadButton::DPadLeft) {
			player_rotation.set(SPACESHIP_ROTATION_INCREMENT);
		} else if gamepad.pressed(GamepadButton::DPadRight) {
			player_rotation.set(-SPACESHIP_ROTATION_INCREMENT);
		}

		if gamepad.any_pressed([
			GamepadButton::South,
			GamepadButton::LeftTrigger2,
			GamepadButton::RightTrigger2,
		]) && player_weapon_cooldown.finished()
		{
			weapon_event.write_default();
			player_weapon_cooldown.reset();
		}
	}
}
