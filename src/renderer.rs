pub mod renderable_object;
pub mod mesh;
pub mod shading;

use crate::game::world::World;
use crate::game::player::player_projection_matrix_3D;
use crate::game::transform::AffineTransform3D;
use crate::global_data::GlobalData;
use glium::Surface;
use glium::uniforms::UniformBuffer;
use shading::abstract_material::Material;
use shading::shaders::ShaderProgramContainer;
use shading::uniform::{GlobalVertexBlock, GlobalFragmentBlock, UniformBlock};
use shading::glsl_conversion::ToStd140;
use crate::options::AsVector;
use self::renderable_object::RenderableObject;

pub struct Renderer {
    shader_programs: ShaderProgramContainer
}
impl Renderer {
    pub fn new(display: &glium::Display) -> Self {
        Self {
            shader_programs: ShaderProgramContainer::new(display)
        }
    }

    pub fn render_frame(&self, display: &glium::Display, world: &World, global_data: &mut GlobalData) {
        let mut target = display.draw();
        target.clear_color_and_depth(
            (0.0, 0.0, 1.0, 1.0),
            1.0
        );

        let inverse_camera_trs_matrix = world.player.get_camera_trs_matrix().inverse();
        let projection_matrix = player_projection_matrix_3D(global_data);

        let fragment_block = GlobalFragmentBlock {
            light_position: world.player.get_camera_world_position().std140(),
            light_color: global_data.options.dev.light.light_color.as_vector().std140(),
            light_ambient_color: global_data.options.dev.light.ambient_color.as_vector().std140(),
            light_linear_attenuation: global_data.options.dev.light.linear_attenuation.std140(),
            light_quadratic_attenuation: global_data.options.dev.light.quadratic_attenuation.std140()
        };
        let fragment_block_buffer = fragment_block.get_glium_uniform_buffer(display);

        let glium_draw_parameters = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let object_draw_parameters = ObjectDrawParameters {
            display: display,
            inverse_camera_trs_matrix: inverse_camera_trs_matrix,
            projection_matrix: projection_matrix,
            fragment_block_buffer: fragment_block_buffer,
            glium_draw_parameters: glium_draw_parameters
        };

        self.render_objects(world, &mut target, &object_draw_parameters);
    
        target.finish().unwrap();
    }

    fn render_objects(&self, world: &World, target: &mut glium::Frame, params: &ObjectDrawParameters) {
        for object in &world.static_scene {
            self.render_object(object, target, params);
        }
    }

    fn render_object<M: Material>(&self, object: &RenderableObject<M>, target: &mut glium::Frame, params: &ObjectDrawParameters) {
        let to_world_transform = object.transform;
        let to_view_transform = params.inverse_camera_trs_matrix * to_world_transform;
        let to_clip_transform = params.projection_matrix * to_view_transform;
        let normal_matrix = to_world_transform.point_transform_to_normal_transform();
        
        let vertex_block = GlobalVertexBlock {
            to_world_transform: to_world_transform.std140(),
            to_view_transform: to_view_transform.std140(),
            to_clip_transform: to_clip_transform.std140(),
            normal_matrix: normal_matrix.std140()
        };
        let vertex_block_buffer = vertex_block.get_glium_uniform_buffer(params.display);

        object.material.draw_mesh(
            target,
            &object.mesh.vertices,
            &object.mesh.indeces,
            &self.shader_programs,
            &vertex_block_buffer,
            &params.fragment_block_buffer,
            &params.glium_draw_parameters
        ).unwrap();
    }
}

struct ObjectDrawParameters<'a> {
    pub display: &'a glium::Display,
    pub inverse_camera_trs_matrix: AffineTransform3D,
    pub projection_matrix: AffineTransform3D,
    pub fragment_block_buffer: UniformBuffer<GlobalFragmentBlock>,
    pub glium_draw_parameters: glium::DrawParameters<'a>
}
