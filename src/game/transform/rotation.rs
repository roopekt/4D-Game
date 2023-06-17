use glam::{Mat3, Mat4};
use super::{matrix3x3, matrix4x4};
use rand::Rng;
use std::f32::consts::TAU;

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

//these naive random Euler angles are not uniform in SO(3) (think about gimbal lock)
pub fn random_3D_nonuniform<R: Rng>(rng: &mut R) -> Mat3 {
    around_z(rng.gen_range(0.0..TAU)) *
    around_y(rng.gen_range(0.0..TAU)) *
    around_x(rng.gen_range(0.0..TAU))
}
pub fn random_4D_nonuniform<R: Rng>(rng: &mut R) -> Mat4 {
    around_xy(rng.gen_range(0.0..TAU)) *
    around_xz(rng.gen_range(0.0..TAU)) *
    around_xw(rng.gen_range(0.0..TAU)) *
    around_yz(rng.gen_range(0.0..TAU)) *
    around_yw(rng.gen_range(0.0..TAU)) *
    around_zw(rng.gen_range(0.0..TAU))
}
