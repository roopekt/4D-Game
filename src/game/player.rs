use super::transform::{Transform3D, AffineTransform3D, matrix3x3, Transform4D, AffineTransform4D, matrix4x4, rotation};
use crate::events::input::InputHandler;
use crate::global_data::GlobalData;
use glam::{Vec2, Vec3, Vec4, Mat3, Mat4};
use std::f32::consts::TAU;
use std::f32;
use glium::glutin::event::{VirtualKeyCode, MouseButton};

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
                position: Vec3::Y * 1.0,
                ..Transform3D::default()
            },
            look_direction: Vec2::ZERO
        }
    }

    pub fn update(&mut self, delta_time: f32, input: &InputHandler, global_data: &mut GlobalData) {
        //linear movement
        let mut pos_delta = Vec3::ZERO;
        if input.keyboard_is_pressed(&VirtualKeyCode::A     ) { pos_delta += Vec3::NEG_X };
        if input.keyboard_is_pressed(&VirtualKeyCode::D     ) { pos_delta += Vec3::X     };
        if input.keyboard_is_pressed(&VirtualKeyCode::LShift) { pos_delta += Vec3::NEG_Y };
        if input.keyboard_is_pressed(&VirtualKeyCode::Space ) { pos_delta += Vec3::Y     };
        if input.keyboard_is_pressed(&VirtualKeyCode::S     ) { pos_delta += Vec3::NEG_Z };
        if input.keyboard_is_pressed(&VirtualKeyCode::W     ) { pos_delta += Vec3::Z     };
        pos_delta = self.transform.orientation * pos_delta;
        self.transform.position += pos_delta * delta_time * global_data.options.dev.player.walking_speed;
        
        //rotation
        self.look_direction -= input.mouse_delta() * global_data.options.user.input.mouse_sensitivity;
        self.look_direction.x = self.look_direction.x.rem_euclid(TAU);//keep within reasonable range to prevent precision issues
        self.look_direction.y = self.look_direction.y.clamp(-TAU / 4.0, TAU / 4.0);
        self.transform.orientation = rotation::around_y(self.look_direction.x);
        self.relative_camera_transform.orientation = rotation::around_x(self.look_direction.y);
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

pub struct Player4D {
    pub transform: Transform4D,
    pub relative_camera_transform: Transform4D,
    pub tilt: f32
}
impl Player4D {
    pub fn new(_global_data: &GlobalData) -> Self {
        Self {
            transform: Transform4D::IDENTITY,
            relative_camera_transform: Transform4D {
                position: Vec4::Z * 1.0,
                ..Transform4D::default()
            },
            tilt: 0.0
        }
    }

    pub fn update(&mut self, delta_time: f32, input: &InputHandler, global_data: &mut GlobalData) {
        //linear movement
        let mut pos_delta = Vec4::ZERO;
        if input.keyboard_is_pressed(&VirtualKeyCode::Q     ) { pos_delta += Vec4::NEG_X };
        if input.keyboard_is_pressed(&VirtualKeyCode::E     ) { pos_delta += Vec4::X     };
        if input.keyboard_is_pressed(&VirtualKeyCode::A     ) { pos_delta += Vec4::NEG_Y };
        if input.keyboard_is_pressed(&VirtualKeyCode::D     ) { pos_delta += Vec4::Y     };
        if input.keyboard_is_pressed(&VirtualKeyCode::LShift) { pos_delta += Vec4::NEG_Z };
        if input.keyboard_is_pressed(&VirtualKeyCode::Space ) { pos_delta += Vec4::Z     };
        if input.keyboard_is_pressed(&VirtualKeyCode::S     ) { pos_delta += Vec4::NEG_W };
        if input.keyboard_is_pressed(&VirtualKeyCode::W     ) { pos_delta += Vec4::W     };
        pos_delta = self.transform.orientation * pos_delta;
        self.transform.position += pos_delta * delta_time * global_data.options.dev.player.walking_speed;

        //rotation
        //probably suffers from drifting
        let look_delta = -input.mouse_delta() * global_data.options.user.input.mouse_sensitivity;
        let is_left_button_pressed = input.mouse_is_pressed(&MouseButton::Left);
        let plane_delta = if is_left_button_pressed {
            rotation::around_xz(look_delta.x)
        }
        else {
            rotation::around_zw(look_delta.x) *
            rotation::around_yz(look_delta.y)
        };
        self.transform.orientation = 
            self.transform.orientation *
            plane_delta;
        if is_left_button_pressed {
            self.tilt -= look_delta.y;
            self.tilt = self.tilt.clamp(-TAU / 4.0, TAU / 4.0);
            self.relative_camera_transform.orientation = rotation::around_xy(self.tilt);
        }
    }

    pub fn get_trs_matrix(&self) -> AffineTransform4D {
        self.transform.into()
    }

    pub fn get_camera_trs_matrix(&self) -> AffineTransform4D {
        self.transform.as_matrix_ignore_scale() * self.relative_camera_transform.into()
    }

    pub fn get_camera_world_position(&self) -> Vec4 {
        &self.transform.as_matrix_ignore_scale() * &self.relative_camera_transform.position
    }

    pub fn get_camera_world_orientation(&self) -> Mat4 {
        self.transform.orientation * self.relative_camera_transform.orientation
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
        linear_transform: matrix3x3![
            y/aspect, 0.0, 0.0,
            0.0,      y,   0.0,
            0.0,      0.0, (near+far)/(far-near)
        ],
        translation: Vec3::new(0.0, 0.0, -(2.0*near*far)/(far-near))
    }
}

//affine transformation, so doesn't give depth divider
pub fn player_projection_matrix_4D(global_data: &GlobalData) -> AffineTransform4D {
    let z = 1.0 / f32::tan(global_data.options.dev.camera.fov.to_radians() * 0.5);
    let aspect = global_data.aspect_ratio();
    let near = global_data.options.dev.camera.near_plane;
    let far = global_data.options.dev.camera.far_plane;

    AffineTransform4D {
        linear_transform: matrix4x4![
            1.0, 0.0,      0.0, 0.0,
            0.0, z/aspect, 0.0, 0.0,
            0.0, 0.0,      z,   0.0,
            0.0, 0.0,      0.0, (near+far)/(far-near)
        ],
        translation: Vec4::new(0.0, 0.0, 0.0, -(2.0*near*far)/(far-near))
    }
}