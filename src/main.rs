#![feature(float_algebraic)]

mod components;
mod constants;
mod systems;
mod traits;
mod window;

use bevy::{
	asset::embedded_asset,
	prelude::*,
	window::{PrimaryWindow, WindowResized},
};
use components::{
	common::{GameBounds, HealthBar, WeaponCooldown},
	enemy::EnemyWeaponImpact,
	player::{PlayerBoundsImpact, PlayerFireWeapon, PlayerWeaponImpact},
};
use constants::{INFO_TEXT, MUSIC_FILENAME, WINDOW_BOUNDS, WINDOW_BOUNDS_MINIMUM_IMPACT};
use systems::{
	collisions::{
		ammo_collision, ammo_translation, bounds_collision, bounds_impact, weapon_impact,
	},
	enemy::{enemy_movement, spawn_ammo, spawn_enemy},
	input::input_events,
	player::{player_ammo, player_health, player_movement, spawn_player, update_health_bar},
};
use window::create_window;

struct EmbeddedAssetPlugin;

impl Plugin for EmbeddedAssetPlugin {
	fn build(&self, app: &mut App) {
		embedded_asset!(app, "assets/ammo.webp");
	}
}

fn spawn_camera(mut commands: Commands) {
	commands.spawn(Camera2d);
}

fn spawn_info_text(mut commands: Commands) {
	commands.spawn((
		Text::new(INFO_TEXT),
		Node {
			position_type: PositionType::Absolute,
			top: Val::Px(12.0),
			left: Val::Px(12.0),
			..default()
		},
	));
}

fn spawn_music(mut commands: Commands, assets_server: Res<AssetServer>) {
	commands.spawn(AudioPlayer::new(assets_server.load(MUSIC_FILENAME)));
}

fn spawn_health_bar(mut commands: Commands) {
	commands.spawn((
		Sprite {
			color: Color::srgb(0.2, 0.8, 0.2),
			custom_size: Some(Vec2::new(200.0, 20.0)),
			..Default::default()
		},
		Transform::from_xyz(0.0, 0.0, 0.1),
		HealthBar,
	));
}

fn tick_cooldowns(mut cooldowns_query: Query<&mut WeaponCooldown>, time: Res<Time>) {
	for mut cooldown in &mut cooldowns_query {
		cooldown.tick(time.delta());
	}
}

fn setup_bounds(mut commands: Commands, window_query: Single<&Window, With<PrimaryWindow>>) {
	let window_size = Vec2::new(window_query.width(), window_query.height());
	commands.insert_resource(GameBounds::new(
		window_size,
		WINDOW_BOUNDS,
		WINDOW_BOUNDS_MINIMUM_IMPACT,
	));
}

fn update_bounds_on_resize(
	mut bounds: ResMut<GameBounds>,
	mut resize_events: EventReader<WindowResized>,
) {
	for event in resize_events.read() {
		let window_size = Vec2::new(event.width, event.height);
		bounds.update(window_size);
	}
}

fn main() {
	App::new()
		.add_plugins((DefaultPlugins.set(create_window()), EmbeddedAssetPlugin))
		.add_event::<PlayerBoundsImpact>()
		.add_event::<PlayerFireWeapon>()
		.add_event::<PlayerWeaponImpact>()
		.add_event::<EnemyWeaponImpact>()
		.add_systems(
			Startup,
			(
				setup_bounds,
				spawn_camera,
				spawn_info_text,
				spawn_music,
				spawn_player,
				spawn_enemy,
				spawn_health_bar,
			)
				.chain(),
		)
		.add_systems(
			Update,
			(
				input_events,
				bounds_impact,
				weapon_impact,
				player_health,
				update_health_bar,
				tick_cooldowns,
				update_bounds_on_resize,
			),
		)
		.add_systems(
			FixedUpdate,
			(
				player_movement,
				enemy_movement,
				bounds_collision,
				ammo_collision,
				spawn_ammo,
				player_ammo,
				ammo_translation,
			),
		)
		.run();
}
