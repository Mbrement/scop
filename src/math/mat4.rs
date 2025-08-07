use crate::Vec4;

pub struct Mat4([[f32; 4]; 4]);

impl Mat4 {
    pub fn new() -> Self {
        Mat4([[0.0; 4]; 4])
    }

    pub fn from_cols_array(arr: &[f32; 16]) -> Self {
        let mut mat = Mat4::new();
        for i in 0..4 {
            for j in 0..4 {
                mat.0[i][j] = arr[i * 4 + j];
            }
        }
        mat
    }

    pub fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn to_cols_array(&self) -> [f32; 16] {
        let mut arr = [0.0; 16];
        for i in 0..4 {
            for j in 0..4 {
                arr[i * 4 + j] = self.0[i][j];
            }
        }
        arr
    }

    pub fn from_rotation_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        Self([
			[c, s, 0.0, 0.0],
			[-s, c, 0.0, 0.0],
			Vec4::Z.to_array(),
			Vec4::W.to_array(),
		])
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let c = angle.cos();
        let s = angle.sin();
        self.0[1][1] = c;
        self.0[1][2] = -s;
        self.0[2][1] = s;
        self.0[2][2] = c;
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let c = angle.cos();
        let s = angle.sin();
        self.0[0][0] = c;
        self.0[0][2] = s;
        self.0[2][0] = -s;
        self.0[2][2] = c;
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let c = angle.cos();
        let s = angle.sin();
        self.0[0][0] = c;
        self.0[0][1] = -s;
        self.0[1][0] = s;
        self.0[1][1] = c;
    }

    pub fn translate(&mut self, translation: [f32; 3]) {
        self.0[0][3] += translation[0];
        self.0[1][3] += translation[1];
        self.0[2][3] += translation[2];
    }
}
impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(arr: [[f32; 4]; 4]) -> Self {
        Self(arr)
    }
}
