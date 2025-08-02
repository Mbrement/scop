pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3{
	pub fn new() -> Self {
		Vec3 { x: 0.0, y: 0.0, z: 0.0 }
	}

	pub fn from_array(arr: &[f32; 3]) -> Self {
		Vec3 {
			x: arr[0],
			y: arr[1],
			z: arr[2],
		}
	}

	pub fn to_array(&self) -> [f32; 3] {
		[self.x, self.y, self.z]
	}

	pub fn add(&self, other: &Vec3) -> Self {
		Vec3 {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}

	pub fn subtract(&self, other: &Vec3) -> Self {
		Vec3 {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}


	pub fn add_self(&mut self, other: &Vec3) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
	}

	pub fn sub_self(&mut self, other: &Vec3){
		self.x -= other.x;
		self.y -= other.y;
		self.z -= other.z;
	}

	pub fn mul(self, other: &Vec3) -> Self {
		Vec3 {
			x: self.x * other.x,
			y: self.y * other.y,
			z: self.z * other.z,
		}	
	}

	pub fn mul_self(&mut self, other: &Vec3) {
		self.x *= other.x;
		self.y *= other.y;
		self.z *= other.z;
	}

	pub fn div(self, other: &Vec3) -> Self {
		Vec3 {
			x: self.x / other.x,
			y: self.y / other.y,
			z: self.z / other.z,
		}
	}

	pub fn div_self(&mut self, other: &Vec3) {
		self.x /= other.x;
		self.y /= other.y;
		self.z /= other.z;
	}

	pub fn dot(&self, other: &Vec3) -> f32 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}

	pub fn length(&self) -> f32 {
		self.dot(self).sqrt()
	}

	pub fn mul_f32(&self, other: f32) -> Self {
		Vec3 {
			x: self.x * other,
			y: self.y * other,
			z: self.z * other,
		}
	}

	pub fn mul_f32_self(&mut self, other: f32) {
		self.x *= other;
		self.y *= other;
		self.z *= other;
	}

	pub fn div_f32(&self, other: f32) -> Self {
		if other != 0.0 {
			Vec3 {
				x: self.x / other,
				y: self.y / other,
				z: self.z / other,
			}
		} else {
			Vec3::new()
		}
	}

	pub fn div_f32_self(&mut self, other: f32) {
		if other != 0.0 {
			self.x /= other;
			self.y /= other;
			self.z /= other;
		}
		else {
			self.x = 0.0;
			self.y = 0.0;
			self.z = 0.0;
		}
	}

	pub fn display (&self) {
		println!("Vec3: x: {}, y: {}, z: {}", self.x, self.y, self.z);
	}
}