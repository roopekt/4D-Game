// Reason for the weird macro based code is that the type returned by the glium::uniform! macro is huge
// (impractical to type out for each Material implementor) and depends on the input.
// Trait method implementations require the return type to be specified explicitly (impl Trait doesn't work either),
// so the value must be used on site in case of draw_mesh. This is why it isn't just a get_uniforms, but also handles rendering.
// However, an individual Material implementor can reasonably define an uniform getter using impl Trait, and a macro can then implement draw_mesh.
macro_rules! implement_material_draw { ($get_uniforms_func:expr) => {
    fn draw_mesh_3D<'a, 'b, T, V, I>(
        &self,
        target: &mut T,
        vertices: V,
        indeces: I,
        program: &glium::Program,
        vertex_block: &glium::uniforms::UniformBuffer<crate::renderer::shading::uniform::GlobalVertexBlock3D>,
        fragment_block: &glium::uniforms::UniformBuffer<crate::renderer::shading::uniform::GlobalFragmentBlock3D>,
        draw_parameters: &glium::DrawParameters<'_>)
        -> Result<(), glium::DrawError>
        where T: glium::Surface, V: glium::vertex::MultiVerticesSource<'b>, I: Into<glium::index::IndicesSource<'a>>
    {
        let uniforms = $get_uniforms_func(self);
        let uniforms = uniforms.add("vertex_uniforms", vertex_block);
        let uniforms = uniforms.add("fragment_uniforms", fragment_block);

        target.draw(vertices, indeces, program, &uniforms, draw_parameters)
    }

    fn draw_mesh_4D<'a, 'b, T, V, I>(
        &self,
        target: &mut T,
        vertices: V,
        indeces: I,
        program: &glium::Program,
        vertex_block: &glium::uniforms::UniformBuffer<crate::renderer::shading::uniform::GlobalVertexBlock4D>,
        fragment_block: &glium::uniforms::UniformBuffer<crate::renderer::shading::uniform::GlobalFragmentBlock4D>,
        draw_parameters: &glium::DrawParameters<'_>)
        -> Result<(), glium::DrawError>
        where T: glium::Surface, V: glium::vertex::MultiVerticesSource<'b>, I: Into<glium::index::IndicesSource<'a>>
    {
        let uniforms = $get_uniforms_func(self);
        let uniforms = uniforms.add("vertex_uniforms", vertex_block);
        let uniforms = uniforms.add("fragment_uniforms", fragment_block);

        target.draw(vertices, indeces, program, &uniforms, draw_parameters)
    }
}}
macro_rules! any_uniforms_storage {() => { glium::uniforms::UniformsStorage<impl glium::uniforms::AsUniformValue + '_, impl glium::uniforms::Uniforms + '_> }}
pub(crate) use implement_material_draw;
pub(crate) use any_uniforms_storage;

pub trait Material {
    const PROGRAM_DESCRIPTORS: ProgramDescriptorGroup;
    const PROGRAM_IDS: ShaderProgramIdGroup;

    fn draw_mesh_3D<'a, 'b, T, V, I>(
        &self,
        target: &mut T,
        vertices: V,
        indeces: I,
        program: &glium::Program,
        vertex_block: &glium::uniforms::UniformBuffer<crate::renderer::shading::uniform::GlobalVertexBlock3D>,
        fragment_block: &glium::uniforms::UniformBuffer<crate::renderer::shading::uniform::GlobalFragmentBlock3D>,
        draw_parameters: &glium::DrawParameters<'_>)
        -> Result<(), glium::DrawError>
        where T: glium::Surface, V: glium::vertex::MultiVerticesSource<'b>, I: Into<glium::index::IndicesSource<'a>>;

    fn draw_mesh_4D<'a, 'b, T, V, I>(
        &self,
        target: &mut T,
        vertices: V,
        indeces: I,
        program: &glium::Program,
        vertex_block: &glium::uniforms::UniformBuffer<crate::renderer::shading::uniform::GlobalVertexBlock4D>,
        fragment_block: &glium::uniforms::UniformBuffer<crate::renderer::shading::uniform::GlobalFragmentBlock4D>,
        draw_parameters: &glium::DrawParameters<'_>)
        -> Result<(), glium::DrawError>
        where T: glium::Surface, V: glium::vertex::MultiVerticesSource<'b>, I: Into<glium::index::IndicesSource<'a>>;
}

pub type ShaderProgramId = usize;
#[derive(Debug, Copy, Clone)]
pub struct ShaderProgramIdGroup {
    pub normal_3D: ShaderProgramId,
    pub normal_3D_skeleton: ShaderProgramId,
    pub degenerate_3D: ShaderProgramId,
    pub degenerate_3D_skeleton: ShaderProgramId,
    pub degenerate_4D: ShaderProgramId,
    pub degenerate_4D_skeleton: ShaderProgramId
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramDescriptorGroup {
    pub normal_3D: ProgramDescriptor,
    pub normal_3D_skeleton: ProgramDescriptor,
    pub degenerate_3D: ProgramDescriptor,
    pub degenerate_3D_skeleton: ProgramDescriptor,
    pub degenerate_4D: ProgramDescriptor,
    pub degenerate_4D_skeleton: ProgramDescriptor
}
impl ProgramDescriptorGroup {
    /// assigns same descriptor to all fields
    pub const fn new_trivial(descriptor: ProgramDescriptor) -> Self {
        Self {
            normal_3D: descriptor,
            normal_3D_skeleton: descriptor,
            degenerate_3D: descriptor,
            degenerate_3D_skeleton: descriptor,
            degenerate_4D: descriptor,
            degenerate_4D_skeleton: descriptor
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ProgramDescriptor {
    pub vertex_shader_path: &'static str,
    pub fragment_shader_path: &'static str,
    pub geometry_shader_path: Option<&'static str>
}
impl ProgramDescriptor {
    pub const fn new(vertex_shader_path: &'static str, fragment_shader_path: &'static str) -> Self {
        Self {
            vertex_shader_path: vertex_shader_path,
            fragment_shader_path: fragment_shader_path,
            geometry_shader_path: None
        }
    }

    pub const fn new_with_geometry(vertex_shader_path: &'static str, fragment_shader_path: &'static str, geometry_shader_path: &'static str) -> Self {
        Self {
            vertex_shader_path: vertex_shader_path,
            fragment_shader_path: fragment_shader_path,
            geometry_shader_path: Some(geometry_shader_path)
        }
    }

    pub const fn is_equal(&self, other: &Self) -> bool {
        const_str::equal!(self.vertex_shader_path, other.vertex_shader_path) &&
        const_str::equal!(self.fragment_shader_path, other.fragment_shader_path)
    }
}
