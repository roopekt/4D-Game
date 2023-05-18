pub mod mesh;
pub mod projection_matrices;
mod shaders;

use crate::game::world::World;
use crate::global_data::GlobalData;
use glam::Mat4;
use glium::{Surface, uniform};
use self::shaders::ShaderProgramContainer;

pub struct Renderer {
    shaders: ShaderProgramContainer,
    display: glium::Display
}
impl Renderer {
    pub fn new(display: glium::Display) -> Self {
        Self {
            shaders: ShaderProgramContainer::new(&display),
            display: display
        }
    }

    pub fn render_frame(&self, world: &World, global_data: &mut GlobalData) {
        let mut target = self.display.draw();
        target.clear_color_and_depth(
            (0.0, 0.0, 1.0, 1.0),
            1.0
        );
    
        let uniforms = uniform! {
            matrix: get_gl_mvp_matrix(
                &world.player.camera_trs_matrix(),
                &(world.player.projection_matrix)(global_data),
                &Mat4::from_translation(glam::Vec3::Z)
            )
        };

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        target.draw(
            &world.scene_mesh.vertices,
            &world.scene_mesh.indeces,
            &self.shaders.simple,
            &uniforms,
            &params
        ).unwrap();
    
        target.finish().unwrap();
    }
}

fn get_gl_mvp_matrix(camera_trs: &Mat4, camera_proj: &Mat4, object_trs: &Mat4) -> [[f32; 4]; 4] {
    let matrix = *camera_proj * camera_trs.inverse() * *object_trs;
    matrix.to_cols_array_2d()
}
