use super::transform::{Transform3D, MatrixTransform3D};
use crate::events::input::InputHandler;
use crate::global_data::GlobalData;
use glam::{Vec3, Vec2, Mat3, Mat4, Affine3A};
use std::f32::consts::PI;
use glium::glutin::event::VirtualKeyCode;

pub struct Player {
    pub transform: Transform3D,
    pub relative_camera_transform: Transform3D,
    pub look_direction: Vec2// (around y, around x), radians
}
impl Player {
    pub fn new(_global_data: &GlobalData) -> Self {
        Self {
            transform: Transform3D::IDENTITY,
            relative_camera_transform: Transform3D {
                position: Vec3::Y * 0.2,
                ..Transform3D::default()
            },
            look_direction: Vec2::ZERO
        }
    }

    pub fn update(&mut self, delta_time: f32, input: &InputHandler, global_data: &mut GlobalData) {

        let mut pos_delta = Vec3::ZERO;
        if input.is_pressed(&VirtualKeyCode::A) { pos_delta += Vec3::NEG_X };
        if input.is_pressed(&VirtualKeyCode::D) { pos_delta += Vec3::X };
        if input.is_pressed(&VirtualKeyCode::S) { pos_delta += Vec3::NEG_Z };
        if input.is_pressed(&VirtualKeyCode::W) { pos_delta += Vec3::Z };
        if input.is_pressed(&VirtualKeyCode::LShift) { pos_delta += Vec3::NEG_Y };
        if input.is_pressed(&VirtualKeyCode::Space) { pos_delta += Vec3::Y };
        pos_delta = self.transform.orientation * pos_delta;
        self.transform.position += pos_delta * delta_time * global_data.options.dev.player.walking_speed;
        
        self.look_direction += input.mouse_delta() * global_data.options.user.input.mouse_sensitivity;
        self.look_direction.x = self.look_direction.x.rem_euclid(2.0 * PI);//keep within reasonable range to prevent precision issues
        self.look_direction.y = self.look_direction.y.clamp(-PI / 2.0, PI / 2.0);
        self.transform.orientation = Mat3::from_rotation_y(self.look_direction.x);
        self.relative_camera_transform.orientation = Mat3::from_rotation_x(self.look_direction.y);
    }

    pub fn get_trs_matrix(&self) -> MatrixTransform3D {
        self.transform.into()
    }

    pub fn get_camera_trs_matrix(&self) -> MatrixTransform3D {
        self.transform.as_matrix_ignore_scale() * self.relative_camera_transform.into()
    }

    pub fn get_camera_world_position(&self) -> Vec3 {
        &self.transform.as_matrix_ignore_scale() * &self.relative_camera_transform.position
    }
}

pub fn player_projection_matrix_3D(global_data: &GlobalData) -> MatrixTransform3D {
    let matrix_4x4 = Mat4::perspective_rh_gl(
        global_data.options.dev.camera.fov,
        global_data.aspect_ratio(),
        global_data.options.dev.camera.near_plane,
        global_data.options.dev.camera.far_plane
    ) * Mat4::from_scale(Vec3::NEG_ONE);
    Affine3A::from_mat4(matrix_4x4).into()
}