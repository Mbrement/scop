use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}

impl Vec2 {
	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}

	pub fn zero() -> Self {
		Self { x: 0.0, y: 0.0 }
	}

	pub fn dot(self, other: Self) -> f32 {
		self.x * other.x + self.y * other.y
	}

	pub fn length(self) -> f32 {
		self.dot(self).sqrt()
	}

	pub fn normalize(self) -> Self {
		let len = self.length();
		if len != 0.0 {
			self / len
		} else {
			Self::zero()
		}
	}

	pub fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
	pub fn add_self(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
}

	pub fn sub(self, other: Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
		}
	}

	pub fn sub_self(&mut self, other: Self) {
		self.x -= other.x;
		self.y -= other.y;
	}

	pub fn mul(self, other: Vec2) -> Self {
		Self {
			x: self.x * other.x,
			y: self.y * other.y,
		}
	}
	
	pub fn mul_self(&mut self, other: Vec2) {
		self.x *= other.x;
		self.y *= other.y;
	}

	pub fn mul_f32(self, other: f32) -> Self {
		Self {
			x: self.x * other,
			y: self.y * other,
		}
	}

	pub fn mul_f32_self(self, other :f32){
		self.x *= other;
		self.y *= other;
	}

	pub fn div(self, other: Vec2) -> Self {
		Self {
			x: self.x / other.x,
			y: self.y / other.y,
		}
	}

	pub fn div_self(&mut self, other: Vec2) {
		self.x /= other.x;
		self.y /= other.y;
	}

	pub fn div_f32(self, other: f32) -> Self {
		if other != 0.0 {
		
			Self {
				x: self.x / other,
				y: self.y / other,
			}
		}
		else {
			Self::zero()
		}

		pub fn div_f32_self(&mut self, other: f32) {
			if other != 0.0 {
				self.x /= other;
				self.y /= other;
			} else {
				*self = Self::zero();
			}
		}
	}
}
