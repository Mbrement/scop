use crate::Vec3;
use crate::Vec4;

pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}

impl Mat4 {
    pub fn new() -> Self {
        Mat4 {
            x_axis: Vec4::zero(),
            y_axis: Vec4::zero(),
            z_axis: Vec4::zero(),
            w_axis: Vec4::zero(),
        }
    }

    // pub fn from_cols_array(arr: &[f32; 16]) -> Self {
    //     let mut mat = Mat4::new();
    //     for i in 0..4 {
    //         for j in 0..4 {
    //             mat.0[i][j] = arr[i * 4 + j];
    //         }
    //     }
    //     mat
    // }

    pub fn identity() -> Self {
        Self {
            x_axis: Vec4::from_array(&[1.0, 0.0, 0.0, 0.0]),
            y_axis: Vec4::from_array(&[0.0, 1.0, 0.0, 0.0]),
            z_axis: Vec4::from_array(&[0.0, 0.0, 1.0, 0.0]),
            w_axis: Vec4::from_array(&[0.0, 0.0, 0.0, 1.0]),
        }
    }

    pub fn to_cols_array(&self) -> [f32; 16] {
        let mut arr = [0.0; 16];
        arr[0..4].copy_from_slice(&self.x_axis.to_array());
        arr[4..8].copy_from_slice(&self.y_axis.to_array());
        arr[8..12].copy_from_slice(&self.z_axis.to_array());
        arr[12..16].copy_from_slice(&self.w_axis.to_array());
        arr
    }

    pub fn from_array(col0: [f32; 4], col1: [f32; 4], col2: [f32; 4], col3: [f32; 4]) -> Self {
        Mat4 {
            x_axis: Vec4::from_array(&col0),
            y_axis: Vec4::from_array(&col1),
            z_axis: Vec4::from_array(&col2),
            w_axis: Vec4::from_array(&col3),
        }
    }

    pub fn from_vec4s(col0: Vec4, col1: Vec4, col2: Vec4, col3: Vec4) -> Self {
        Mat4 {
            x_axis: col0,
            y_axis: col1,
            z_axis: col2,
            w_axis: col3,
        }
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self::from_vec4s(
            Vec4::X,
            Vec4::Y,
            Vec4::Z,
            Vec4::from_array(&[translation.x, translation.y, translation.z, 1.0]),
        )
    }

