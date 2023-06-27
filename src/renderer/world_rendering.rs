use crate::game::world::{World3D, World4D};
use crate::game::player;
use crate::global_data::{GlobalData, VisualMode};
use super::Renderer;
use super::shading::shaders::ShaderProgramContainer;
use super::shading::uniform::{GlobalFragmentBlock3D, GlobalFragmentBlock4D, UniformBlock};
use super::shading::glsl_conversion::ToStd140;
use crate::options::AsVector;
use super::renderable_object::{ObjectDrawContext3D, ObjectDrawContext4D};

impl Renderer<'_> {
    pub fn render_objects_3D(
        &mut self,
        target: &mut glium::Frame,
        display: &glium::Display,
        world: &World3D,
        global_data: &GlobalData)
    {
        let inverse_camera_trs_matrix = world.player.get_camera_trs_matrix().inverse();
        let projection_matrix = player::player_projection_matrix_3D(global_data);

        let fragment_block = GlobalFragmentBlock3D {
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


        if global_data.visual_mode == VisualMode::Combined3D {
            let mut object_draw_context = ObjectDrawContext3D {
                display,
                inverse_camera_trs_matrix,
                projection_matrix,
                fragment_block_buffer,
                glium_draw_parameters,
                visual_mode: VisualMode::Normal3D,
                _global_data: global_data
            };
            render_objects_simple_visual_mode_3D(world, target, &object_draw_context, &self.shader_programs);

            object_draw_context.visual_mode = VisualMode::Degenerate3D;
            self.alternate_target.setup_for_rendering(display);
            self.alternate_target.with_frame_buffer_mut(|frame_buffer|
                render_objects_simple_visual_mode_3D(world, frame_buffer, &object_draw_context, &self.shader_programs)
            );

            self.alternate_target.blend_onto(target, &self.shader_programs,
                global_data.options.user.graphics.combined_render_degenerate_strength);
            self.draw_vertical_line(target);
        }
        else {
            let object_draw_context = ObjectDrawContext3D {
                display,
                inverse_camera_trs_matrix,
                projection_matrix,
                fragment_block_buffer,
                glium_draw_parameters,
                visual_mode: global_data.visual_mode,
                _global_data: global_data
            };
            render_objects_simple_visual_mode_3D(world, target, &object_draw_context, &self.shader_programs);
        }
    }
    pub fn render_objects_4D(
        &mut self,
        target: &mut glium::Frame,
        display: &glium::Display,
        world: &World4D,
        global_data: &GlobalData)
    {
        let inverse_camera_trs_matrix = world.player.get_camera_trs_matrix().inverse();
        let projection_matrix = player::player_projection_matrix_4D(global_data);

        let fragment_block = GlobalFragmentBlock4D {
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

        let object_draw_context = ObjectDrawContext4D {
            display,
            inverse_camera_trs_matrix,
            projection_matrix,
            fragment_block_buffer,
            glium_draw_parameters,
            _global_data: global_data
        };

        render_objects_simple_visual_mode_4D(world, target, &object_draw_context, &self.shader_programs);
    }
}

fn render_objects_simple_visual_mode_3D<T: glium::Surface>(world: &World3D, target: &mut T, context: &ObjectDrawContext3D, shaders: &ShaderProgramContainer) {
    for object in &world.static_scene {
        object.render(target, context, shaders);
    }
}
fn render_objects_simple_visual_mode_4D<T: glium::Surface>(world: &World4D, target: &mut T, context: &ObjectDrawContext4D, shaders: &ShaderProgramContainer) {
    for object in &world.static_scene {
        object.render(target, context, shaders);
    }
}