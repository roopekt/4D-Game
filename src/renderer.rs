pub mod renderable_object;
pub mod mesh;
pub mod shading;
pub mod text_rendering;
mod render_target;
mod world_rendering;

use crate::game::world::Multiverse;
use crate::global_data::GlobalData;
use glium::Surface;
use shading::abstract_material::Material;
use shading::materials;
use shading::shaders::ShaderProgramContainer;
use crate::info_screen::render_info_screen;
use render_target::RenderTarget;

pub struct Renderer<'a> {
    shader_programs: ShaderProgramContainer,
    text_renderer: text_rendering::TextRenderer<'a>,
    alternate_target: RenderTarget,
    VERTICAL_LINE: mesh::StaticUploadedMeshSimple,
    BLIT_QUAD: mesh::StaticUploadedMeshSimple
}
impl<'a> Renderer<'a> {
    pub fn new(display: &glium::Display, global_data: &GlobalData) -> Self {
        Self {
            shader_programs: ShaderProgramContainer::new(display),
            text_renderer: text_rendering::TextRenderer::new(display, global_data),
            alternate_target: RenderTarget::build(display),
            VERTICAL_LINE: mesh::primitives::vertical_line().upload_static(display),
            BLIT_QUAD: mesh::primitives::blit_quad().upload_static(display)
        }
    }

    pub fn render_frame(&mut self, display: &glium::Display, multiverse: &Multiverse, global_data: &mut GlobalData) {
        let mut target = display.draw();
        target.clear_color_and_depth(
            (0.0, 0.0, 1.0, 1.0),
            1.0
        );

        self.render_objects(&mut target, display, multiverse, global_data);
        if global_data.info_screen_visible {
            render_info_screen(&mut target, display, &mut self.text_renderer, multiverse, global_data);
        }

        target.finish().unwrap();
    }

    fn render_objects(
        &mut self,
        target: &mut glium::Frame,
        display: &glium::Display,
        multiverse: &Multiverse,
        global_data: &GlobalData)
    {
        if global_data.is_4D_active() {
            self.render_objects_4D(target, display, &multiverse.world_4D, global_data);
        }
        else {
            self.render_objects_3D(target, display, &multiverse.world_3D, global_data);
        }
    }

    fn draw_vertical_line(&self, target: &mut glium::Frame) {
        let material = materials::SingleColorScreenSpaceMaterial {
            color: glam::Vec3::new(0.0, 0.0, 0.0)
        };
        let draw_parameters = glium::DrawParameters { ..Default::default() };

        target.draw(
            &self.VERTICAL_LINE.vertices,
            &self.VERTICAL_LINE.indeces,
            &self.shader_programs.get_program(materials::SingleColorScreenSpaceMaterial::PROGRAM_IDS.normal_3D),
            &material.get_uniforms(),
            &draw_parameters
        ).unwrap();
    }
}
