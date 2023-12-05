use super::player::{Player3D, Player4D};
use crate::global_data::GlobalData;
use crate::renderer::mesh;
use std::time::Instant;
use glam::{Mat3, Vec3, Mat4, Vec4};
use std::vec::Vec;
use super::transform::{Transform3D, Transform4D, rotation, switch_matrix3_columns, switch_matrix4_columns};
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
            orientation: switch_matrix3_columns(Mat3::IDENTITY, 1, 2),
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

    //torus
    let torus_major_radius = 0.5;
    let torus_minor_radius = 0.2;
    let torus = mesh::isosurface::get_connected_isosurface_3D(
        &|p| {
            //the closest point on the circle in the center of the torus
            let circle_point = p
                .reject_from(Vec3::Z)
                .try_normalize().unwrap_or(Vec3::X)
                * torus_major_radius;
            circle_point.distance(p) - torus_minor_radius
        },
        &|p| {
            let circle_point = p
                .reject_from(Vec3::Z)
                .try_normalize().unwrap_or(Vec3::X)
                * torus_major_radius;
            (p - circle_point).normalize_or_zero()
        },
        0.05,
        0.2,
        Vec3::X * torus_major_radius,
        Vec3::ZERO,
        true
    );
    objects.push(RenderableObject3D {
        transform: Transform3D {
            position: Vec3::new(0.0, 3.0, 3.0),
            ..Default::default()
        }.into(),
        mesh: torus.upload_static(display),
        material: materials::SingleColorMaterial { albedo_color: Vec3::new(0.0, 1.0, 0.0) }
    });

    const CUBE_COUNT: usize = 5;
    const SPHERE_COUNT: usize = 5;
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
    let sphere = mesh::primitives::sphere_3D(4, 1);
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
            orientation: switch_matrix4_columns(Mat4::IDENTITY, 2, 3),
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

    //torus
    let torus_major_radius = 0.5;
    let torus_minor_radius = 0.2;
    let torus = mesh::isosurface::get_connected_isosurface_4D(
        &|p| {
            //the closest point on the circle in the center of the torus
            let circle_point = p
                .reject_from(Vec4::W)
                .try_normalize().unwrap_or(Vec4::X)
                * torus_major_radius;
            circle_point.distance(p) - torus_minor_radius
        },
        &|p| {
            let circle_point = p
                .reject_from(Vec4::W)
                .try_normalize().unwrap_or(Vec4::X)
                * torus_major_radius;
            (p - circle_point).normalize_or_zero()
        },
        0.08,
        0.5,
        Vec4::X * torus_major_radius,
        Vec4::ZERO,
        true
    );
    objects.push(RenderableObject4D {
        transform: Transform4D {
            position: Vec4::new(0.0, 0.0, 3.0, 3.0),
            ..Default::default()
        }.into(),
        mesh: torus.upload_static(display),
        material: materials::SingleColorMaterial { albedo_color: Vec3::new(0.0, 1.0, 0.0) }
    });

    const TESSERACT_COUNT: usize = 20;
    const SPHERE_COUNT: usize = 20;
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
    let sphere = mesh::primitives::sphere_4D(4, 1);
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
