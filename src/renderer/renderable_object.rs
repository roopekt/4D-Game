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
    pub fn render<A: glium::Surface, B: glium::Surface>(&self, targets: &mut ObjectDrawTargets<'_, A, B>, context: &ObjectDrawContext3D) {
        let to_world_transform = self.transform;
        let to_view_transform = context.inverse_camera_trs_matrix * to_world_transform;
        let to_clip_transform = context.projection_matrix * to_view_transform;
        let normal_matrix = to_world_transform.point_transform_to_normal_transform();
        
        let vertex_block = GlobalVertexBlock3D {
            to_world_transform: to_world_transform.std140(),
            to_view_transform: to_view_transform.std140(),
            to_clip_transform: to_clip_transform.std140(),
            normal_matrix: normal_matrix.std140()
        };
        let vertex_block_buffer = vertex_block.get_glium_uniform_buffer(context.display);

        let (surface_program_id, skeleton_program_id) = match context.visual_mode {
            VisualMode::Normal3D => (M::PROGRAM_IDS.normal_3D, M::PROGRAM_IDS.normal_3D_skeleton),
            VisualMode::Degenerate3D => (M::PROGRAM_IDS.degenerate_3D, M::PROGRAM_IDS.degenerate_3D_skeleton),
            VisualMode::Combined3D => panic!("Cannot handle {:?}. Please render in separate passes.", context.visual_mode),
            VisualMode::Degenerate4D => panic!("Cannot handle {:?}. Please use the 4D pipeline. ", context.visual_mode)
        };
        let surface_program  = context.shaders.get_program(surface_program_id);
        let skeleton_program = context.shaders.get_program(skeleton_program_id);

        //surface
        self.material.draw_mesh_3D(
            targets.surface_target,
            &self.mesh.vertices,
            &self.mesh.indeces,
            surface_program,
            &vertex_block_buffer,
            &context.fragment_block_buffer,
            &context.surface_glium_draw_parameters
        ).unwrap();

        //skeleton
        self.material.draw_mesh_3D(
            targets.skeleton_target,
            &self.mesh.vertices,
            &self.mesh.skeleton_indeces,
            skeleton_program,
            &vertex_block_buffer,
            &context.fragment_block_buffer,
            &context.skeleton_glium_draw_parameters
        ).unwrap();
    }
}
impl<M: Material> RenderableObject4D<M> {
    pub fn render<A: glium::Surface, B: glium::Surface>(&self, targets: &mut ObjectDrawTargets<'_, A, B>, context: &ObjectDrawContext4D) {
            let to_world_transform = self.transform;
            let to_view_transform = context.inverse_camera_trs_matrix * to_world_transform;
            let to_clip_transform = context.projection_matrix * to_view_transform;
            let normal_matrix = to_world_transform.point_transform_to_normal_transform();
            
            let vertex_block = GlobalVertexBlock4D {
                to_world_transform: to_world_transform.std140(),
                to_view_transform: to_view_transform.std140(),
                to_clip_transform: to_clip_transform.std140(),
                normal_matrix: normal_matrix.std140()
            };
            let vertex_block_buffer = vertex_block.get_glium_uniform_buffer(context.display);

            let surface_program = context.shaders.get_program(M::PROGRAM_IDS.degenerate_4D);
            let skeleton_program = context.shaders.get_program(M::PROGRAM_IDS.degenerate_4D_skeleton);

            //surface
            self.material.draw_mesh_4D(
                targets.surface_target,
                &self.mesh.vertices,
                &self.mesh.indeces,
                surface_program,
                &vertex_block_buffer,
                &context.fragment_block_buffer,
                &context.surface_glium_draw_parameters
            ).unwrap();

            //skeleton
            self.material.draw_mesh_4D(
                targets.skeleton_target,
                &self.mesh.vertices,
                &self.mesh.skeleton_indeces,
                skeleton_program,
                &vertex_block_buffer,
                &context.fragment_block_buffer,
                &context.skeleton_glium_draw_parameters
            ).unwrap();
    }
}

pub struct ObjectDrawContext3D<'a> {
    pub display: &'a glium::Display,
    pub shaders: &'a ShaderProgramContainer,
    pub inverse_camera_trs_matrix: AffineTransform3D,
    pub projection_matrix: AffineTransform3D,
    pub fragment_block_buffer: glium::uniforms::UniformBuffer<GlobalFragmentBlock3D>,
    pub surface_glium_draw_parameters: glium::DrawParameters<'a>,
    pub skeleton_glium_draw_parameters: glium::DrawParameters<'a>,
    pub visual_mode: VisualMode,
    pub _global_data: &'a GlobalData
}
pub struct ObjectDrawContext4D<'a> {
    pub display: &'a glium::Display,
    pub shaders: &'a ShaderProgramContainer,
    pub inverse_camera_trs_matrix: AffineTransform4D,
    pub projection_matrix: AffineTransform4D,
    pub fragment_block_buffer: glium::uniforms::UniformBuffer<GlobalFragmentBlock4D>,
    pub surface_glium_draw_parameters: glium::DrawParameters<'a>,
    pub skeleton_glium_draw_parameters: glium::DrawParameters<'a>,
    pub _global_data: &'a GlobalData
}

pub struct ObjectDrawTargets<'a, A: glium::Surface, B: glium::Surface> {
    pub surface_target:  &'a mut A,
    pub skeleton_target: &'a mut B
}
