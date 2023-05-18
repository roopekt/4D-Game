use glam::{Mat4, Vec3};
use crate::global_data::GlobalData;

pub fn player_projetion_matrix(global_data: &GlobalData) -> Mat4 {
    Mat4::perspective_rh_gl(
        global_data.options.dev.graphics.fov,
        global_data.aspect_ratio(),
        global_data.options.dev.graphics.near_plane,
        global_data.options.dev.graphics.far_plane
    ) * Mat4::from_scale(Vec3::NEG_ONE)
}
