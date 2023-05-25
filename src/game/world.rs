use super::player::Player;
use crate::global_data::GlobalData;
use crate::renderer::mesh;
use std::time::Instant;
use glam::{Mat4, Vec3, Quat};
use rand::{rngs::SmallRng, SeedableRng, Rng};
use std::vec::Vec;

pub struct World {
    pub last_update_time: Instant,
    pub player: Player,
    pub static_scene: Vec<mesh::StaticUploadedMesh>
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

fn get_static_scene_objects(display: &glium::Display) -> Vec<mesh::StaticUploadedMesh> {
    let mut meshes: Vec<mesh::StaticUploadedMesh> = Vec::new();

    let floor_trs_matrix = Mat4::from_scale_rotation_translation(
        Vec3::splat(100.0),
        Quat::from_rotation_arc_colinear(Vec3::Z, Vec3::Y),
        Vec3::ZERO
    );
    meshes.push(mesh::primitives::quad().as_transformed(floor_trs_matrix).upload_static(display));

    let mut rng = SmallRng::from_entropy();
    for _ in 0..15 {
        let cube = mesh::primitives::cube();

        let rotation = Quat::from_xyzw(random_float(&mut rng), random_float(&mut rng), random_float(&mut rng), random_float(&mut rng)).normalize();

        const R: f32 = 7.0;
        let position = Vec3::new(random_float(&mut rng) * R, 0.3, random_float(&mut rng) * R);

        let transform_matrix = Mat4::from_rotation_translation(rotation, position);
        meshes.push(cube.as_transformed(transform_matrix).upload_static(display));
    }

    meshes
}

fn random_float<T>(rng: &mut T) -> f32
    where T: Rng
{
    rng.gen_range(-1.0..1.0)
}