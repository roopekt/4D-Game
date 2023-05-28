use super::uniform::{GlobalVertexBlock, GlobalFragmentBlock};
use super::shaders::ShaderProgramContainer;

macro_rules! implement_material_draw { ($get_uniforms_func:expr) => {
    fn draw_mesh<'a, 'b, V, I>(
        &self,
        target: &mut glium::Frame,
        vertices: V,
        indeces: I,
        programs: &crate::renderer::shaders::ShaderProgramContainer,
        vertex_block: glium::uniforms::UniformBuffer<crate::renderer::uniform::GlobalVertexBlock>,
        fragment_block: glium::uniforms::UniformBuffer<crate::renderer::uniform::GlobalFragmentBlock>,
        draw_parameters: &glium::DrawParameters<'_>)
        -> Result<(), glium::DrawError>
        where V: glium::vertex::MultiVerticesSource<'b>, I: Into<glium::index::IndicesSource<'a>>
    {
        let uniforms = $get_uniforms_func(self);
        let uniforms = uniforms.add("vertex_uniforms", &vertex_block);
        let uniforms = uniforms.add("fragment_uniforms", &fragment_block);

        let program = programs.get_program(Self::PROGRAM_ID);

        target.draw(vertices, indeces, program, &uniforms, draw_parameters)
    }
}}
macro_rules! any_uniforms_storage {() => { glium::uniforms::UniformsStorage<impl glium::uniforms::AsUniformValue, impl glium::uniforms::Uniforms> }}
pub(crate) use implement_material_draw;
pub(crate) use any_uniforms_storage;

pub trait Material {
    const PROGRAM_DESCRIPTOR: ProgramDescriptor;
    const PROGRAM_ID: ShaderProgramId;

    fn draw_mesh<'a, 'b, V, I>(
        &self,
        target: &mut glium::Frame,
        vertices: V,
        indeces: I,
        programs: &ShaderProgramContainer,
        vertex_block: glium::uniforms::UniformBuffer<GlobalVertexBlock>,
        fragment_block: glium::uniforms::UniformBuffer<GlobalFragmentBlock>,
        draw_parameters: &glium::DrawParameters<'_>)
        -> Result<(), glium::DrawError>
        where V: glium::vertex::MultiVerticesSource<'b>, I: Into<glium::index::IndicesSource<'a>>;
}

pub type ShaderProgramId = usize;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ProgramDescriptor {
    pub vertex_shader_path: &'static str,
    pub fragment_shader_path: &'static str
}
impl ProgramDescriptor {
    pub const fn new(vertex_shader_path: &'static str, fragment_shader_path: &'static str) -> Self {
        Self {
            vertex_shader_path: vertex_shader_path,
            fragment_shader_path: fragment_shader_path
        }
    }

    pub const fn is_equal(&self, other: &Self) -> bool {
        const_str::equal!(self.vertex_shader_path, other.vertex_shader_path) &&
        const_str::equal!(self.fragment_shader_path, other.fragment_shader_path)
    }
}
