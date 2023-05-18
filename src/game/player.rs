use super::transform::Transform;
use crate::renderer::projection_matrices::player_projetion_matrix;
use crate::events::input::InputHandler;
use crate::global_data::GlobalData;
use glam::{Vec3, Vec2, Quat, Mat4};
use std::f32::consts::PI;
use glium::glutin::event::VirtualKeyCode;

pub struct Player {
    pub transform: Transform,
    pub camera_transform: Transform,
    pub projection_matrix: fn(&GlobalData) -> Mat4,
    pub look_direction: Vec2// (around y, around x), radians
}
impl Player {
    pub fn new(_global_data: &GlobalData) -> Self {
        Self {
            transform: Transform::IDENTITY,
            camera_transform: Transform {
                position: Vec3::Y * 0.2,
                ..Transform::default()
            },
            projection_matrix: player_projetion_matrix,
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
        self.transform.orientation = Quat::from_rotation_y(self.look_direction.x);
        self.camera_transform.orientation = Quat::from_rotation_x(self.look_direction.y);
    }

    pub fn trs_matrix(&self) -> Mat4 {
        self.transform.as_matrix()
    }

    pub fn camera_trs_matrix(&self) -> Mat4 {
        self.transform.as_matrix_ignore_scale() * self.camera_transform.as_matrix()
    }
}