    pub fn from_rotation_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Mat4 {
            x_axis: Vec4::from_array(&[c, s, 0.0, 0.0]),
            y_axis: Vec4::from_array(&[-s, c, 0.0, 0.0]),
            z_axis: Vec4::Z,
            w_axis: Vec4::W,
        }
    }

    pub fn from_rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Mat4 {
            x_axis: Vec4::X,
            y_axis: Vec4::from_array(&[0.0, c, s, 0.0]),
            z_axis: Vec4::from_array(&[0.0, -s, c, 0.0]),
            w_axis: Vec4::W,
        }
    }

    pub fn from_rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Mat4 {
            x_axis: Vec4::from_array(&[c, 0.0, -s, 0.0]),
            y_axis: Vec4::Y,
            z_axis: Vec4::from_array(&[s, 0.0, c, 0.0]),
            w_axis: Vec4::W,
        }
    }

    // pub fn rotate_y(&mut self, angle: f32) {
    //     let c = angle.cos();
    //     let s = angle.sin();
    //     self.0[0][0] = c;
    //     self.0[0][2] = s;
    //     self.0[2][0] = -s;
    //     self.0[2][2] = c;
    // }

    // pub fn rotate_z(&mut self, angle: f32) {
    //     let c = angle.cos();
    //     let s = angle.sin();
    //     self.0[0][0] = c;
    //     self.0[0][1] = -s;
    //     self.0[1][0] = s;
    //     self.0[1][1] = c;
    // }

    #[inline]
    pub fn rotate(rotation: Vec3) -> Mat4 {
        let c3: f32 = rotation.z.cos();
        let s3: f32 = rotation.z.sin();
        let c2: f32 = rotation.x.cos();
        let s2: f32 = rotation.x.sin();
        let c1: f32 = rotation.y.cos();
        let s1: f32 = rotation.y.sin();

        Mat4::from_array(
            [
                c1 * c3 + s1 * s2 * s3,
                c2 * s3,
                c1 * s2 * s3 - c3 * s1,
                0.0f32,
            ],
            [
                c3 * s1 * s2 - c1 * s3,
                c2 * c3,
                c1 * c3 * s2 + s1 * s3,
                0.0f32,
            ],
            [c2 * s1, -s2, c1 * c2, 0.0f32],
            [0.0f32, 0.0f32, 0.0f32, 1.0f32],
        )
    }

    #[inline]
    pub fn create_perspective_gl(
        vertical_fov: f32,
        aspect_ratio: f32,
        z_near: f32,
        z_far: f32,
    ) -> Mat4 {
        let t = (vertical_fov / 2.0).tan();
        let sy = 1.0 / t;
        let sx = sy / aspect_ratio;
        let nmf = z_near - z_far;

        Mat4::from_array(
            [sx, 0.0, 0.0, 0.0],
            [0.0, sy, 0.0, 0.0],
            [0.0, 0.0, (z_far + z_near) / nmf, -1.0],
            [0.0, 0.0, 2.0 * z_near * z_far / nmf, 0.0],
        )
    }

    // pub fn rotate(&mut self, rotation : Vec3) {
    // 	let rot_x = Mat4::from_rotation_x(rotation.x);
    // 	let rot_y = Mat4::from_rotation_y(rotation.y);
    // 	let rot_z = Mat4::from_rotation_z(rotation.z);

    // 	*self = Mat4::mul_mat4(rot_z, Mat4::mul_mat4(rot_y, rot_x));
    // }
    // pub fn translate(&mut self, translation: [f32; 3]) {
    //     self.0[0][3] += translation[0];
    //     self.0[1][3] += translation[1];
    //     self.0[2][3] += translation[2];
    // }

    // pub fn scale(&mut self, scale: [f32; 3]) {
    // 	self.0[0][0] *= scale[0];
    // 	self.0[1][1] *= scale[1];
    // 	self.0[2][2] *= scale[2];
    // }

    pub fn mul_vec4(&self, rhs: Vec4) -> Mat4 {
        Mat4 {
            x_axis: Vec4::from_array(&[
                self.x_axis.x * rhs.x,
                self.x_axis.y * rhs.y,
                self.x_axis.z * rhs.z,
                self.x_axis.w * rhs.w,
            ]),
            y_axis: Vec4::from_array(&[
                self.y_axis.x * rhs.x,
                self.y_axis.y * rhs.y,
                self.y_axis.z * rhs.z,
                self.y_axis.w * rhs.w,
            ]),
            z_axis: Vec4::from_array(&[
                self.z_axis.x * rhs.x,
                self.z_axis.y * rhs.y,
                self.z_axis.z * rhs.z,
                self.z_axis.w * rhs.w,
            ]),
            w_axis: Vec4::from_array(&[
                self.w_axis.x * rhs.x,
                self.w_axis.y * rhs.y,
                self.w_axis.z * rhs.z,
                self.w_axis.w * rhs.w,
            ]),
        }
        // res
    }

    pub fn mul_mat4(lhs: Mat4, rhs: Mat4) -> Mat4 {
        Mat4 {
            x_axis: Vec4::from_array(&[
                lhs.x_axis.x * rhs.x_axis.x
                    + lhs.y_axis.x * rhs.x_axis.y
                    + lhs.z_axis.x * rhs.x_axis.z
                    + lhs.w_axis.x * rhs.x_axis.w,
                lhs.x_axis.y * rhs.x_axis.x
                    + lhs.y_axis.y * rhs.x_axis.y
                    + lhs.z_axis.y * rhs.x_axis.z
                    + lhs.w_axis.y * rhs.x_axis.w,
                lhs.x_axis.z * rhs.x_axis.x
                    + lhs.y_axis.z * rhs.x_axis.y
                    + lhs.z_axis.z * rhs.x_axis.z
                    + lhs.w_axis.z * rhs.x_axis.w,
                lhs.x_axis.w * rhs.x_axis.x
                    + lhs.y_axis.w * rhs.x_axis.y
                    + lhs.z_axis.w * rhs.x_axis.z
                    + lhs.w_axis.w * rhs.x_axis.w,
            ]),
            y_axis: Vec4::from_array(&[
                lhs.x_axis.x * rhs.y_axis.x
                    + lhs.y_axis.x * rhs.y_axis.y
                    + lhs.z_axis.x * rhs.y_axis.z
                    + lhs.w_axis.x * rhs.y_axis.w,
                lhs.x_axis.y * rhs.y_axis.x
                    + lhs.y_axis.y * rhs.y_axis.y
                    + lhs.z_axis.y * rhs.y_axis.z
                    + lhs.w_axis.y * rhs.y_axis.w,
                lhs.x_axis.z * rhs.y_axis.x
                    + lhs.y_axis.z * rhs.y_axis.y
                    + lhs.z_axis.z * rhs.y_axis.z
                    + lhs.w_axis.z * rhs.y_axis.w,
                lhs.x_axis.w * rhs.y_axis.x
                    + lhs.y_axis.w * rhs.y_axis.y
                    + lhs.z_axis.w * rhs.y_axis.z
                    + lhs.w_axis.w * rhs.y_axis.w,
            ]),
            z_axis: Vec4::from_array(&[
                lhs.x_axis.x * rhs.z_axis.x
                    + lhs.y_axis.x * rhs.z_axis.y
                    + lhs.z_axis.x * rhs.z_axis.z
                    + lhs.w_axis.x * rhs.z_axis.w,
                lhs.x_axis.y * rhs.z_axis.x
                    + lhs.y_axis.y * rhs.z_axis.y
                    + lhs.z_axis.y * rhs.z_axis.z
                    + lhs.w_axis.y * rhs.z_axis.w,
                lhs.x_axis.z * rhs.z_axis.x
                    + lhs.y_axis.z * rhs.z_axis.y
                    + lhs.z_axis.z * rhs.z_axis.z
                    + lhs.w_axis.z * rhs.z_axis.w,
                lhs.x_axis.w * rhs.z_axis.x
                    + lhs.y_axis.w * rhs.z_axis.y
                    + lhs.z_axis.w * rhs.z_axis.z
                    + lhs.w_axis.w * rhs.z_axis.w,
            ]),
            w_axis: Vec4::from_array(&[
                lhs.x_axis.x * rhs.w_axis.x
                    + lhs.y_axis.x * rhs.w_axis.y
                    + lhs.z_axis.x * rhs.w_axis.z
                    + lhs.w_axis.x * rhs.w_axis.w,
                lhs.x_axis.y * rhs.w_axis.x
                    + lhs.y_axis.y * rhs.w_axis.y
                    + lhs.z_axis.y * rhs.w_axis.z
                    + lhs.w_axis.y * rhs.w_axis.w,
                lhs.x_axis.z * rhs.w_axis.x
                    + lhs.y_axis.z * rhs.w_axis.y
                    + lhs.z_axis.z * rhs.w_axis.z
                    + lhs.w_axis.z * rhs.w_axis.w,
                lhs.x_axis.w * rhs.w_axis.x
                    + lhs.y_axis.w * rhs.w_axis.y
                    + lhs.z_axis.w * rhs.w_axis.z
                    + lhs.w_axis.w * rhs.w_axis.w,
            ]),
        }
    }

    pub fn print_mat4(&self) {
        println!("Mat4:");
        println!("x_axis: {:?}", self.x_axis);
        println!("y_axis: {:?}", self.y_axis);
        println!("z_axis: {:?}", self.z_axis);
        println!("w_axis: {:?}", self.w_axis);
    }
    #[inline]
    pub const fn as_ptr(&self) -> *const f32 {
        self as *const Self as *const f32
    }
}

// impl From<[[f32; 4]; 4]> for Mat4 {
//     fn from(arr: [[f32; 4]; 4]) -> Self {
//         Self(arr)
//     }
// }
