use super::{Mesh3D, Mesh4D, CpuVertex3D, CpuVertex4D, CpuVertexSimple, SimpleMesh};
use glam::{Mat3, Vec3, Mat4, Vec4, Vec4Swizzles};
use crate::game::transform::{AffineTransform3D, Transform3D, AffineTransform4D, Transform4D};
use crate::errors::assert_equal;
use std::fmt::Debug as DebugTrait;
use std::f32;
use combinatorial::Combinations;

pub fn blit_quad() -> SimpleMesh {
    quad_3D()
        .as_transformed(&Transform3D { scale: 2.0 * Vec3::ONE, ..Default::default() }.into())
        .into()
}

pub fn vertical_line() -> SimpleMesh {
    SimpleMesh {
        vertices: vec![
            CpuVertexSimple { position: Vec3::new(0.0, -1.0, 0.0) },
            CpuVertexSimple { position: Vec3::new(0.0,  1.0, 0.0) }
        ],
        indeces: vec![0, 1],
        topology: glium::index::PrimitiveType::LinesList
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

    let mut indeces: Vec<[usize; 3]> = Vec::new();
    for i in 0..corners.len() {
        let corner_i = &corners[i];
        if !corners_of_diagonal.contains(corner_i) {
            indeces.push([
                i,
                index_of([!corner_i[0],  corner_i[1]], &corners),
                index_of([ corner_i[0], !corner_i[1]], &corners)
            ]);
        }
    }
    assert_equal!(indeces.len(), 2);

    let normal = Vec3::Z;
    let corner_signs_to_vertex = |signs: &CornerSigns| -> CpuVertex3D {
        let corner_sign_to_number = |sign: bool| {
            match sign {
                false => -0.5,
                true => 0.5
            }
        };

        CpuVertex3D {
            position: Vec3::new(
                corner_sign_to_number(signs[0]),
                corner_sign_to_number(signs[1]),
                0.0
            ),
            normal
        }
    };

    Mesh3D {
        vertices: corners
            .iter()
            .map(|&c| corner_signs_to_vertex(&c))
            .collect(),
        indeces
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

    let mut indeces: Vec<[usize; 4]> = vec![corners_of_central.iter()
        .map(|corner| index_of(*corner, &corners))
        .collect::<Vec<usize>>().try_into().unwrap()];
    for i in 0..corners.len() {
        let corner_i = &corners[i];
        if !corners_of_central.contains(corner_i) {
            indeces.push([
                i,
                index_of([!corner_i[0],  corner_i[1],  corner_i[2]], &corners),
                index_of([ corner_i[0], !corner_i[1],  corner_i[2]], &corners),
                index_of([ corner_i[0],  corner_i[1], !corner_i[2]], &corners)
            ]);
        }
    }
    assert_equal!(indeces.len(), 5);

    let normal = Vec4::W;
    let corner_signs_to_vertex = |signs: &CornerSigns| -> CpuVertex4D {
        let corner_sign_to_number = |sign: bool| {
            match sign {
                false => -0.5,
                true => 0.5
            }
        };

        CpuVertex4D {
            position: Vec4::new(
                corner_sign_to_number(signs[0]),
                corner_sign_to_number(signs[1]),
                corner_sign_to_number(signs[2]),
                0.0
            ),
            normal
        }
    };

    Mesh4D {
        vertices: corners
            .iter()
            .map(|&c| corner_signs_to_vertex(&c))
            .collect(),
        indeces
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
    let mut triangle_indeces: Vec<[usize; 3]> = array_combinations(0..vertices.len()).collect();//all possible triangles

    for _ in 0..subdivisions {
        (vertices, triangle_indeces) = subdivide_unit_sphere_3D(vertices, triangle_indeces);
    };

    Mesh3D {
        vertices: vertices
            .iter()
            .map(|&v| CpuVertex3D { position: v, normal: v })
            .collect(),
        indeces: triangle_indeces
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

fn subdivide_unit_sphere_3D(mut vertices: Vec<Vec3>, triangle_indeces: Vec<[usize; 3]>) -> (Vec<Vec3>, Vec<[usize; 3]>) {
    let mut new_indeces = Vec::new();
    for triangle in triangle_indeces {    
        let index_offset = vertices.len();
        let edges: Vec<Vec<usize>> = Combinations::of_size(triangle.clone(), 2).collect();
        assert_equal!(edges.len(), 3);

        //new vertices
        for edge in &edges {
            let mid_edge_vertex = ((vertices[edge[0]] + vertices[edge[1]]) * 0.5).normalize();
            vertices.push(mid_edge_vertex);
        }

        //corner triangles
        for corner_index in triangle {
            let new_relative_vertex_indeces: Vec<usize> = edges.iter()
                .enumerate()
                .filter(|(_i, edge)| edge.contains(&corner_index))
                .map(|(i, _edge)| i)
                .collect();
            assert_equal!(new_relative_vertex_indeces.len(), 2);

            new_indeces.push([
                corner_index,
                index_offset + new_relative_vertex_indeces[0],
                index_offset + new_relative_vertex_indeces[1]
            ]);
        }

        //mid triangle
        new_indeces.push([
            index_offset + 0,
            index_offset + 1,
            index_offset + 2
        ]);
    }

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

fn array_combinations<T: Ord + Clone + DebugTrait, const COMBINATION_SIZE: usize>(source: impl IntoIterator<Item = T>) -> impl Iterator<Item = [T; COMBINATION_SIZE]> {
    Combinations::of_size(source, COMBINATION_SIZE)
        .map(|c| c.try_into().expect("Combinations::of_size should return vectors of compatible length."))
}
