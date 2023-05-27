pub mod mesh;
mod shaders;
mod uniform;

use crate::game::world::World;
use crate::game::transform::Transform3D;
use crate::global_data::GlobalData;
use glium::Surface;
use self::shaders::ShaderProgramContainer;
use crate::game::player::player_projection_matrix_3D;
use uniform::{GlobalVertexBlock, GlobalFragmentBlock, UniformBlock};
use uniform::glsl_conversion::ToStd140;
use crate::options::AsVector;

pub struct Renderer {
    shaders: ShaderProgramContainer
}
impl Renderer {
    pub fn new(display: &glium::Display) -> Self {
        Self {
            shaders: ShaderProgramContainer::new(display)
        }
    }

    pub fn render_frame(&self, display: &glium::Display, world: &World, global_data: &mut GlobalData) {
        let mut target = display.draw();
        target.clear_color_and_depth(
            (0.0, 0.0, 1.0, 1.0),
            1.0
        );
    
        for mesh in &world.static_scene {
            let to_world_transform = Transform3D::IDENTITY.as_matrix();
            let to_view_transform = world.player.get_camera_trs_matrix().inverse() * to_world_transform;
            let to_clip_transform = player_projection_matrix_3D(global_data) * to_view_transform;
            let normal_matrix = to_world_transform.point_transform_to_normal_transform();
            
            let vertex_block = GlobalVertexBlock {
                to_world_transform: to_world_transform.std140(),
                to_view_transform: to_view_transform.std140(),
                to_clip_transform: to_clip_transform.std140(),
                normal_matrix: normal_matrix.std140()
            };
            let vertex_block_buffer = vertex_block.get_glium_uniform_buffer(display);
            let fragment_block = GlobalFragmentBlock {
                light_position: world.player.get_camera_world_position().std140(),
                light_color: global_data.options.dev.light.light_color.as_vector().std140(),
                light_ambient_color: global_data.options.dev.light.ambient_color.as_vector().std140(),
                light_linear_attenuation: global_data.options.dev.light.linear_attenuation.std140(),
                light_quadratic_attenuation: global_data.options.dev.light.quadratic_attenuation.std140()
            };
            let fragment_block_buffer = fragment_block.get_glium_uniform_buffer(display);

            let albedo_color: [f32; 3] = [1.0, 0.0, 0.0];
            let uniforms = glium::uniform! {
                vertex_uniforms: &vertex_block_buffer,
                fragment_uniforms: &fragment_block_buffer,
                albedo: albedo_color
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
