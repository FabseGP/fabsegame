use std::time::Duration;

use bevy::prelude::*;

use crate::traits::{F32Component, Vec2Component, Vec3Component};

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct MovementSpeed(pub f32);

impl F32Component for MovementSpeed {
	fn value(&self) -> f32 {
		self.0
	}

	fn value_mut(&mut self) -> &mut f32 {
		&mut self.0
	}
}

#[derive(Component)]
pub struct MaxSpeed(pub f32);

impl F32Component for MaxSpeed {
	fn value(&self) -> f32 {
		self.0
	}

	fn value_mut(&mut self) -> &mut f32 {
		&mut self.0
	}
}

#[derive(Component)]
pub struct AmmoSpeed(pub f32);

impl F32Component for AmmoSpeed {
	fn value(&self) -> f32 {
		self.0
	}

	fn value_mut(&mut self) -> &mut f32 {
		&mut self.0
	}
}

#[derive(Component)]
pub struct AmmoRange(pub f32);

impl F32Component for AmmoRange {
	fn value(&self) -> f32 {
		self.0
	}

	fn value_mut(&mut self) -> &mut f32 {
		&mut self.0
	}
}

#[derive(Component)]
pub struct AmmoDistanceTraveled(pub f32);

impl F32Component for AmmoDistanceTraveled {
	fn value(&self) -> f32 {
		self.0
	}

	fn value_mut(&mut self) -> &mut f32 {
		&mut self.0
	}
}

#[derive(Component)]
pub struct MovementRotation(pub f32);

impl F32Component for MovementRotation {
	fn value(&self) -> f32 {
		self.0
	}

	fn value_mut(&mut self) -> &mut f32 {
		&mut self.0
	}
}

#[derive(Component)]
pub struct MaxRotation(pub f32);

impl F32Component for MaxRotation {
	fn value(&self) -> f32 {
		self.0
	}

	fn value_mut(&mut self) -> &mut f32 {
		&mut self.0
	}
}

#[derive(Component)]
pub struct ShipHealth(pub f32);

impl F32Component for ShipHealth {
	fn value(&self) -> f32 {
		self.0
	}

	fn value_mut(&mut self) -> &mut f32 {
		&mut self.0
	}
}

#[derive(Component)]
pub struct AmmoDamage(pub f32);

impl F32Component for AmmoDamage {
	fn value(&self) -> f32 {
		self.0
	}

	fn value_mut(&mut self) -> &mut f32 {
		&mut self.0
	}
}

#[derive(Component)]
pub struct BoundsDamage(pub f32);

impl F32Component for BoundsDamage {
	fn value(&self) -> f32 {
		self.0
	}

	fn value_mut(&mut self) -> &mut f32 {
		&mut self.0
	}
}

#[derive(Component, Default)]
pub struct ShipSize(pub Vec2);

impl Vec2Component for ShipSize {
	fn value(&self) -> Vec2 {
		self.0
	}
}

#[derive(Component)]
pub struct AmmoSize(pub Vec2);

impl Vec2Component for AmmoSize {
	fn value(&self) -> Vec2 {
		self.0
	}
}

#[derive(Component)]
pub struct AmmoDirection(pub Vec3);

impl Vec3Component for AmmoDirection {
	fn value(&self) -> Vec3 {
		self.0
	}
}

#[derive(Component, Default)]
pub struct WeaponCooldown(pub Timer);

impl WeaponCooldown {
	pub fn new(duration: f32) -> Self {
		Self(Timer::from_seconds(duration, TimerMode::Once))
	}

	pub fn tick(&mut self, delta: Duration) {
		self.0.tick(delta);
	}

	pub fn finished(&self) -> bool {
		self.0.is_finished()
	}

	pub fn reset(&mut self) {
		self.0.reset();
	}
}

#[derive(Resource)]
pub struct GameBounds {
	pub bounds: Vec2,
	pub extents: Vec2,
	pub margin: f32,
	pub minimum_impact: f32,
}

impl GameBounds {
	pub fn new(window_size: Vec2, margin: f32, minimum_impact: f32) -> Self {
		let bounds = window_size - Vec2::splat(margin.algebraic_mul(2.0));
		let extents = bounds / 2.0;

		Self {
			bounds,
			extents,
			margin,
			minimum_impact,
		}
	}

	pub fn update(&mut self, window_size: Vec2) {
		self.bounds = window_size - Vec2::splat(self.margin.algebraic_mul(2.0));
		self.extents = self.bounds / 2.0;
	}
}
