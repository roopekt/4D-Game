use super::shading::abstract_material::Material;
use super::mesh;
use crate::game::transform;

use crate::game::transform::{AffineTransform3D, AffineTransform4D};
use crate::global_data::{GlobalData, VisualMode};
use super::shading::shaders::ShaderProgramContainer;
use super::shading::uniform::{GlobalVertexBlock3D, GlobalFragmentBlock3D, GlobalVertexBlock4D, GlobalFragmentBlock4D, UniformBlock};
use super::shading::glsl_conversion::ToStd140;

pub struct RenderableObject3D<M: Material> {
    pub transform: transform::AffineTransform3D,
    pub mesh: mesh::StaticUploadedMesh3D,
    pub material: M
}
pub struct RenderableObject4D<M: Material> {
    pub transform: transform::AffineTransform4D,
    pub mesh: mesh::StaticUploadedMesh4D,
    pub material: M
}

impl<M: Material> RenderableObject3D<M> {
    pub fn render<T: glium::Surface>(&self, target: &mut T, params: &ObjectDrawContext3D, shaders: &ShaderProgramContainer) {
        let to_world_transform = self.transform;
        let to_view_transform = params.inverse_camera_trs_matrix * to_world_transform;
        let to_clip_transform = params.projection_matrix * to_view_transform;
        let normal_matrix = to_world_transform.point_transform_to_normal_transform();
        
        let vertex_block = GlobalVertexBlock3D {
            to_world_transform: to_world_transform.std140(),
            to_view_transform: to_view_transform.std140(),
            to_clip_transform: to_clip_transform.std140(),
            normal_matrix: normal_matrix.std140()
        };
        let vertex_block_buffer = vertex_block.get_glium_uniform_buffer(params.display);

        let program_id = match params.visual_mode {
            VisualMode::Normal3D => M::PROGRAM_IDS.normal_3D,
            VisualMode::Degenerate3D => M::PROGRAM_IDS.degenerate_3D,
            VisualMode::Combined3D => panic!("Cannot handle {:?}. Please render in separate passes.", params.visual_mode),
            VisualMode::Degenerate4D => panic!("Cannot handle {:?}. Please use the 4D pipeline. ", params.visual_mode)
        };
        let program = shaders.get_program(program_id);

        self.material.draw_mesh_3D(
            target,
            &self.mesh.vertices,
            &self.mesh.indeces,
            program,
            &vertex_block_buffer,
            &params.fragment_block_buffer,
            &params.glium_draw_parameters
        ).unwrap();
    }
}
impl<M: Material> RenderableObject4D<M> {
    pub fn render<T: glium::Surface>(&self, target: &mut T, params: &ObjectDrawContext4D, shaders: &ShaderProgramContainer) {
            let to_world_transform = self.transform;
            let to_view_transform = params.inverse_camera_trs_matrix * to_world_transform;
            let to_clip_transform = params.projection_matrix * to_view_transform;
            let normal_matrix = to_world_transform.point_transform_to_normal_transform();
            
            let vertex_block = GlobalVertexBlock4D {
                to_world_transform: to_world_transform.std140(),
                to_view_transform: to_view_transform.std140(),
                to_clip_transform: to_clip_transform.std140(),
                normal_matrix: normal_matrix.std140()
            };
            let vertex_block_buffer = vertex_block.get_glium_uniform_buffer(params.display);

            let program = shaders.get_program(M::PROGRAM_IDS.degenerate_4D);

            self.material.draw_mesh_4D(
                target,
                &self.mesh.vertices,
                &self.mesh.indeces,
                program,
                &vertex_block_buffer,
                &params.fragment_block_buffer,
                &params.glium_draw_parameters
            ).unwrap();
    }
}

pub struct ObjectDrawContext3D<'a> {
    pub display: &'a glium::Display,
    pub inverse_camera_trs_matrix: AffineTransform3D,
    pub projection_matrix: AffineTransform3D,
    pub fragment_block_buffer: glium::uniforms::UniformBuffer<GlobalFragmentBlock3D>,
    pub glium_draw_parameters: glium::DrawParameters<'a>,
    pub visual_mode: VisualMode,
    pub _global_data: &'a GlobalData
}
pub struct ObjectDrawContext4D<'a> {
    pub display: &'a glium::Display,
    pub inverse_camera_trs_matrix: AffineTransform4D,
    pub projection_matrix: AffineTransform4D,
    pub fragment_block_buffer: glium::uniforms::UniformBuffer<GlobalFragmentBlock4D>,
    pub glium_draw_parameters: glium::DrawParameters<'a>,
    pub _global_data: &'a GlobalData
}
