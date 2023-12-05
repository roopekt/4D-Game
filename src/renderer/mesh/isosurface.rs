mod sample_cloud;

use glam::{Vec3, IVec3, Vec4, IVec4};
use itertools::Itertools;
use self::sample_cloud::{SampleCloud3D, SampleCloud4D};
use super::{Mesh3D, Mesh4D, primitives, vertex::CpuVertex3D, vertex::CpuVertex4D};
use indexmap::IndexMap;

pub fn get_connected_isosurface_3D<F1, F2>(function: &F1, gradient: &F2, voxel_width: f32, skeleton_voxel_width: f32, negative_point: Vec3, positive_point: Vec3, use_newton_method: bool) -> Mesh3D
    where F1: Fn(Vec3) -> f32, F2: Fn(Vec3) -> Vec3
{
    let mut main_mesh = get_connected_isosurface_3D_no_skeleton(function, gradient,          voxel_width, negative_point, positive_point, use_newton_method);
    let skeleton_mesh = get_connected_isosurface_3D_no_skeleton(function, gradient, skeleton_voxel_width, negative_point, positive_point, use_newton_method);

    main_mesh.attach_skeleton(skeleton_mesh);
    main_mesh
}
pub fn get_connected_isosurface_4D<F1, F2>(function: &F1, gradient: &F2, voxel_width: f32, skeleton_voxel_width: f32, negative_point: Vec4, positive_point: Vec4, use_newton_method: bool) -> Mesh4D
    where F1: Fn(Vec4) -> f32, F2: Fn(Vec4) -> Vec4
{
    let mut main_mesh = get_connected_isosurface_4D_no_skeleton(function, gradient,          voxel_width, negative_point, positive_point, use_newton_method);
    let skeleton_mesh = get_connected_isosurface_4D_no_skeleton(function, gradient, skeleton_voxel_width, negative_point, positive_point, use_newton_method);

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
                get_vertex_3D(discrete_corner_vertex, &sample_cloud, &function, &gradient, voxel_width, use_newton_method)
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
pub fn get_connected_isosurface_4D_no_skeleton<F1, F2>(function: &F1, gradient: &F2, voxel_width: f32, negative_point: Vec4, positive_point: Vec4, use_newton_method: bool) -> Mesh4D
    where F1: Fn(Vec4) -> f32, F2: Fn(Vec4) -> Vec4
{
    let normalized_function = |normalized_coordinate: IVec4| {
        function(voxel_width * normalized_coordinate.as_vec4())
    };
    let normalized_negative_point = (negative_point / voxel_width).round().as_ivec4();
    let normalized_positive_point = (positive_point / voxel_width).round().as_ivec4();
    let sample_cloud = sample_cloud::SampleCloud4D::new(&normalized_function, normalized_negative_point, normalized_positive_point);

    let relative_cubes          = [0, 1, 2, 3].map(|axis_index| get_cube_4D_with_nth_axis_as_normal(axis_index, false));
    let relative_cubes_mirrored = [0, 1, 2, 3].map(|axis_index| get_cube_4D_with_nth_axis_as_normal(axis_index, true));

    let mut vertices = IndexMap::<IVec4, CpuVertex4D>::new();
    let mut tetrahedra = Vec::<[IVec4; 4]>::new();

    for border_pair in &sample_cloud.border_pairs {
        /*
        The surface of the mesh is built out of cubes, and adjacent cubes share a quadrilateral (a face).
        While smoothing, the vertices of the quadrilateral are moved independently, which means that they
        will not be on the same plane. This means that the surface of the quadrilateral is not uniquely
        determined, as there are two ways to cut a quadrilateral into triangles. If two adjacent cubes
        make the cut differently, the final mesh will have a visible hole. The below mirroring procedure
        should make sure that the cut is always made the same way.

        If smoothing was disabled, the vertices would lie on a square lattice. Each lattice point has
        a parity value, which is either Y or N. The parity values form a 4D chess board pattern, where two
        adjacent lattice points always have a different parity value. A quadrilateral is cut into triangles
        by a single diagonal. If a vertex lies on a lattice point with a parity value of N, it shall not be
        part of any diagonals. If the vertex has a parity value of Y, it shall instead be part of every
        quadrilateral's diagonal that the vertex is part of.

        Due to how a cube is cut into tetrahedra (see diagram: https://i.stack.imgur.com/kEkEA.gif),
        if one vertex of a cube has the correct parity value, all of its vertices have a correct parity value.
        One can see this by observing which vertices form the central tetrahedron of the cube, and remembering
        that adjacent lattice points always have a different parity value.

        The parity_flag variable represents the parity value of a single vertex (the one with biggest coordinates)
        of the cube in question. I have not checked whether false corresponds to N or Y, but it shouldn't matter.
        Depending on the value of parity_flag, a "normal" or a mirrored version of a cube is picked, such that
        the diagonals of the cube's faces don't violate the parity value of the vertex the flag corresponds to.
        If the parity value is correct for that vertex of the cube, it must be correct for the rest as well, as
        seen earlier. Thus, this procedure makes sure that all cubes added to the mesh respect the parity values
        of the lattice.

        Consider a quadrilateral shared by two adjacent cubes. The parity value of each vertex is same from the
        perspective of both cubes, as the parity value depends only on position. As adjacent lattice points always
        have a different parity value, exactly two of the quadrilateral's four vertices have a parity value of Y.
        The only legal way to cut the quadrilateral into triangles is to connect the two vertices with the value Y
        by a diagonal. As there is only one way to do the cut, both cubes will cut the quadrilateral the same way,
        leaving no holes.
         */
        let parity_flag = <[i32; 4]>::from(border_pair.A).iter()//is the number of odd components odd?
            .map(|c| c.rem_euclid(2))
            .sum::<i32>().rem_euclid(2) == 1;
        let cube_source = if parity_flag { &relative_cubes } else { &relative_cubes_mirrored };

        let (relative_vertices, relative_tetrahedra) = &cube_source[border_pair.axis_index];

        for relative_vertex in relative_vertices {
            let discrete_corner_vertex = *relative_vertex + border_pair.A;//the minimum coordinate corner of the cube in which the new vertex will be in
            vertices.entry(discrete_corner_vertex).or_insert_with(|| {
                get_vertex_4D(discrete_corner_vertex, &sample_cloud, &function, &gradient, voxel_width, use_newton_method)
            });
        }

        for relative_tetrahedron in relative_tetrahedra {
            tetrahedra.push(relative_tetrahedron.map(|relative_coord| relative_coord + border_pair.A));
        }
    };

    Mesh4D {
        indeces: tetrahedra.iter()
            .map(|&tetrahedron| tetrahedron.map(|i4| vertices.get_index_of(&i4).unwrap()))
            .collect_vec(),
        vertices: vertices.into_values().collect_vec(),
        skeleton_indeces: Vec::new()
    }
}

fn get_vertex_3D<F1, F2>(discrete_corner_vertex: IVec3, sample_cloud: &SampleCloud3D, function: &F1, gradient_func: &F2, voxel_width: f32, use_newton_method: bool) -> CpuVertex3D
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
            if (value_A > 0.0) != (value_B > 0.0) {
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

    assert!(position.is_finite());

    CpuVertex3D {
        position: position,
        normal: gradient_func(position).normalize()
    }
}
fn get_vertex_4D<F1, F2>(discrete_corner_vertex: IVec4, sample_cloud: &SampleCloud4D, function: &F1, gradient_func: &F2, voxel_width: f32, use_newton_method: bool) -> CpuVertex4D
    where F1: Fn(Vec4) -> f32, F2: Fn(Vec4) -> Vec4
{
    const EDGES_PER_AXIS: usize = 8;

    let min_corner = discrete_corner_vertex.as_vec4() * voxel_width;
    let max_corner = (discrete_corner_vertex + IVec4::ONE).as_vec4() * voxel_width;

    fn get_coordinate_given_edge(value_A: f32, value_B: f32, coordinate_A: f32, coordinate_B: f32) -> f32 {
        let t = -value_A / (value_B - value_A);//inverse lerp (with a target of 0)
        coordinate_A + (coordinate_B - coordinate_A) * t //lerp
    }
    let get_coordinate = |relative_edges: &[(IVec4, IVec4); EDGES_PER_AXIS], min_coordinate: f32, max_coordinate: f32| -> f32 {
        let mut coordinate: f32 = 0.0;
        let mut counter: u32 = 0;
        for (offset_A, offset_B) in relative_edges {
            let value_A = sample_cloud.sample_map[&(discrete_corner_vertex + *offset_A)];
            let value_B = sample_cloud.sample_map[&(discrete_corner_vertex + *offset_B)];
            if (value_A > 0.0) != (value_B > 0.0) {
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

    const RELATIVE_EDGES_X: [(IVec4, IVec4); EDGES_PER_AXIS] = [
        (IVec4::new(0,0,0,0), IVec4::new(1,0,0,0)),
        (IVec4::new(0,0,0,1), IVec4::new(1,0,0,1)),
        (IVec4::new(0,0,1,0), IVec4::new(1,0,1,0)),
        (IVec4::new(0,0,1,1), IVec4::new(1,0,1,0)),
        (IVec4::new(0,1,0,0), IVec4::new(1,1,0,0)),
        (IVec4::new(0,1,0,1), IVec4::new(1,1,0,1)),
        (IVec4::new(0,1,1,0), IVec4::new(1,1,1,0)),
        (IVec4::new(0,1,1,1), IVec4::new(1,1,1,0))
    ];
    const RELATIVE_EDGES_Y: [(IVec4, IVec4); EDGES_PER_AXIS] = [
        (IVec4::new(0,0,0,0), IVec4::new(0,1,0,0)),
        (IVec4::new(0,0,0,1), IVec4::new(0,1,0,1)),
        (IVec4::new(0,0,1,0), IVec4::new(0,1,1,0)),
        (IVec4::new(0,0,1,1), IVec4::new(0,1,1,0)),
        (IVec4::new(1,0,0,0), IVec4::new(1,1,0,0)),
        (IVec4::new(1,0,0,1), IVec4::new(1,1,0,1)),
        (IVec4::new(1,0,1,0), IVec4::new(1,1,1,0)),
        (IVec4::new(1,0,1,1), IVec4::new(1,1,1,0))
    ];
    const RELATIVE_EDGES_Z: [(IVec4, IVec4); EDGES_PER_AXIS] = [
        (IVec4::new(0,0,0,0), IVec4::new(0,0,1,0)),
        (IVec4::new(0,0,0,1), IVec4::new(0,0,1,1)),
        (IVec4::new(0,1,0,0), IVec4::new(0,1,1,0)),
        (IVec4::new(0,1,0,1), IVec4::new(0,1,1,0)),
        (IVec4::new(1,0,0,0), IVec4::new(1,0,1,0)),
        (IVec4::new(1,0,0,1), IVec4::new(1,0,1,1)),
        (IVec4::new(1,1,0,0), IVec4::new(1,1,1,0)),
        (IVec4::new(1,1,0,1), IVec4::new(1,1,1,0))
    ];
    const RELATIVE_EDGES_W: [(IVec4, IVec4); EDGES_PER_AXIS] = [
        (IVec4::new(0,0,0,0), IVec4::new(0,0,0,1)),
        (IVec4::new(0,0,1,0), IVec4::new(0,0,1,1)),
        (IVec4::new(0,1,0,0), IVec4::new(0,1,0,1)),
        (IVec4::new(0,1,1,0), IVec4::new(0,1,0,1)),
        (IVec4::new(1,0,0,0), IVec4::new(1,0,0,1)),
        (IVec4::new(1,0,1,0), IVec4::new(1,0,1,1)),
        (IVec4::new(1,1,0,0), IVec4::new(1,1,0,1)),
        (IVec4::new(1,1,1,0), IVec4::new(1,1,0,1))
    ];

    let mut position = Vec4::new(
        get_coordinate(&RELATIVE_EDGES_X, min_corner.x, max_corner.x),
        get_coordinate(&RELATIVE_EDGES_Y, min_corner.y, max_corner.y),
        get_coordinate(&RELATIVE_EDGES_Z, min_corner.z, max_corner.z),
        get_coordinate(&RELATIVE_EDGES_W, min_corner.w, max_corner.w)
    );

    //a single iteration of Newton's method
    if use_newton_method {
        let gradient = gradient_func(position);
        let gradient_inverse_length = gradient.length_recip();
        position = position - (gradient_inverse_length * gradient) * function(position) * gradient_inverse_length;
    }

    assert!(position.is_finite());

    CpuVertex4D {
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
fn get_cube_4D_with_nth_axis_as_normal(axis_index: usize, mirrored: bool) -> (Vec<IVec4>, Vec<[IVec4; 4]>) {
    let cube = primitives::cube_4D_discrete_vertices();
    
    let (coord_A, coord_B) = if mirrored { (-1, 0) } else { (0, -1) };

    let vertices = cube.0.iter()
        .map(|corner2| corner2.map(coord_A, coord_B))
        .map(|array3| IVec4::new(array3[0], array3[1], array3[2],  0))
        .map(|mut vec_w_normal| {
            (vec_w_normal[axis_index], vec_w_normal.w) = (vec_w_normal.w, vec_w_normal[axis_index]);
            vec_w_normal
        })
        .collect_vec();

    let tetrahedra = cube.1.iter()
        .map(|vertex_indeces| vertex_indeces.map(|i| vertices[i]))
        .collect_vec();

    (vertices, tetrahedra)
}
