use crate::Vec3;

pub struct Mat3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}

// impl Mat3 {
//     pub fn new() -> Self {
//         Mat3 {
//             x_axis: Vec3::zero(),
//             y_axis: Vec3::zero(),
//             z_axis: Vec3::zero(),
//         }
//     }

//     pub fn from_cols_array(arr: &[f32; 9]) -> Self {
//         let mut mat = Mat3::new();
//         for i in 0..3 {
//             for j in 0..3 {
//                 mat.0[i][j] = arr[i * 3 + j];
//             }
//         }
//         mat
//     }

//     pub fn identity() -> Self {
//         Self([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
//     }

//     pub fn to_cols_array(&self) -> [f32; 9] {
//         let mut arr = [0.0; 9];
//         for i in 0..3 {
//             for j in 0..3 {
//                 arr[i * 3 + j] = self.0[i][j];
//             }
//         }
//         arr
//     }
// }
