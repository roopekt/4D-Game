use super::player::Player3D;
use crate::global_data::GlobalData;
use crate::renderer::mesh;
use std::time::Instant;
use glam::{Mat3, Vec3, Mat4, Vec4};
use std::vec::Vec;
use super::transform::{Transform3D, Transform4D, matrix3x3, matrix4x4};
use crate::renderer::renderable_object::{RenderableObject3D, RenderableObject4D};
use crate::renderer::shading::materials;

pub struct World3D {
    pub last_update_time: Instant,
    pub player: Player3D,
    pub static_scene: Vec<RenderableObject3D<materials::SingleColorMaterial>>
}
impl World3D {
    pub fn new(global_data: &GlobalData, display: &glium::Display) -> Self {
        Self {
            last_update_time: Instant::now(),
            player: Player3D::new(global_data),
            static_scene: get_static_scene_objects_3D(display)
        }
    }
}
pub struct World4D {
    pub last_update_time: Instant,
    pub player: Player3D,
    pub static_scene: Vec<RenderableObject4D<materials::SingleColorMaterial>>
}
impl World4D {
    pub fn new(global_data: &GlobalData, display: &glium::Display) -> Self {
        Self {
            last_update_time: Instant::now(),
            player: Player3D::new(global_data),
            static_scene: get_static_scene_objects_4D(display)
        }
    }
}

fn get_static_scene_objects_3D(display: &glium::Display) -> Vec<RenderableObject3D<materials::SingleColorMaterial>> {
    let mut objects: Vec<RenderableObject3D<materials::SingleColorMaterial>> = Vec::new();

    //floor
    objects.push(RenderableObject3D {
        transform: Transform3D {
            scale: Vec3::splat(100.0),
            orientation: matrix3x3![
                1.0, 0.0, 0.0,
                0.0, 0.0, 1.0,
                0.0, 1.0, 0.0
            ],
            ..Default::default()
        }.into(),
        mesh: mesh::primitives::quad_3D().upload_static(display),
        material: materials::SingleColorMaterial { albedo_color: Vec3::new(1.0, 1.0, 1.0) }
    });

    //cube
    objects.push(RenderableObject3D {
        transform: Transform3D {
            position: Vec3::new(0.0, 1.0, 3.0),
            ..Default::default()
        }.into(),
        mesh: mesh::primitives::cube_3D().upload_static(display),
        material: materials::SingleColorMaterial { albedo_color: Vec3::new(1.0, 0.0, 0.0) }
    });

    objects
}
fn get_static_scene_objects_4D(display: &glium::Display) -> Vec<RenderableObject4D<materials::SingleColorMaterial>> {
    let mut objects: Vec<RenderableObject4D<materials::SingleColorMaterial>> = Vec::new();

    //floor
    objects.push(RenderableObject4D {
        transform: Transform4D {
            scale: Vec4::splat(100.0),
            orientation: matrix4x4![
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 0.0
            ],
            ..Default::default()
        }.into(),
        mesh: mesh::primitives::cube_4D().upload_static(display),
        material: materials::SingleColorMaterial { albedo_color: Vec3::new(1.0, 1.0, 1.0) }
    });

    //tesseract
    objects.push(RenderableObject4D {
        transform: Transform4D {
            position: Vec4::new(0.0, 1.0, 3.0, 0.0),
            ..Default::default()
        }.into(),
        mesh: mesh::primitives::tesseract_4D().upload_static(display),
        material: materials::SingleColorMaterial { albedo_color: Vec3::new(1.0, 0.0, 0.0) }
    });

    objects
}
