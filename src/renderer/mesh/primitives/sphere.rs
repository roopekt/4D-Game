use super::{Mesh3D, CpuVertex3D};
use glam::{Vec4, Vec4Swizzles};
use crate::errors::assert_equal;
use std::fmt::Debug as DebugTrait;
use std::f32;
use combinatorial::Combinations;

pub fn sphere_3D(subdivisions: usize) -> Mesh3D {
    let vertices: Vec<CpuVertex3D> = get_low_poly_sphere_vertices_general_dimension(3)
        .iter()
        .map(|&v| v.xyz())
        .map(|v| CpuVertex3D { position: v, normal: v })
        .collect();
    let indeces = array_combinations(0..vertices.len()).collect();//all possible triangles
    let mut mesh = Mesh3D {
        vertices: vertices,
        indeces: indeces
    };

    for _ in 0..subdivisions {
        mesh = mesh.subdivide();

        //project onto the unit sphere
        for vertex in mesh.vertices.iter_mut() {
            vertex.position = vertex.position.normalize();
        }
    };

    mesh
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

fn array_combinations<T: Ord + Clone + DebugTrait, const COMBINATION_SIZE: usize>(source: impl IntoIterator<Item = T>) -> impl Iterator<Item = [T; COMBINATION_SIZE]> {
    Combinations::of_size(source, COMBINATION_SIZE)
        .map(|c| c.try_into().expect("Combinations::of_size should return vectors of compatible length."))
}