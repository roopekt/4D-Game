use super::transform::{Transform3D, AffineTransform3D};
use crate::events::input::InputHandler;
use crate::global_data::GlobalData;
use glam::{Vec3, Vec2, Mat3};
use std::f32::consts::PI;
use std::f32;
use glium::glutin::event::VirtualKeyCode;

pub struct Player3D {
    pub transform: Transform3D,
    pub relative_camera_transform: Transform3D,
    pub look_direction: Vec2// (around y, around x), radians
}
impl Player3D {
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

    pub fn get_trs_matrix(&self) -> AffineTransform3D {
        self.transform.into()
    }

    pub fn get_camera_trs_matrix(&self) -> AffineTransform3D {
        self.transform.as_matrix_ignore_scale() * self.relative_camera_transform.into()
    }

    pub fn get_camera_world_position(&self) -> Vec3 {
        &self.transform.as_matrix_ignore_scale() * &self.relative_camera_transform.position
    }

    pub fn get_pretty_look_direction(&self) -> Vec2 {
        Vec2::new(
            self.look_direction.x.to_degrees(),
            self.look_direction.y.to_degrees()
        )
    }
}

//affine transformation, so doesn't give W (depth divider)
pub fn player_projection_matrix_3D(global_data: &GlobalData) -> AffineTransform3D {
    let y = 1.0 / f32::tan(global_data.options.dev.camera.fov.to_radians() * 0.5);
    let aspect = global_data.aspect_ratio();
    let near = global_data.options.dev.camera.near_plane;
    let far = global_data.options.dev.camera.far_plane;

    /* Parameters for computing depth can be solved with algebra:
    (a * near + b) / near = -1
    (a * far  + b) / far  = 1
    =>
    a = (near + far) / (far - near)
    b = -(2 * near * far) / (far - near) */

    AffineTransform3D {
        linear_transform: Mat3::from_cols_array(&[
            y/aspect, 0.0, 0.0,
            0.0,      y,   0.0,
            0.0,      0.0, (near+far)/(far-near)
        ]).transpose(),//because column major expected
        translation: Vec3::new(0.0, 0.0, -(2.0*near*far)/(far-near))
    }
}