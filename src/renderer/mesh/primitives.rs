use super::{Mesh, Vertex};
use std::f32::consts::PI;
use glam::{Mat3, Vec3};
use crate::game::transform::{MatrixTransform3D, Transform3D};

pub fn quad() -> Mesh {
    Mesh {
        vertices: vec!(
            Vertex { position: [-0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [ 0.5, -0.5, 0.0], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [ 0.5,  0.5, 0.0], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [-0.5,  0.5, 0.0], normal: [0.0, 0.0, 1.0] }
        ),
        indeces: vec!(
            0, 1, 2,
            0, 2, 3
        )
    }
}

//gives a cube with width 1 and center as origin
pub fn cube() -> Mesh {
    let rotations: Vec<MatrixTransform3D> = vec!{
        Mat3::from_rotation_y(PI *  0.0).into(),
        Mat3::from_rotation_y(PI *  0.5).into(),
        Mat3::from_rotation_y(PI *  1.0).into(),
        Mat3::from_rotation_y(PI *  1.5).into(),
        Mat3::from_rotation_x(PI * -0.5).into(),
        Mat3::from_rotation_x(PI *  0.5).into(),
    };
    let translation: MatrixTransform3D = Transform3D{ position: Vec3::Z * 0.5, ..Default::default() }.into();

    return rotations
        .iter()
        .map(|rotation| quad().as_transformed(&(*rotation * translation)))
        .sum();
}