use bevy::prelude::*;

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
pub struct Velocity(pub Vec2);

#[derive(Component, Default)]
pub struct Rotation(pub f32);
