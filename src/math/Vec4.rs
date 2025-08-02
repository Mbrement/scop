use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Vec4 {
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
		Self { x, y, z, w }
	}

	pub fn from_array(arr: &[f32; 4]) -> Self {
		Self {
			x: arr[0],
			y: arr[1],
			z: arr[2],
			w: arr[3],
		}
	}

	pub fn zero() -> Self {
		Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
	}

	pub fn dot(self, other: Self) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
	}

	pub fn length(self) -> f32 {
		self.dot().sqrt()
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
			z: self.z + other.z,
			w: self.w + other.w,
		}
	}

	pub fn add_self(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
		self.w += other.w;
	}

	pub fn sub(self, other: Self) -> Self {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
			w: self.w - other.w,
		}
	}

	pub fn sub_self(&mut self, other: Self) {
		self.x -= other.x;
		self.y -= other.y;
		self.z -= other.z;
		self.w -= other.w;
	}

	pub fn mul(self, other: Self) -> Self {
		Self {
			x: self.x * other.x,
			y: self.y * other.y,
			z: self.z * other.z,
			w: self.w * other.w,
		}
	}

	pub fn mul_self(&mut self, other: Self) {
		self.x *= other.x;
		self.y *= other.y;
		self.z *= other.z;
		self.w *= other.w;
	}

	pub fn mul_f32(self, other: f32) -> Self {
		Self {
			x: self.x * other,
			y: self.y * other,
			z: self.z * other,
			w: self.w * other,
		}
	}

	pub fn mul_f32_self(&mut self, other: f32) {
		self.x *= other;
		self.y *= other;
		self.z *= other;
		self.w *= other;
	}

	pub fn div(self, other: Self) -> Self {
		Self {
			x: self.x / other.x,
			y: self.y / other.y,
			z: self.z / other.z,
			w: self.w / other.w,
		}
	}

	pub fn div_self(&mut self, other: Self) {
		self.x /= other.x;
		self.y /= other.y;
		self.z /= other.z;
		self.w /= other.w;
	}

	pub fn div_f32(self, other: f32) -> Self {
		if other != 0.0 {
			Self {
				x: self.x / other,
				y: self.y / other,
				z: self.z / other,
				w: self.w / other,
			}
		} else {
			Self::zero()
		}
	}

	pub fn div_f32_self(&mut self, other: f32) {
		if other != 0.0 {
			self.x /= other;
			self.y /= other;
			self.z /= other;
			self.w /= other;
		} else {
			*self = Self::zero();
		}
	}
	
	pub fn display(&self){
		println!("Vec4: x: {}, y: {}, z: {}, w: {}", self.x, self.y, self.z, self.w);
	}

}