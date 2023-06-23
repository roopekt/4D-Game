use super::{Mesh3D, Mesh4D, Vertex3D, Vertex4D, IndexT};
use glam::{Mat3, Vec3, Mat4, Vec4, Vec4Swizzles};
use crate::game::transform::{AffineTransform3D, Transform3D, AffineTransform4D, Transform4D};
use crate::errors::assert_equal;
use std::fmt::Debug as DebugTrait;
use std::f32;
use combinatorial::Combinations;

pub fn blit_quad() -> Mesh3D {
    quad_3D().as_transformed(&Transform3D { scale: 2.0 * Vec3::ONE, ..Default::default() }.into())
}

pub fn vertical_line() -> Mesh3D {
    Mesh3D {
        vertices: vec![
            Vertex3D { position: [0.0, -1.0, 0.0], normal: [0.0, 0.0, 0.0] },
            Vertex3D { position: [0.0,  1.0, 0.0], normal: [0.0, 0.0, 0.0] }
        ],
        indeces: vec![0, 1]
    }
}

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
    let mut orthogonal_transforms: Vec<AffineTransform3D> = Vec::new();
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

//gives a tesseract with width 1 and origo as the center
pub fn tesseract_4D() -> Mesh4D {
    //construct 8 linear transformations (1 per face) such that the W basis vector gets swapped for any positive or negative basis vector exactly once
    let mut orthogonal_transforms: Vec<AffineTransform4D> = Vec::new();
    for i in 0..4 {
        for sign in [-1.0, 1.0] {
            let mut matrix = Mat4::IDENTITY;

            const W_INDEX: usize = 3;
            let column_W = matrix.col(W_INDEX);
            let column_i = matrix.col(i);
            *matrix.col_mut(i) = column_W;
            *matrix.col_mut(W_INDEX) = sign * column_i;

            orthogonal_transforms.push(matrix.into());
        }
    }
    let translation: AffineTransform4D = Transform4D { position: Vec4::W * 0.5, ..Default::default() }.into();

    let face = cube_4D();
    return orthogonal_transforms
        .iter()
        .map(|orthogonal_transform| face.clone().as_transformed(&(*orthogonal_transform * translation)))
        .sum();
}

pub fn sphere_3D(subdivisions: usize) -> Mesh3D {
    let mut vertices: Vec<Vec3> = get_low_poly_sphere_vertices_general_dimension(3)
        .iter().map(|&v| v.xyz()).collect();
    let mut triangle_indeces: Vec<Vec<usize>> = Combinations::of_size(0..vertices.len(), 3)
        .collect(); //all possible triangles

    for _ in 0..subdivisions {
        (vertices, triangle_indeces) = subdivide_unit_sphere_3D(vertices, triangle_indeces);
    };

    Mesh3D {
        vertices: vertices
            .iter()
            .map(|&v| Vertex3D { position: v.into(), normal: v.into() })
            .collect(),
        indeces: triangle_indeces
            .iter().flatten()
            .map(|&i| i.try_into().unwrap())
            .collect()
    }
}

fn get_low_poly_sphere_vertices_general_dimension(dimension: usize) -> Vec<Vec4> {
    let mut vertices = vec![Vec4::ZERO];
    const BASIS_VECTORS: [Vec4; 4] = [Vec4::X, Vec4::Y, Vec4::Z, Vec4::W];
    for i in 0..dimension {
        let center = vertices.iter().sum::<Vec4>() / vertices.len() as f32;
        let r_squared = (center - vertices[0]).length_squared();
        let h = f32::sqrt(1.0 - r_squared);//such that length of new edges (hypotenuse) is 1
        vertices.push(center + h * BASIS_VECTORS[i]);
    };

    //center and normalize the shape
    let center = vertices.iter().sum::<Vec4>() / vertices.len() as f32;
    vertices = vertices
        .iter()
        .map(|&v| (v - center).normalize())
        .collect();

    assert_equal!(vertices.len(), dimension + 1);
    vertices
}

fn subdivide_unit_sphere_3D(mut vertices: Vec<Vec3>, triangle_indeces: Vec<Vec<usize>>) -> (Vec<Vec3>, Vec<Vec<usize>>) {
    let mut new_indeces = Vec::new();
    for triangle in triangle_indeces {
        let original_vertices: Vec<Vec3> = triangle.iter()
            .map(|&i| vertices[i]).collect();
        let mid_vertex = (original_vertices.iter().sum::<Vec3>() / original_vertices.len() as f32).normalize();
        
        // let index_offset = new_vertices.len();
        let mid_vertex_index = vertices.len();
        vertices.push(mid_vertex);

        for outer_indeces in Combinations::<usize>::of_size(triangle, 2) {
            new_indeces.push(vec![
                outer_indeces[0],
                outer_indeces[1],
                mid_vertex_index
            ]);
        }
    };

    (vertices, new_indeces)
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
