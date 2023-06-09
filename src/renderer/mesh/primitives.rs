use super::{Mesh3D, Mesh4D, Vertex3D, Vertex4D};
use glam::{Mat3, Vec3};
use crate::game::transform::{AffineTransform3D, Transform3D};
use crate::errors::assert_equal;
use std::fmt::Debug as DebugTrait;

//yes, this is a very convoluted way to get a quad, but this is easier to generalize to 4D
pub fn quad_3D() -> Mesh3D {
    type CornerSigns = [bool; 2];
    let corners: Vec<CornerSigns> = all_bool_arrays();
    assert_equal!(corners.len(), 4);

    let mut corners_of_diagonal: Vec<CornerSigns> = vec!{[false, false]};
    for corner in &corners {
        let match_count = bool_array_match_count(&corners[0], corner);
        let is_new_diagonal = match match_count {
            0 => true,
            1 | 2 => false,
            _ => panic!("impossible match count {}", match_count)
        };
        if is_new_diagonal {
            corners_of_diagonal.push(*corner);
        }
    }
    assert_equal!(corners_of_diagonal.len(), 2);

    let mut triangle_indeces: Vec<usize> = Vec::new();
    for i in 0..corners.len() {
        let corner_i = &corners[i];
        if !corners_of_diagonal.contains(corner_i) {
            triangle_indeces.push(i);
            triangle_indeces.push(index_of([!corner_i[0],  corner_i[1]], &corners));
            triangle_indeces.push(index_of([ corner_i[0], !corner_i[1]], &corners));
        }
    }
    assert_equal!(triangle_indeces.len(), 2*3);

    let normal = [0.0, 0.0, 1.0];
    let corner_signs_to_vertex = |signs: &CornerSigns| -> Vertex3D {
        let corner_sign_to_number = |sign: bool| {
            match sign {
                false => -0.5,
                true => 0.5
            }
        };

        Vertex3D {
            position: [
                corner_sign_to_number(signs[0]),
                corner_sign_to_number(signs[1]),
                0.0
            ],
            normal
        }
    };

    Mesh3D {
        vertices: corners
            .iter()
            .map(|&c| corner_signs_to_vertex(&c))
            .collect(),
        indeces: triangle_indeces
            .iter()
            .map(|&i| i.try_into().unwrap())
            .collect()
    }
}

pub fn blit_quad() -> Mesh3D {
    quad_3D().as_transformed(&Transform3D { scale: 2.0 * Vec3::ONE, ..Default::default() }.into())
}

pub fn cube_4D() -> Mesh4D {
    type CornerSigns = [bool; 3];
    let corners: Vec<CornerSigns> = all_bool_arrays();
    assert_equal!(corners.len(), 8);

    let mut corners_of_central: Vec<CornerSigns> = vec![[false, false, false]];
    for corner in &corners {
        let match_count = bool_array_match_count(&corners[0], corner);
        let is_new_diagonal = match match_count {
            1 => true,
            0 | 2 | 3 => false,
            _ => panic!("impossible match count {}", match_count)
        };
        if is_new_diagonal {
            corners_of_central.push(*corner);
        }
    }
    assert_equal!(corners_of_central.len(), 4);

    let mut tetrahedron_indeces: Vec<usize> = corners_of_central
        .iter()
        .map(|corner| index_of(*corner, &corners))
        .collect();
    for i in 0..corners.len() {
        let corner_i = &corners[i];
        if !corners_of_central.contains(corner_i) {
            tetrahedron_indeces.push(i);
            tetrahedron_indeces.push(index_of([!corner_i[0],  corner_i[1],  corner_i[2]], &corners));
            tetrahedron_indeces.push(index_of([ corner_i[0], !corner_i[1],  corner_i[2]], &corners));
            tetrahedron_indeces.push(index_of([ corner_i[0],  corner_i[1], !corner_i[2]], &corners));
        }
    }
    assert_equal!(tetrahedron_indeces.len(), 5*4);

    let normal = [0.0, 0.0, 0.0, 1.0];
    let corner_signs_to_vertex = |signs: &CornerSigns| -> Vertex4D {
        let corner_sign_to_number = |sign: bool| {
            match sign {
                false => -0.5,
                true => 0.5
            }
        };

        Vertex4D {
            position: [
                corner_sign_to_number(signs[0]),
                corner_sign_to_number(signs[1]),
                corner_sign_to_number(signs[2]),
                0.0
            ],
            normal
        }
    };

    Mesh4D {
        vertices: corners
            .iter()
            .map(|&c| corner_signs_to_vertex(&c))
            .collect(),
        indeces: tetrahedron_indeces
            .iter()
            .map(|&i| i.try_into().unwrap())
            .collect()
    }
}

//gives a cube with width 1 and origo as the center
pub fn cube_3D() -> Mesh3D {
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

    let face = quad_3D();
    return orthogonal_transforms
        .iter()
        .map(|orthogonal_transform| face.clone().as_transformed(&(*orthogonal_transform * translation)))
        .sum();
}

fn all_bool_arrays<const BOOL_COUNT: usize>() -> Vec<[bool; BOOL_COUNT]> {
    (0..(1 << BOOL_COUNT))
        .map(|int| int_to_bool_array(int))
        .collect()
}

fn bool_array_match_count<const BOOL_COUNT: usize>(a: &[bool; BOOL_COUNT], b: &[bool; BOOL_COUNT]) -> usize {
    let mut match_count = 0;
    for i in 0..BOOL_COUNT {
        if a[i] == b[i] {
            match_count += 1;
        }
    }
    match_count
}

fn int_to_bool_array<const COUNT: usize>(int: u32) -> [bool; COUNT] {
    let vec: Vec<bool> = (0..COUNT)
        .map(|i| ((int >> i) & 1) == 1)
        .collect();
    vec.try_into().unwrap()
}

fn index_of<T: PartialEq + DebugTrait>(element: T, vec: &Vec<T>) -> usize {
    vec.iter().position(|e| *e == element).expect(&format!("Didn't find {:?}", element))
}
