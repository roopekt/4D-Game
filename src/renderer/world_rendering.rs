use glium::PolygonMode;
use crate::game::world::{World3D, World4D};
use crate::game::player;
use crate::global_data::{GlobalData, VisualMode};
use super::Renderer;
use super::shading::uniform::{GlobalFragmentBlock3D, GlobalFragmentBlock4D, UniformBlock};
use super::shading::glsl_conversion::ToStd140;
use crate::options::AsVector;
use super::renderable_object::{ObjectDrawContext3D, ObjectDrawContext4D, ObjectDrawTargets};

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

        let surface_glium_draw_parameters = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            polygon_mode: global_data.polygon_mode,
            line_width: if global_data.polygon_mode == PolygonMode::Line { Some(global_data.options.dev.debug.line_width) } else { None },
            point_size: if global_data.polygon_mode == PolygonMode::Point { Some(global_data.options.dev.debug.point_size) } else { None },
            .. Default::default()
        };
        let skeleton_glium_draw_parameters = glium::DrawParameters {
            point_size: Some(global_data.options.user.graphics.skeleton_width),
            ..Default::default()
        };


        if global_data.visual_mode == VisualMode::Combined3D {
            let mut object_draw_context = ObjectDrawContext3D {
                display,
                shaders: &self.shader_programs,
                inverse_camera_trs_matrix,
                projection_matrix,
                fragment_block_buffer,
                surface_glium_draw_parameters,
                skeleton_glium_draw_parameters,
                visual_mode: VisualMode::Normal3D,
                _global_data: global_data
            };
            self.skeleton_target.setup_for_rendering(display, (0.0, 0.0, 0.0, 0.0));
            self.skeleton_target.with_frame_buffer_mut(|skeleton_target|
                render_objects_simple_visual_mode_3D(world, target, skeleton_target, &object_draw_context)
            );
            self.skeleton_target.blit_onto_with_alpha(target, &self.shader_programs, &self.BLIT_QUAD);

            object_draw_context.visual_mode = VisualMode::Degenerate3D;
            self.skeleton_target.setup_for_rendering(display, (0.0, 0.0, 0.0, 0.0));
            self.alternate_target.setup_for_rendering(display, (0.0, 0.0, 1.0, 1.0));
            self.alternate_target.with_frame_buffer_mut(|alternate_target|
                self.skeleton_target.with_frame_buffer_mut(|skeleton_target|
                    render_objects_simple_visual_mode_3D(world, alternate_target, skeleton_target, &object_draw_context))
            );
            self.skeleton_target.blit_onto_with_alpha(target, &self.shader_programs, &self.BLIT_QUAD);

            self.alternate_target.blend_onto(target,
                global_data.options.user.graphics.combined_render_degenerate_strength,
                &self.shader_programs, &self.BLIT_QUAD);
            self.draw_vertical_line(target);
        }
        else {
            let object_draw_context = ObjectDrawContext3D {
                display,
                shaders: &self.shader_programs,
                inverse_camera_trs_matrix,
                projection_matrix,
                fragment_block_buffer,
                surface_glium_draw_parameters,
                skeleton_glium_draw_parameters,
                visual_mode: global_data.visual_mode,
                _global_data: global_data
            };

            self.skeleton_target.setup_for_rendering(display, (0.0, 0.0, 0.0, 0.0));
            self.skeleton_target.with_frame_buffer_mut(|skeleton_target|
                render_objects_simple_visual_mode_3D(world, target, skeleton_target, &object_draw_context)
            );
            self.skeleton_target.blit_onto_with_alpha(target, &self.shader_programs, &self.BLIT_QUAD);
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

        let surface_glium_draw_parameters = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            polygon_mode: global_data.polygon_mode,
            line_width: if global_data.polygon_mode == PolygonMode::Line { Some(global_data.options.dev.debug.line_width) } else { None },
            point_size: if global_data.polygon_mode == PolygonMode::Point { Some(global_data.options.dev.debug.point_size) } else { None },
            .. Default::default()
        };
        let skeleton_glium_draw_parameters = glium::DrawParameters {
            ..Default::default()
        };

        let object_draw_context = ObjectDrawContext4D {
            display,
            shaders: &self.shader_programs,
            inverse_camera_trs_matrix,
            projection_matrix,
            fragment_block_buffer,
            surface_glium_draw_parameters,
            skeleton_glium_draw_parameters,
            _global_data: global_data
        };

        self.skeleton_target.setup_for_rendering(display, (0.0, 0.0, 0.0, 0.0));
        self.skeleton_target.with_frame_buffer_mut(|skeleton_target|
            render_objects_simple_visual_mode_4D(world, target, skeleton_target, &object_draw_context)
        );
        self.skeleton_target.blit_onto_with_alpha(target, &self.shader_programs, &self.BLIT_QUAD);
    }
}

fn render_objects_simple_visual_mode_3D<A: glium::Surface, B: glium::Surface>(world: &World3D, surface_target: &mut A, skeleton_target: &mut B, context: &ObjectDrawContext3D) {
    let mut targets = ObjectDrawTargets { surface_target, skeleton_target };

    for object in &world.static_scene {
        object.render(&mut targets, context);
    }
}
fn render_objects_simple_visual_mode_4D<A: glium::Surface, B: glium::Surface>(world: &World4D, surface_target: &mut A, skeleton_target: &mut B, context: &ObjectDrawContext4D) {
    let mut targets = ObjectDrawTargets { surface_target, skeleton_target };

    for object in &world.static_scene {
        object.render(&mut targets, context);
    }
}