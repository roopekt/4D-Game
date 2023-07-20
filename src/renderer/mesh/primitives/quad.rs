pub mod corner_signs;
pub use corner_signs::*;
use itertools::Itertools;

use super::{Mesh3D, Mesh4D, CpuVertex3D, CpuVertex4D};
use glam::{Vec3, Vec4};
use crate::errors::assert_equal;
use super::{index_of, combinations_constsize};

pub fn quad_3D() -> Mesh3D {
    let (vertices, indeces) = quad_3D_discrete_vertices();
    quad_3D_from_discrete_quad(vertices, indeces)
}
pub fn cube_4D() -> Mesh4D {
    let (vertices, indeces) = cube_4D_discrete_vertices();
    cube_4D_from_discrete_cube(vertices, indeces)
}

pub fn quad_3D_from_discrete_quad(vertices: Vec<CornerSigns<2>>, indeces: Vec<[usize; 3]>) -> Mesh3D {
    let normal = Vec3::Z;
    let corner_signs_to_vertex = |vertex: &CornerSigns<2>| -> CpuVertex3D {
        let corner = vertex.map(-0.5_f32, 0.5);
        CpuVertex3D {
            position: Vec3::new(
                corner[0],
                corner[1],
                0.0
            ),
            normal
        }
    };

    Mesh3D {
        vertices: vertices
            .iter()
            .map(|&c| corner_signs_to_vertex(&c))
            .collect(),
        indeces,
        skeleton_indeces: Vec::new()
    }.with_full_skeleton()
}
pub fn cube_4D_from_discrete_cube(vertices: Vec<CornerSigns<3>>, indeces: Vec<[usize; 4]>) -> Mesh4D {
    let mut skeleton_indeces: Vec<[usize; 2]> = Vec::new();
    for edge in combinations_constsize::<2,_>(&(0..vertices.len()).collect_vec()) {
        let corner_A = vertices[edge[0]];
        let corner_B = vertices[edge[1]];
        let match_count = corner_A.match_count(&corner_B);

        let is_outer_edge = match match_count {
            2 => true,
            0 | 1 => false,
            _ => panic!("impossible match count {}", match_count)
        };
        if is_outer_edge {
            skeleton_indeces.push(edge);
        }
    }
    assert_equal!(skeleton_indeces.len(), 12);

    let normal = Vec4::W;
    let corner_signs_to_vertex = |vertex: &CornerSigns<3>| -> CpuVertex4D {
        let corner = vertex.map(-0.5_f32, 0.5);
        CpuVertex4D {
            position: Vec4::new(
                corner[0],
                corner[1],
                corner[2],
                0.0
            ),
            normal
        }
    };

    Mesh4D {
        vertices: vertices
            .iter()
            .map(|&c| corner_signs_to_vertex(&c))
            .collect(),
        indeces,
        skeleton_indeces: skeleton_indeces
    }
}

//yes, this is a very convoluted way to get a quad, but this is easier to generalize to 4D
pub fn quad_3D_discrete_vertices() -> (Vec<CornerSigns<2>>, Vec<[usize; 3]>) {
    let corners = CornerSigns::<2>::all();
    assert_equal!(corners.len(), 4);

    let mut corners_of_diagonal = vec!{CornerSigns::<2>::NEG};
    for corner in &corners {
        let match_count = corners_of_diagonal[0].match_count(corner);
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
                index_of(CornerSigns::<2>([-corner_i[0],  corner_i[1]]), &corners),
                index_of(CornerSigns::<2>([ corner_i[0], -corner_i[1]]), &corners)
            ]);
        }
    }
    assert_equal!(indeces.len(), 2);

    (corners, indeces)
}
pub fn cube_4D_discrete_vertices() -> (Vec<CornerSigns<3>>, Vec<[usize; 4]>) {
    let corners = CornerSigns::<3>::all();
    assert_equal!(corners.len(), 8);

    let mut corners_of_central = vec!{CornerSigns::<3>::NEG};
    for corner in &corners {
        let match_count = corners_of_central[0].match_count(corner);
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
                index_of(CornerSigns::<3>([-corner_i[0],  corner_i[1],  corner_i[2]]), &corners),
                index_of(CornerSigns::<3>([ corner_i[0], -corner_i[1],  corner_i[2]]), &corners),
                index_of(CornerSigns::<3>([ corner_i[0],  corner_i[1], -corner_i[2]]), &corners)
            ]);
        }
    }
    assert_equal!(indeces.len(), 5);

    (corners, indeces)
}
