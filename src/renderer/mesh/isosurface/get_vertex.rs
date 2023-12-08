use glam::{Vec3, IVec3, Vec4, IVec4};
use super::sample_cloud::{SampleCloud3D, SampleCloud4D};
use crate::renderer::mesh::vertex::{CpuVertex3D, CpuVertex4D};

pub fn get_vertex_3D<F1, F2>(discrete_corner_vertex: IVec3, sample_cloud: &SampleCloud3D, function: &F1, gradient_func: &F2, voxel_width: f32, use_newton_method: bool) -> CpuVertex3D
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
pub fn get_vertex_4D<F1, F2>(discrete_corner_vertex: IVec4, sample_cloud: &SampleCloud4D, function: &F1, gradient_func: &F2, voxel_width: f32, use_newton_method: bool) -> CpuVertex4D
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