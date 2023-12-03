mod sample_cloud;

use glam::{Vec3, IVec3};
use itertools::Itertools;
use self::sample_cloud::SampleCloud3D;
use super::{Mesh3D, primitives, vertex::CpuVertex3D};
use indexmap::IndexMap;

pub fn get_connected_isosurface_3D<F1, F2>(function: &F1, gradient: &F2, voxel_width: f32, skeleton_voxel_width: f32, negative_point: Vec3, positive_point: Vec3, use_newton_method: bool) -> Mesh3D
    where F1: Fn(Vec3) -> f32, F2: Fn(Vec3) -> Vec3
{
    let mut main_mesh = get_connected_isosurface_3D_no_skeleton(function, gradient,          voxel_width, negative_point, positive_point, use_newton_method);
    let skeleton_mesh = get_connected_isosurface_3D_no_skeleton(function, gradient, skeleton_voxel_width, negative_point, positive_point, use_newton_method);

    main_mesh.attach_skeleton(skeleton_mesh);
    main_mesh
}

pub fn get_connected_isosurface_3D_no_skeleton<F1, F2>(function: &F1, gradient: &F2, voxel_width: f32, negative_point: Vec3, positive_point: Vec3, use_newton_method: bool) -> Mesh3D
    where F1: Fn(Vec3) -> f32, F2: Fn(Vec3) -> Vec3
{
    let normalized_function = |normalized_coordinate: IVec3| {
        function(voxel_width * normalized_coordinate.as_vec3())
    };
    let normalized_negative_point = (negative_point / voxel_width).round().as_ivec3();
    let normalized_positive_point = (positive_point / voxel_width).round().as_ivec3();
    let sample_cloud = sample_cloud::SampleCloud3D::new(&normalized_function, normalized_negative_point, normalized_positive_point);

    let relative_quads = [0, 1, 2].map(|axis_index| get_quad_3D_with_nth_axis_as_normal(axis_index));

    let mut vertices = IndexMap::<IVec3, CpuVertex3D>::new();
    let mut triangles = Vec::<[IVec3; 3]>::new();

    for border_pair in &sample_cloud.border_pairs {
        let (relative_vertices, relative_triangles) = &relative_quads[border_pair.axis_index];

        for relative_vertex in relative_vertices {
            let discrete_corner_vertex = *relative_vertex + border_pair.A;//the minimum coordinate corner of the cube in which the new vertex will be in
            vertices.entry(discrete_corner_vertex).or_insert_with(|| {
                get_vertex(discrete_corner_vertex, &sample_cloud, &function, &gradient, voxel_width, use_newton_method)
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
    }
}

fn get_vertex<F1, F2>(discrete_corner_vertex: IVec3, sample_cloud: &SampleCloud3D, function: &F1, gradient_func: &F2, voxel_width: f32, use_newton_method: bool) -> CpuVertex3D
    where F1: Fn(Vec3) -> f32, F2: Fn(Vec3) -> Vec3
{
    const EDGES_PER_AXIS: usize = 4;

    let min_corner = discrete_corner_vertex.as_vec3() * voxel_width;
    let max_corner = (discrete_corner_vertex + IVec3::ONE).as_vec3() * voxel_width;

    fn get_coordinate_given_edge(value_A: f32, value_B: f32, coordinate_A: f32, coordinate_B: f32) -> f32 {
        let t = -value_A / (value_B - value_A);//inverse lerp (with a target of 0)
        coordinate_A + (coordinate_B - coordinate_A) * t //lerp
    }
    let get_coordinate = |relative_edges: &[(IVec3, IVec3); EDGES_PER_AXIS], min_coordinate: f32, max_coordinate: f32| -> f32 {
        let mut coordinate: f32 = 0.0;
        let mut counter: u32 = 0;
        for (offset_A, offset_B) in relative_edges {
            let value_A = sample_cloud.sample_map[&(discrete_corner_vertex + *offset_A)];
            let value_B = sample_cloud.sample_map[&(discrete_corner_vertex + *offset_B)];
            if (value_A < 0.0) != (value_B < 0.0) {
                coordinate += get_coordinate_given_edge(value_A, value_B, min_coordinate, max_coordinate);
                counter += 1;
            }
        };

        if counter > 0 {
            coordinate / (counter as f32)
        }
        else {
            (min_coordinate + max_coordinate) * 0.5
        }
    };

    const RELATIVE_EDGES_X: [(IVec3, IVec3); EDGES_PER_AXIS] = [
        (IVec3::new(0,0,0), IVec3::new(1,0,0)),
        (IVec3::new(0,0,1), IVec3::new(1,0,1)),
        (IVec3::new(0,1,0), IVec3::new(1,1,0)),
        (IVec3::new(0,1,1), IVec3::new(1,1,0))
    ];
    const RELATIVE_EDGES_Y: [(IVec3, IVec3); EDGES_PER_AXIS] = [
        (IVec3::new(0,0,0), IVec3::new(0,1,0)),
        (IVec3::new(0,0,1), IVec3::new(0,1,1)),
        (IVec3::new(1,0,0), IVec3::new(1,1,0)),
        (IVec3::new(1,0,1), IVec3::new(1,1,0))
    ];
    const RELATIVE_EDGES_Z: [(IVec3, IVec3); EDGES_PER_AXIS] = [
        (IVec3::new(0,0,0), IVec3::new(0,0,1)),
        (IVec3::new(0,1,0), IVec3::new(0,1,1)),
        (IVec3::new(1,0,0), IVec3::new(1,0,1)),
        (IVec3::new(1,1,0), IVec3::new(1,0,1))
    ];

    let mut position = Vec3::new(
        get_coordinate(&RELATIVE_EDGES_X, min_corner.x, max_corner.x),
        get_coordinate(&RELATIVE_EDGES_Y, min_corner.y, max_corner.y),
        get_coordinate(&RELATIVE_EDGES_Z, min_corner.z, max_corner.z)
    );

    //a single iteration of Newton's method
    if use_newton_method {
        let gradient = gradient_func(position);
        let gradient_inverse_length = gradient.length_recip();
        position = position - (gradient_inverse_length * gradient) * function(position) * gradient_inverse_length;
    }

    CpuVertex3D {
        position: position,
        normal: gradient_func(position).normalize()
    }
}

//all coordinates are either -1 or 0
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
