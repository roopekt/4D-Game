use super::{Mesh, Vertex};
use glam::{Mat3, Vec3};
use crate::game::transform::{AffineTransform3D, Transform3D};

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

pub fn blit_quad() -> Mesh {
    quad().as_transformed(&Transform3D { scale: 2.0 * Vec3::ONE, ..Default::default() }.into())
}

//gives a cube with width 1 and origo as the center
pub fn cube() -> Mesh {
    //construct 6 linear transformations (1 per face) such that the Z basis vector gets swapped for any positive or negative basis vector exactly once
    let mut orthogonal_transforms: Vec<AffineTransform3D> = Vec::with_capacity(2*3);
    for i in 0..3 {
        for sign in [-1.0, 1.0] {
            let mut matrix = Mat3::IDENTITY;

            const Z_INDEX: usize = 2;
            let column_Z = matrix.col(Z_INDEX);
            let column_i = matrix.col(i);
            *matrix.col_mut(i) = column_Z;
            *matrix.col_mut(Z_INDEX) = sign * column_i;

            orthogonal_transforms.push(matrix.into());
        }
    }
    let translation: AffineTransform3D = Transform3D{ position: Vec3::Z * 0.5, ..Default::default() }.into();

    return orthogonal_transforms
        .iter()
        .map(|orthogonal_transform| quad().as_transformed(&(*orthogonal_transform * translation)))
        .sum();
}