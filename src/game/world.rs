use super::player::Player;
use crate::global_data::GlobalData;
use crate::renderer::mesh;
use std::time::Instant;
use glam::{Mat3, Vec3, Quat};
use rand::{rngs::SmallRng, SeedableRng, Rng};
use std::vec::Vec;
use super::transform::Transform3D;
use crate::renderer::renderable_object::RenderableObject;
use crate::renderer::shading::materials;

pub struct World {
    pub last_update_time: Instant,
    pub player: Player,
    pub static_scene: Vec<RenderableObject<materials::SingleColorMaterial3D>>,
    pub white_cube: RenderableObject<materials::WhiteMaterial3D>
}
impl World {
    pub fn new(global_data: &GlobalData, display: &glium::Display) -> Self {
        let white_cube = RenderableObject {
            transform: Transform3D{position: Vec3::Y * 2.0, ..Default::default()}.into(),
            mesh: mesh::primitives::cube().upload_static(display),
            material: materials::WhiteMaterial3D{}
        };

        World {
            last_update_time: Instant::now(),
            player: Player::new(global_data),
            static_scene: get_static_scene_objects(display),
            white_cube: white_cube
        }
    }
}

fn get_static_scene_objects(display: &glium::Display) -> Vec<RenderableObject<materials::SingleColorMaterial3D>> {
    let mut objects: Vec<RenderableObject<materials::SingleColorMaterial3D>> = Vec::new();
    let mut rng = SmallRng::from_entropy();

    let floor_trs = Transform3D {
        scale: Vec3::splat(100.0),
        orientation: Mat3::from_quat(Quat::from_rotation_arc_colinear(Vec3::Z, Vec3::Y)),
        ..Default::default()
    };
    objects.push(RenderableObject {
        transform: floor_trs.into(),
        mesh: mesh::primitives::quad().upload_static(display),
        material: materials::SingleColorMaterial3D { albedo_color: random_color(&mut rng) }
    });

    for _ in 0..15 {
        const SPAWN_RADIUS: f32 = 7.0;
        let position = Vec3::new(random_float(&mut rng) * SPAWN_RADIUS, 0.3, random_float(&mut rng) * SPAWN_RADIUS);
        let orientation = Mat3::from_quat(Quat::from_xyzw(random_float(&mut rng), random_float(&mut rng), random_float(&mut rng), random_float(&mut rng)).normalize());

        objects.push(RenderableObject {
            transform: Transform3D { position, orientation, ..Default::default() }.into(),
            mesh: mesh::primitives::cube().upload_static(display),
            material: materials::SingleColorMaterial3D { albedo_color: random_color(&mut rng) }
        });
    }

    objects
}

fn random_float<T: Rng>(rng: &mut T) -> f32
{
    rng.gen_range(-1.0..1.0)
}

fn random_color<T: Rng>(rng: &mut T) -> Vec3 {
    Vec3::new(
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0)
    )
}