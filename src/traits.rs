use bevy::prelude::*;

pub trait F32Component {
	fn value(&self) -> f32;
	fn value_mut(&mut self) -> &mut f32;

	fn add(&mut self, increment: f32) {
		*self.value_mut() = self.value().algebraic_add(increment);
	}

	fn get(&self) -> f32 {
		self.value()
	}

	fn multiply(&mut self, factor: f32) {
		*self.value_mut() = self.value().algebraic_mul(factor);
	}

	fn reset(&mut self) {
		*self.value_mut() = 0.0;
	}

	fn set(&mut self, value: f32) {
		*self.value_mut() = value;
	}

	fn sub(&mut self, decrement: f32) {
		*self.value_mut() = self.value().algebraic_sub(decrement);
	}
}

pub trait Vec2Component {
	fn value(&self) -> Vec2;

	fn get_x(&self) -> f32 {
		self.value().x
	}

	fn get_y(&self) -> f32 {
		self.value().y
	}

	fn length(&self) -> f32 {
		self.value().length()
	}

	fn normalize(&self) -> Vec2 {
		self.value().normalize_or_zero()
	}
}

pub trait Vec3Component {
	fn value(&self) -> Vec3;

	fn get_x(&self) -> f32 {
		self.value().x
	}

	fn get_y(&self) -> f32 {
		self.value().y
	}

	fn length(&self) -> f32 {
		self.value().length()
	}

	fn normalize(&self) -> Vec3 {
		self.value().normalize_or_zero()
	}
}
