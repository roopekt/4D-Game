use super::player::{Player3D, Player4D};
use crate::global_data::GlobalData;
use crate::renderer::mesh;
use std::time::Instant;
use glam::{Mat3, Vec3, Mat4, Vec4};
use std::vec::Vec;
use super::transform::{Transform3D, Transform4D, matrix3x3, matrix4x4, rotation};
use crate::renderer::renderable_object::{RenderableObject3D, RenderableObject4D};
use crate::renderer::shading::materials;
use rand::{rngs::SmallRng, SeedableRng, Rng};

pub struct Multiverse {
    pub world_3D: World3D,
    pub world_4D: World4D,
    pub last_update_time: Instant
}
impl Multiverse {
    pub fn new(global_data: &GlobalData, display: &glium::Display) -> Self {
        Self {
            world_3D: World3D::new(global_data, display),
            world_4D: World4D::new(global_data, display),
            last_update_time: Instant::now()
        }
    }
}

pub struct World3D {
    pub player: Player3D,
    pub static_scene: Vec<RenderableObject3D<materials::SingleColorMaterial>>
}
impl World3D {
    pub fn new(global_data: &GlobalData, display: &glium::Display) -> Self {
        Self {
            player: Player3D::new(global_data),
            static_scene: get_static_scene_objects_3D(display)
        }
    }
}
pub struct World4D {
    pub player: Player4D,
    pub static_scene: Vec<RenderableObject4D<materials::SingleColorMaterial>>
}
impl World4D {
    pub fn new(global_data: &GlobalData, display: &glium::Display) -> Self {
        Self {
            player: Player4D::new(global_data),
            static_scene: get_static_scene_objects_4D(display)
        }
    }
}

fn get_static_scene_objects_3D(display: &glium::Display) -> Vec<RenderableObject3D<materials::SingleColorMaterial>> {
    let mut objects: Vec<RenderableObject3D<materials::SingleColorMaterial>> = Vec::new();
    let mut rng = SmallRng::from_entropy();

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

    //big cube
    objects.push(RenderableObject3D {
        transform: Transform3D {
            position: Vec3::new(0.0, 1.0, 3.0),
            scale: Vec3::splat(1.5),
            ..Default::default()
        }.into(),
        mesh: mesh::primitives::cube_3D().upload_static(display),
        material: materials::SingleColorMaterial { albedo_color: Vec3::new(1.0, 0.0, 0.0) }
    });

    const CUBE_COUNT: usize = 7;
    const SPHERE_COUNT: usize = 7;
    const SPAWN_RADIUS: f32 = 7.0;

    //random cubes
    let cube = mesh::primitives::cube_3D();
    for _ in 0..CUBE_COUNT {
        let position = Vec3::new(rng.gen_range(-1.0..1.0) * SPAWN_RADIUS, 0.3, rng.gen_range(-1.0..1.0) * SPAWN_RADIUS);
        let orientation = rotation::random_3D_nonuniform(&mut rng);
        let color = Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));

        objects.push(RenderableObject3D {
            transform: Transform3D { position, orientation, ..Default::default() }.into(),
            mesh: cube.clone().upload_static(display),
            material: materials::SingleColorMaterial { albedo_color: color }
        });
    }

    //random spheres
    let sphere = mesh::primitives::sphere_3D(4);
    for _ in 0..SPHERE_COUNT {
        let position = Vec3::new(rng.gen_range(-1.0..1.0) * SPAWN_RADIUS, 0.3, rng.gen_range(-1.0..1.0) * SPAWN_RADIUS);
        let color = Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));

        objects.push(RenderableObject3D {
            transform: Transform3D { position, ..Default::default() }.into(),
            mesh: sphere.clone().upload_static(display),
            material: materials::SingleColorMaterial { albedo_color: color }
        });
    }

    objects
}
fn get_static_scene_objects_4D(display: &glium::Display) -> Vec<RenderableObject4D<materials::SingleColorMaterial>> {
    let mut objects: Vec<RenderableObject4D<materials::SingleColorMaterial>> = Vec::new();
    let mut rng = SmallRng::from_entropy();

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

    //big tesseract
    objects.push(RenderableObject4D {
        transform: Transform4D {
            position: Vec4::new(0.0, 0.0, 1.0, 3.0),
            scale: Vec4::splat(1.5),
            ..Default::default()
        }.into(),
        mesh: mesh::primitives::tesseract_4D().upload_static(display),
        material: materials::SingleColorMaterial { albedo_color: Vec3::new(1.0, 0.0, 0.0) }
    });

    const TESSERACT_COUNT: usize = 70;
    const SPHERE_COUNT: usize = 70;
    const SPAWN_RADIUS: f32 = 7.0;

    //random tesseracts
    let tesseract = mesh::primitives::tesseract_4D();
    for _ in 0..TESSERACT_COUNT {
        let position = Vec4::new(rng.gen_range(-1.0..1.0) * SPAWN_RADIUS, rng.gen_range(-1.0..1.0) * SPAWN_RADIUS, 0.3, rng.gen_range(-1.0..1.0) * SPAWN_RADIUS);
        let orientation = rotation::random_4D_nonuniform(&mut rng);
        let color = Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));

        objects.push(RenderableObject4D {
            transform: Transform4D { position, orientation, ..Default::default() }.into(),
            mesh: tesseract.clone().upload_static(display),
            material: materials::SingleColorMaterial { albedo_color: color }
        });
    }

    //random spheres
    let sphere = mesh::primitives::sphere_4D(4);
    for _ in 0..SPHERE_COUNT {
        let position = Vec4::new(rng.gen_range(-1.0..1.0) * SPAWN_RADIUS, rng.gen_range(-1.0..1.0) * SPAWN_RADIUS, 0.3, rng.gen_range(-1.0..1.0) * SPAWN_RADIUS);
        let color = Vec3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));

        objects.push(RenderableObject4D {
            transform: Transform4D { position, ..Default::default() }.into(),
            mesh: sphere.clone().upload_static(display),
            material: materials::SingleColorMaterial { albedo_color: color }
        });
    }

    objects
}
