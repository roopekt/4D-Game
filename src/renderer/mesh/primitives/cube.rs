use super::{Mesh3D, Mesh4D, EdgeIndeces};
use glam::{Mat3, Vec3, BVec3, Mat4, Vec4, BVec4A};
use crate::game::transform::{AffineTransform3D, Transform3D, AffineTransform4D, Transform4D};
use super::{quad_3D, cube_4D};
use std::collections::HashMap;

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
    let mut mesh: Mesh3D = orthogonal_transforms
        .iter()
        .map(|orthogonal_transform| face.clone().as_transformed(&(*orthogonal_transform * translation)))
        .sum();

    //remove duplicate skeleton edges
    let discrete_vertices: Vec<BVec3> = mesh.vertices.iter()
        .map(|&v| v.position.cmpge(Vec3::ZERO))
        .collect();
    let mut filter_hash_map: HashMap<[BVec3; 1], [usize; 1]> = HashMap::new();
    for packed_vertex_index in mesh.skeleton_indeces {
        filter_hash_map.insert([discrete_vertices[packed_vertex_index[0]]], packed_vertex_index);
    }
    mesh.skeleton_indeces = filter_hash_map.values().copied().collect();

    mesh
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
    let mut mesh: Mesh4D = orthogonal_transforms
        .iter()
        .map(|orthogonal_transform| face.clone().as_transformed(&(*orthogonal_transform * translation)))
        .sum();

    //remove duplicate skeleton edges
    let discrete_vertices: Vec<BVec4A> = mesh.vertices.iter()
        .map(|&v| v.position.cmpge(Vec4::ZERO))
        .collect();
    let mut filter_hash_map: HashMap<[BVec4A; 2], [usize; 2]> = HashMap::new();
    for edge in mesh.skeleton_indeces {
        let ordered_edge: EdgeIndeces = edge.into();
        filter_hash_map.insert([discrete_vertices[ordered_edge.A], discrete_vertices[ordered_edge.B]], edge);
    }
    mesh.skeleton_indeces = filter_hash_map.values().copied().collect();

    mesh
}