pub mod renderable_object;
pub mod mesh;
pub mod shading;
pub mod text_rendering;

use crate::game::world::World;
use crate::game::player::player_projection_matrix_3D;
use crate::game::transform::AffineTransform3D;
use crate::global_data::{GlobalData, VisualMode};
use glium::{Surface, framebuffer, texture};
use shading::abstract_material::Material;
use shading::materials;
use shading::shaders::ShaderProgramContainer;
use shading::uniform::{GlobalVertexBlock3D, GlobalFragmentBlock3D, UniformBlock};
use shading::glsl_conversion::ToStd140;
use crate::options::AsVector;
use self::renderable_object::RenderableObject3D;
use crate::info_screen::render_info_screen;

pub struct Renderer<'a> {
    shader_programs: ShaderProgramContainer,
    text_renderer: text_rendering::TextRenderer<'a>,
    alternate_target: AlternateTarget,
    BLIT_QUAD: mesh::StaticUploadedMesh3D,
    HORIZONTAL_LINE: mesh::StaticUploadedMesh3D
}
impl<'a> Renderer<'a> {
    pub fn new(display: &glium::Display, global_data: &GlobalData) -> Self {
        Self {
            shader_programs: ShaderProgramContainer::new(display),
            text_renderer: text_rendering::TextRenderer::new(display, global_data),
            alternate_target: AlternateTarget::build(display),
            BLIT_QUAD: mesh::primitives::blit_quad().upload_static(display),
            HORIZONTAL_LINE: mesh::primitives::horizontal_line().upload_static_with_topology(display, glium::index::PrimitiveType::LinesList)
        }
    }

    pub fn render_frame(&mut self, display: &glium::Display, world: &World, global_data: &mut GlobalData) {
        let mut target = display.draw();
        target.clear_color_and_depth(
            (0.0, 0.0, 1.0, 1.0),
            1.0
        );

        self.render_objects(&mut target, display, world, global_data);
        if global_data.info_screen_visible {
            render_info_screen(&mut target, display, &mut self.text_renderer, world, global_data);
        }

        target.finish().unwrap();
    }

    fn render_objects(
        &mut self,
        target: &mut glium::Frame,
        display: &glium::Display,
        world: &World,
        global_data: &GlobalData)
    {
        let inverse_camera_trs_matrix = world.player.get_camera_trs_matrix().inverse();
        let projection_matrix = player_projection_matrix_3D(global_data);

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
            let mut object_draw_parameters = ObjectDrawParameters {
                display,
                inverse_camera_trs_matrix,
                projection_matrix,
                fragment_block_buffer,
                glium_draw_parameters,
                visual_mode: VisualMode::Normal3D,
                render_to_alternate: false,
                _global_data: global_data
            };
            self.render_objects_simple_visual_mode(world, target, &object_draw_parameters);

            self.setup_alternate_target(display);
            object_draw_parameters.visual_mode = VisualMode::Degenerate3D;
            object_draw_parameters.render_to_alternate = true;
            self.render_objects_simple_visual_mode(world, target, &object_draw_parameters);

            self.blend_alternate_target_onto_main_target(target, global_data.options.user.graphics.combined_render_degenerate_strength);
            self.draw_horizontal_line(target);
        }
        else {
            let object_draw_parameters = ObjectDrawParameters {
                display,
                inverse_camera_trs_matrix,
                projection_matrix,
                fragment_block_buffer,
                glium_draw_parameters,
                visual_mode: global_data.visual_mode,
                render_to_alternate: false,
                _global_data: global_data
            };
            self.render_objects_simple_visual_mode(world, target, &object_draw_parameters);
        }
    }

    fn render_objects_simple_visual_mode<T: glium::Surface>(&mut self, world: &World, target: &mut T, params: &ObjectDrawParameters) {
        for object in &world.static_scene {
            self.render_object(object, target, params);
        }
    }

    fn render_object<M: Material, T: glium::Surface>(&mut self, object: &RenderableObject3D<M>, target: &mut T, params: &ObjectDrawParameters) {
        let to_world_transform = object.transform;
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
            VisualMode::Combined3D => panic!("Cannot handle {:?}. Please render in separate passes.", {VisualMode::Combined3D})
        };
        let program = self.shader_programs.get_program(program_id);

        if params.render_to_alternate {
            self.alternate_target.with_frame_buffer_mut(|alternate_target| {
                object.material.draw_mesh_3D(
                    alternate_target,
                    &object.mesh.vertices,
                    &object.mesh.indeces,
                    program,
                    &vertex_block_buffer,
                    &params.fragment_block_buffer,
                    &params.glium_draw_parameters
                ).unwrap();
            });
        }
        else {
            object.material.draw_mesh_3D(
                target,
                &object.mesh.vertices,
                &object.mesh.indeces,
                program,
                &vertex_block_buffer,
                &params.fragment_block_buffer,
                &params.glium_draw_parameters
            ).unwrap();
        }
    }

    fn setup_alternate_target(&mut self, display: &glium::Display) {
        if self.alternate_target.get_dimensions() != display.get_framebuffer_dimensions() {
            self.alternate_target = AlternateTarget::build(display);
        }

        self.alternate_target.with_frame_buffer_mut(|frame_buffer| {
            frame_buffer.clear_color_and_depth(
                (0.0, 0.0, 1.0, 1.0),
                1.0
            );
        });
    }
    
    fn blend_alternate_target_onto_main_target(&self, main_target: &mut glium::Frame, degenerate_strength: f32) {
        let blit_material = materials::BlitMaterial {
            texture: self.alternate_target.borrow_color_texture()
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

        main_target.draw(
            &self.BLIT_QUAD.vertices,
            &self.BLIT_QUAD.indeces,
            &self.shader_programs.get_program(materials::BlitMaterial::PROGRAM_IDS.normal_3D),
            &blit_material.get_uniforms(),
            &draw_parameters
        ).unwrap();
    }

    fn draw_horizontal_line(&self, target: &mut glium::Frame) {
        let material = materials::SingleColorScreenSpaceMaterial {
            color: glam::Vec3::new(0.0, 0.0, 0.0)
        };
        let draw_parameters = glium::DrawParameters { ..Default::default() };

        target.draw(
            &self.HORIZONTAL_LINE.vertices,
            &self.HORIZONTAL_LINE.indeces,
            &self.shader_programs.get_program(materials::SingleColorScreenSpaceMaterial::PROGRAM_IDS.normal_3D),
            &material.get_uniforms(),
            &draw_parameters
        ).unwrap();
    }
}

#[ouroboros::self_referencing]
struct AlternateTarget {
    pub depth_texture: texture::DepthTexture2d,
    pub color_texture: texture::Texture2d,

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
}

struct ObjectDrawParameters<'a> {
    pub display: &'a glium::Display,
    pub inverse_camera_trs_matrix: AffineTransform3D,
    pub projection_matrix: AffineTransform3D,
    pub fragment_block_buffer: glium::uniforms::UniformBuffer<GlobalFragmentBlock3D>,
    pub glium_draw_parameters: glium::DrawParameters<'a>,
    pub visual_mode: VisualMode,
    pub render_to_alternate: bool,
    pub _global_data: &'a GlobalData
}
