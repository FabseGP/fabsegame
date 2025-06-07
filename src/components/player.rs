use bevy::{ecs::query::QueryData, prelude::*};

#[derive(Component)]
#[require(Position, Rotation, Velocity)]
pub struct Player;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(pub Vec3);

#[derive(Component)]
#[require(Position, Velocity)]
pub struct Enemy;

#[derive(Component, Default)]
pub struct Velocity(pub f32);

#[derive(Component, Default)]
pub struct Rotation(pub f32);

#[derive(QueryData)]
#[query_data(mutable)]
pub struct MovementData {
	pub transform: &'static mut Transform,
	pub velocity: &'static mut Velocity,
	pub rotation: &'static mut Rotation,
	pub position: &'static mut Position,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub struct InputData {
	pub velocity: &'static mut Velocity,
	pub rotation: &'static mut Rotation,
}

#[derive(Default, Event)]
pub struct BoundsCollisionEvent;
