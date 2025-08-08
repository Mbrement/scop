// use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    //	This is a constant vector with all components set to zero
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    // This is a vector with X component set to 1.0 and others to 0.0
    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);

    // This is a vector with Y component set to 1.0 and others to 0.0
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);

    // This is a vector with Z component set to 1.0 and others to 0.0
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);

    // This is a vector with W component set to 1.0 and others to 0.0
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    // This is a vector with X component set to -1.0 and others to 0.0
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0, 0.0);

    // This is a vector with Y component set to -1.0 and others to 0.0
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0, 0.0);

    // This is a vector with Z component set to -1.0 and others to 0.0
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0, 0.0);

    // This is a vector with W component set to -1.0 and others to 0.0
    pub const NEG_W: Self = Self::new(0.0, 0.0, 0.0, -1.0);

    /// The unit axes.
    pub const AXES: [Self; 4] = [Self::X, Self::Y, Self::Z, Self::W];

    pub fn from_translation(vec3: Vec3) -> Self {
        Self {
            x: vec3.x,
            y: vec3.y,
            z: vec3.z,
            w: 1.0,
        }
    }

    #[inline]
    pub fn from_array(arr: &[f32; 4]) -> Self {
        Self {
            x: arr[0],
            y: arr[1],
            z: arr[2],
            w: arr[3],
        }
    }
    #[inline]
    pub const fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len != 0.0 {
            self.div_f32(len)
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

    pub fn display(&self) {
        println!(
            "Vec4: x: {}, y: {}, z: {}, w: {}",
            self.x, self.y, self.z, self.w
        );
    }
}
