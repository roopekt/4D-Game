mod sample_cloud;

use glam::{Vec3, IVec3};
use itertools::Itertools;
use super::{Mesh3D, primitives, vertex::CpuVertex3D};
use indexmap::IndexMap;

pub fn get_connected_isosurface_3D<F1, F2>(function: &F1, gradient: &F2, voxel_width: f32, negative_point: Vec3, positive_point: Vec3) -> Mesh3D
    where F1: Fn(Vec3) -> f32, F2: Fn(Vec3) -> Vec3
{
    let normalized_function = |normalized_coordinate: IVec3| {
        function(voxel_width * normalized_coordinate.as_vec3())
    };
    let normalized_negative_point = (negative_point / voxel_width).round().as_ivec3();
    let normalized_positive_point = (positive_point / voxel_width).round().as_ivec3();
    let sample_cloud = sample_cloud::SampleCloud3D::new(&normalized_function, normalized_negative_point, normalized_positive_point);

    let relative_quads = [0, 1, 2].map(|axis_index| get_quad_3D_with_nth_axis_as_normal(axis_index));
    let relative_cube = cube_3D_discrete_vertices();

    let mut vertices = IndexMap::<IVec3, CpuVertex3D>::new();
    let mut triangles = Vec::<[IVec3; 3]>::new();

    for border_pair in sample_cloud.border_pairs {
        let (relative_vertices, relative_triangles) = &relative_quads[border_pair.axis_index];

        for relative_vertex in relative_vertices {
            let absolute_discrete_vertex = *relative_vertex + border_pair.A;
            vertices.entry(absolute_discrete_vertex).or_insert_with(|| {
                let sample_points = relative_cube.iter()
                    .map(|&relative_cube_vertex| (relative_cube_vertex + absolute_discrete_vertex).as_vec3() * voxel_width)
                    .map(|pos| (pos, function(pos)))
                    .collect_vec();
                get_vertex(&sample_points, gradient)
            });
        }

        for relative_triangle in relative_triangles {
            triangles.push(relative_triangle.map(|relative_coord| relative_coord + border_pair.A));
        }
    };

    Mesh3D {
        indeces: triangles.iter()
            .map(|&triangle| triangle.map(|i3| vertices.get_index_of(&i3).unwrap()))
            .collect_vec(),
        vertices: vertices.into_values().collect_vec(),
        skeleton_indeces: Vec::new()
    }.with_full_skeleton()
}

fn get_vertex<F: Fn(Vec3) -> Vec3>(surrounding_sample_points: &Vec<(Vec3, f32)>, gradient_func: &F) -> CpuVertex3D {
    // let weighted_sum = surrounding_sample_points.iter()
    //     .map(|(pos, value)| *pos / value.abs())
    //     .sum::<Vec3>();
    // let total_weight = surrounding_sample_points.iter()
    //     .map(|(_pos, value)| 1.0 / value.abs())
    //     .sum::<f32>();
    // let position = weighted_sum / total_weight;

    let position = surrounding_sample_points.iter()
        .map(|(pos, _)| pos)
        .sum::<Vec3>() / surrounding_sample_points.len() as f32;

    CpuVertex3D {
        position: position,
        normal: gradient_func(position).normalize()
    }
}

fn get_quad_3D_with_nth_axis_as_normal(axis_index: usize) -> (Vec<IVec3>, Vec<[IVec3; 3]>) {
    let quad = primitives::quad_3D_discrete_vertices();
    
    let vertices = quad.0.iter()
        .map(|corner2| corner2.map(-1, 0))
        .map(|array2| IVec3::new(array2[0], array2[1], 0))
        .map(|mut vec_z_normal| {
            (vec_z_normal[axis_index], vec_z_normal.z) = (vec_z_normal.z, vec_z_normal[axis_index]);
            vec_z_normal
        })
        .collect_vec();

    let triangles = quad.1.iter()
        .map(|vertex_indeces| vertex_indeces.map(|i| vertices[i]))
        .collect_vec();

    (vertices, triangles)
}

fn cube_3D_discrete_vertices() -> Vec<IVec3> {
    primitives::CornerSigns::<3>::all()
        .iter()
        .map(|p| p.map(0, 1).into())
        .collect_vec()
}
