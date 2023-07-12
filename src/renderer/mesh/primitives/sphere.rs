use super::{Mesh3D, CpuVertex3D, Mesh4D, CpuVertex4D};
use glam::{Vec4, Vec4Swizzles};
use crate::errors::assert_equal;
use super::combinations_csize;

pub fn sphere_3D(surface_subdivisions: usize, skeleton_subdivisions: usize) -> Mesh3D {
    let mut mesh = sphere_3D_no_skeleton(surface_subdivisions);
    mesh.skeleton_indeces = sphere_3D_no_skeleton(skeleton_subdivisions).with_full_skeleton().skeleton_indeces;
    mesh
}
pub fn sphere_4D(surface_subdivisions: usize, skeleton_subdivisions: usize) -> Mesh4D {
    let mut mesh = sphere_4D_no_skeleton(surface_subdivisions);
    mesh.skeleton_indeces = sphere_4D_no_skeleton(skeleton_subdivisions).with_full_skeleton().skeleton_indeces;
    mesh
}

fn sphere_3D_no_skeleton(subdivisions: usize) -> Mesh3D {
    let vertices: Vec<CpuVertex3D> = get_low_poly_sphere_vertices_general_dimension(3)
        .iter()
        .map(|&v| v.xyz())
        .map(|v| CpuVertex3D { position: v, normal: v })
        .collect();
    let indeces = combinations_csize(0..vertices.len()).collect();//all possible triangles
    let mut mesh = Mesh3D {
        vertices: vertices,
        indeces: indeces,
        skeleton_indeces: Vec::new()
    };

    for _ in 0..subdivisions {
        mesh = mesh.subdivide_surface();

        //project onto the unit sphere
        for vertex in mesh.vertices.iter_mut() {
            vertex.position = vertex.position.normalize();
        }
    };

    mesh
}
fn sphere_4D_no_skeleton(subdivisions: usize) -> Mesh4D {
    let vertices: Vec<CpuVertex4D> = get_low_poly_sphere_vertices_general_dimension(4)
        .iter()
        .map(|&v| CpuVertex4D { position: v, normal: v })
        .collect();
    let indeces = combinations_csize(0..vertices.len()).collect();//all possible tetrahedra
    let mut mesh = Mesh4D {
        vertices: vertices,
        indeces: indeces,
        skeleton_indeces: Vec::new()
    };

    for _ in 0..subdivisions {
        mesh = mesh.subdivide_surface();

        //project onto the unit sphere
        for vertex in mesh.vertices.iter_mut() {
            vertex.position = vertex.position.normalize();
        }
    };

    mesh
}

//tetrahedron in 3D, 5-cell in 4D
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
