use glam::{Mat3, Mat4};
use super::{matrix3x3, matrix4x4};

// 3D ones are counter-clockwise for left-handed coordinates

// References:
// https://en.wikipedia.org/wiki/Rotation_matrix
// https://en.wikipedia.org/wiki/Rotations_in_4-dimensional_Euclidean_space
// https://www.research.kobe-u.ac.jp/csi-viz/members/kageyama/docs/150629_euler_angles_in4d.pdf

pub fn around_x(angle: f32) -> Mat3 {
    let (sin, cos) = angle.sin_cos();
    matrix3x3!(
        1.0, 0.0, 0.0,
        0.0, cos, sin,
        0.0,-sin, cos
    )
}

pub fn around_y(angle: f32) -> Mat3 {
    let (sin, cos) = angle.sin_cos();
    matrix3x3!(
        cos, 0.0,-sin,
        0.0, 1.0, 0.0,
        sin, 0.0, cos
    )
}

pub fn around_z(angle: f32) -> Mat3 {
    let (sin, cos) = angle.sin_cos();
    matrix3x3!(
        cos, sin, 0.0,
       -sin, cos, 0.0,
        0.0, 0.0, 1.0
    )
}

pub fn around_xy(angle: f32) -> Mat4 {
    let (sin, cos) = angle.sin_cos();
    matrix4x4!(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, cos,-sin,
        0.0, 0.0, sin, cos
    )
}

pub fn around_xz(angle: f32) -> Mat4 {
    let (sin, cos) = angle.sin_cos();
    matrix4x4!(
        1.0, 0.0, 0.0, 0.0,
        0.0, cos, 0.0,-sin,
        0.0, 0.0, 1.0, 0.0,
        0.0, sin, 0.0, cos
    )
}

pub fn around_xw(angle: f32) -> Mat4 {
    let (sin, cos) = angle.sin_cos();
    matrix4x4!(
        1.0, 0.0, 0.0, 0.0,
        0.0, cos,-sin, 0.0,
        0.0, sin, cos, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

pub fn around_yz(angle: f32) -> Mat4 {
    let (sin, cos) = angle.sin_cos();
    matrix4x4!(
        cos, 0.0, 0.0,-sin,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        sin, 0.0, 0.0, cos
    )
}

pub fn around_yw(angle: f32) -> Mat4 {
    let (sin, cos) = angle.sin_cos();
    matrix4x4!(
        cos, 0.0,-sin, 0.0,
        0.0, 1.0, 0.0, 0.0,
        sin, 0.0, cos, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

pub fn around_zw(angle: f32) -> Mat4 {
    let (sin, cos) = angle.sin_cos();
    matrix4x4!(
        cos,-sin, 0.0, 0.0,
        sin, cos, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}
