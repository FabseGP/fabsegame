#![feature(float_algebraic)]

mod components;
mod constants;
mod systems;
mod window;

use bevy::prelude::*;
use components::player::{BoundsCollisionEvent, Enemy, Player, Position};
use constants::{
	BOUNDS, ENEMY_COUNT, ENEMY_FILENAME, ENEMY_SIZE_MAX, SPACESHIP_FILENAME, SPACESHIP_POSITION,
	SPACESHIP_SIZE,
};
use systems::{
	collisions::{bounds_collision_detection_system, bounds_collision_rumble_system},
	enemy::snap_to_player_system,
	input::input_events,
	player::player_movement_system,
};
use window::create_window;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(create_window()))
		.add_event::<BoundsCollisionEvent>()
		.add_systems(Startup, setup)
		.add_systems(Update, input_events)
		.add_systems(
			FixedUpdate,
			(
				player_movement_system,
				snap_to_player_system,
				bounds_collision_detection_system,
				bounds_collision_rumble_system,
			),
		)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	let ship_handle = asset_server.load(SPACESHIP_FILENAME);
	let enemy_handle = asset_server.load(ENEMY_FILENAME);

	commands.spawn(Camera2d);

	commands.spawn((
		Text::new("Up Arrow: Move Forward\nLeft / Right Arrow: Turn"),
		Node {
			position_type: PositionType::Absolute,
			top: Val::Px(12.0),
			left: Val::Px(12.0),
			..default()
		},
	));

	commands.spawn((
		Sprite {
			image: ship_handle,
			custom_size: Some(SPACESHIP_SIZE),
			..Default::default()
		},
		Transform::from_xyz(SPACESHIP_POSITION.x, SPACESHIP_POSITION.y, 0.0),
		Player,
		Position(SPACESHIP_POSITION.extend(0.0)),
	));

	let horizontal_margin = BOUNDS.x.algebraic_div(2.0);
	let vertical_margin = BOUNDS.y.algebraic_div(2.0);
	let enemy_size = vec2(
		ENEMY_SIZE_MAX.algebraic_div(f32::from(ENEMY_COUNT)),
		ENEMY_SIZE_MAX.algebraic_div(f32::from(ENEMY_COUNT)),
	);

	for i in 0..ENEMY_COUNT {
		commands.spawn((
			Sprite {
				image: enemy_handle.clone(),
				custom_size: Some(enemy_size),
				..Default::default()
			},
			Transform::from_xyz(
				horizontal_margin.algebraic_sub(f32::max(
					enemy_size.y.algebraic_mul(f32::from(i)),
					horizontal_margin
						.algebraic_div(f32::from(ENEMY_COUNT))
						.algebraic_mul(f32::from(i))
						.algebraic_mul(2.0),
				)),
				vertical_margin,
				0.0,
			),
			Enemy,
		));
	}
}
