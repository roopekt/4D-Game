use glium::Surface;
use super::shading::abstract_material::Material;
use super::shading::materials;
use super::shading::shaders::ShaderProgramContainer;
use super::mesh;
use glium::{texture, framebuffer};

#[ouroboros::self_referencing]
pub struct AlternateTarget {
    pub depth_texture: texture::DepthTexture2d,
    pub color_texture: texture::Texture2d,
    BLIT_QUAD: mesh::StaticUploadedMeshSimple,

    #[borrows(depth_texture, color_texture)]
    #[covariant]
    pub frame_buffer: framebuffer::SimpleFrameBuffer<'this>
}
impl AlternateTarget {
    pub fn build(display: &glium::Display) -> AlternateTarget {
        let dimensions = display.get_framebuffer_dimensions();

        AlternateTargetBuilder {
            depth_texture: texture::DepthTexture2d::empty_with_format(
                display,
                texture::DepthFormat::I24,
                texture::MipmapsOption::NoMipmap,
                dimensions.0,
                dimensions.1
            ).unwrap(),

            color_texture: texture::Texture2d::empty_with_format(
                display,
                texture::UncompressedFloatFormat::U8U8U8U8,
                texture::MipmapsOption::NoMipmap,
                dimensions.0,
                dimensions.1
            ).unwrap(),

            BLIT_QUAD: mesh::primitives::blit_quad().upload_static(display),

            frame_buffer_builder: |depth_texture: &texture::DepthTexture2d, color_texture: &texture::Texture2d|
                framebuffer::SimpleFrameBuffer::with_depth_buffer(
                    display,
                    color_texture,
                    depth_texture
            ).unwrap()
        }.build()
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        self.borrow_frame_buffer().get_dimensions()
    }

    pub fn setup_for_rendering(&mut self, display: &glium::Display) {
        if self.get_dimensions() != display.get_framebuffer_dimensions() {
            *self = Self::build(display);
        }

        self.with_frame_buffer_mut(|frame_buffer| {
            frame_buffer.clear_color_and_depth(
                (0.0, 0.0, 1.0, 1.0),
                1.0
            );
        });
    }
    
    pub fn blend_onto(&self, target: &mut glium::Frame, shaders: &ShaderProgramContainer, degenerate_strength: f32) {
        let blit_material = materials::BlitMaterial {
            texture: self.borrow_color_texture()
        };

        let draw_parameters = glium::DrawParameters {
            blend: glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::ConstantAlpha,
                    destination: glium::LinearBlendingFactor::OneMinusConstantAlpha
                },
                alpha: glium::BlendingFunction::Max,
                constant_value: (0.0, 0.0, 0.0, degenerate_strength)
            },
            ..Default::default()
        };

        let blit_quad = self.borrow_BLIT_QUAD();
        target.draw(
            &blit_quad.vertices,
            &blit_quad.indeces,
            &shaders.get_program(materials::BlitMaterial::PROGRAM_IDS.normal_3D),
            &blit_material.get_uniforms(),
            &draw_parameters
        ).unwrap();
    }
}