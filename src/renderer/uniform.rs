pub mod glsl_conversion;

use glsl_conversion::Std140AffineTransform3D;
use std140;

#[std140::repr_std140]
#[derive(Debug, Clone, Copy)]
pub struct GlobalVertexBlock {
    pub to_world_transform: Std140AffineTransform3D,
    pub to_view_transform: Std140AffineTransform3D,
    pub to_clip_transform: Std140AffineTransform3D,
    pub normal_matrix: std140::mat3x3
}

#[std140::repr_std140]
#[derive(Debug, Clone, Copy)]
pub struct GlobalFragmentBlock {
    pub light_position: std140::vec3,
    pub light_color: std140::vec3,
    pub light_ambient_color: std140::vec3,
    pub light_linear_attenuation: std140::float,
    pub light_quadratic_attenuation: std140::float
}

pub trait UniformBlock {
    fn get_glium_uniform_buffer(self, display: &glium::Display) -> glium::uniforms::UniformBuffer<Self> where Self: std::marker::Copy {
        glium::uniforms::UniformBuffer::new(display, self).unwrap()
    }
}
impl UniformBlock for GlobalVertexBlock {}
impl UniformBlock for GlobalFragmentBlock {}

impl glium::uniforms::UniformBlock for GlobalVertexBlock {
    fn matches(_layout: &glium::program::BlockLayout, _base_offset: usize) -> Result<(), glium::uniforms::LayoutMismatchError> {
        Ok(())
    }
    fn build_layout(_base_offset: usize) -> glium::program::BlockLayout {
        panic!("Unexpected call");
    }
}
impl glium::uniforms::UniformBlock for GlobalFragmentBlock {
    fn matches(_layout: &glium::program::BlockLayout, _base_offset: usize) -> Result<(), glium::uniforms::LayoutMismatchError> {
        Ok(())
    }
    fn build_layout(_base_offset: usize) -> glium::program::BlockLayout {
        panic!("Unexpected call");
    }
}
