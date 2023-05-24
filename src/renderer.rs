pub mod mesh;
mod shaders;

use crate::game::world::World;
use crate::game::transform::{Transform3D, MatrixTransform3D};
use crate::global_data::GlobalData;
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
    
        let affine_mvp_matrix = get_affine_mvp_matrix(
            &world.player.camera_trs_matrix(),
            &(world.player.projection_matrix)(global_data),
            &Transform3D::IDENTITY.into()
        );
        let affine_mv_matrix = get_affine_mv_matrix(
            &world.player.camera_trs_matrix(),
            &Transform3D::IDENTITY.into()
        );
        let uniforms = uniform! {
            mvp_matrix: affine_mvp_matrix.linear_transform.to_cols_array_2d(),
            mvp_translation: affine_mvp_matrix.translation.to_array(),
            mv_matrix: affine_mv_matrix.linear_transform.to_cols_array_2d(),
            mv_translation: affine_mv_matrix.translation.to_array()
        };

        let draw_parameters = glium::DrawParameters {
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
            &draw_parameters
        ).unwrap();
    
        target.finish().unwrap();
    }
}

fn get_affine_mvp_matrix(camera_trs: &MatrixTransform3D, camera_proj: &MatrixTransform3D, object_trs: &MatrixTransform3D) -> MatrixTransform3D {
    *camera_proj * camera_trs.inverse() * *object_trs
}

fn get_affine_mv_matrix(camera_trs: &MatrixTransform3D, object_trs: &MatrixTransform3D) -> MatrixTransform3D {
    camera_trs.inverse() * *object_trs
}
