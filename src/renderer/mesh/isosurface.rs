mod sample_cloud;

use glam::{Vec3, IVec3};
use super::Mesh3D;

pub fn get_connected_isosurface_3D(function: fn(Vec3) -> f32, voxel_width: f32, negative_point: Vec3, positive_point: Vec3) -> Mesh3D
{
    let normalized_function = |normalized_coordinate: IVec3| {
        function(voxel_width * normalized_coordinate.as_vec3())
    };

    todo!()
}