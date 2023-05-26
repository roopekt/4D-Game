pub mod mesh;
mod shaders;

use crate::game::world::World;
use crate::game::transform::{Transform3D, MatrixTransform3D};
use crate::global_data::GlobalData;
use glium::{Surface, uniform};
use self::shaders::ShaderProgramContainer;
use crate::game::player::player_projection_matrix_3D;
use glam::f32::Vec3;

pub struct Renderer {
    shaders: ShaderProgramContainer,
    display: glium::Display
}
impl Renderer {
    pub fn new(display: glium::Display) -> Self {
        Self {
            shaders: ShaderProgramContainer::new(&display),
            display: display
        }
    }

    pub fn render_frame(&self, world: &World, global_data: &mut GlobalData) {
        let mut target = self.display.draw();
        target.clear_color_and_depth(
            (0.0, 0.0, 1.0, 1.0),
            1.0
        );
    
        for mesh in &world.static_scene {
            let world_transform: MatrixTransform3D = Transform3D::IDENTITY.into();
            let view_transform = world.player.get_camera_trs_matrix().inverse() * world_transform;
            let clip_transform = player_projection_matrix_3D(global_data) * view_transform;
            let normal_matrix = world_transform.point_transform_to_normal_transform();

            let albedo_color: [f32; 3] = [1.0, 0.0, 0.0];
            // let light_position: [f32; 3] = [0.0, 2.0, 0.0];
            let light_position = world.player.get_camera_world_position().to_array();

            let uniforms = uniform! {
                to_world_matrix: world_transform.linear_transform.to_cols_array_2d(),
                to_world_translation: world_transform.translation.to_array(),
                to_view_matrix: view_transform.linear_transform.to_cols_array_2d(),
                to_view_translation: view_transform.translation.to_array(),
                to_clip_matrix: clip_transform.linear_transform.to_cols_array_2d(),
                to_clip_translation: clip_transform.translation.to_array(),
                normal_matrix: normal_matrix.to_cols_array_2d(),

                albedo: albedo_color,

                light_position: light_position,
                light_color: global_data.options.dev.light.light_color,
                ambient_light_color: global_data.options.dev.light.ambient_color,
                light_linear_attenuation: global_data.options.dev.light.linear_attenuation,
                light_quadratic_attenuation: global_data.options.dev.light.quadratic_attenuation
            };

            let draw_parameters = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            };

            target.draw(
                &mesh.vertices,
                &mesh.indeces,
                &self.shaders.default,
                &uniforms,
                &draw_parameters
            ).unwrap();
        }
    
        target.finish().unwrap();
    }
}
