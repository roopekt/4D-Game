use super::player::Player;
use crate::global_data::GlobalData;
use crate::renderer::mesh;
use std::time::Instant;
use glam::{Mat3, Vec3, Quat};
use std::vec::Vec;
use super::transform::Transform3D;
use crate::renderer::renderable_object::RenderableObject3D;
use crate::renderer::shading::materials;

pub struct World {
    pub last_update_time: Instant,
    pub player: Player,
    pub static_scene: Vec<RenderableObject3D<materials::SingleColorMaterial3D>>
}
impl World {
    pub fn new(global_data: &GlobalData, display: &glium::Display) -> Self {
        World {
            last_update_time: Instant::now(),
            player: Player::new(global_data),
            static_scene: get_static_scene_objects(display)
        }
    }
}

fn get_static_scene_objects(display: &glium::Display) -> Vec<RenderableObject3D<materials::SingleColorMaterial3D>> {
    let mut objects: Vec<RenderableObject3D<materials::SingleColorMaterial3D>> = Vec::new();

    //floor
    objects.push(RenderableObject3D {
        transform: Transform3D {
            scale: Vec3::splat(100.0),
            orientation: Mat3::from_quat(Quat::from_rotation_arc_colinear(Vec3::Z, Vec3::Y)),
            ..Default::default()
        }.into(),
        mesh: mesh::primitives::quad_3D().upload_static(display),
        material: materials::SingleColorMaterial3D { albedo_color: Vec3::new(1.0, 1.0, 1.0) }
    });

    //cube
    objects.push(RenderableObject3D {
        transform: Transform3D {
            position: Vec3::new(0.0, 1.0, 3.0),
            ..Default::default()
        }.into(),
        mesh: mesh::primitives::cube_3D().upload_static(display),
        material: materials::SingleColorMaterial3D { albedo_color: Vec3::new(1.0, 0.0, 0.0) }
    });

    objects
}
