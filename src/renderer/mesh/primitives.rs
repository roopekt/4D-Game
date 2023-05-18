use super::{Mesh, Vertex};
use std::f32::consts::PI;
use glam::{Mat4, Vec3};

pub fn quad() -> Mesh {
    Mesh {
        vertices: vec!(
            Vertex { position: [-0.5, -0.5, 0.0] },
            Vertex { position: [ 0.5, -0.5, 0.0] },
            Vertex { position: [ 0.5,  0.5, 0.0] },
            Vertex { position: [-0.5,  0.5, 0.0] }
        ),
        indeces: vec!(
            0, 1, 2,
            0, 2, 3
        )
    }
}

//gives a cube with width 1 and center as origin
pub fn cube() -> Mesh {
    let rotations = vec!{
        Mat4::from_rotation_y(PI * 0.0),
        Mat4::from_rotation_y(PI * 0.5),
        Mat4::from_rotation_y(PI * 1.0),
        Mat4::from_rotation_y(PI * 1.5),
        Mat4::from_rotation_x(PI * -0.5),
        Mat4::from_rotation_x(PI * 0.5),
    };
    let translation = Mat4::from_translation(Vec3::Z * 0.5);

    return rotations
        .iter()
        .map(|rotation| quad().as_transformed(*rotation * translation))
        .sum();
}