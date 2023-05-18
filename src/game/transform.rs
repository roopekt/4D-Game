use glam::{Vec3, Quat, Mat4};

pub struct Transform {
    pub position: Vec3,
    pub orientation: Quat,
    pub scale: Vec3
}

impl Transform {
    pub const IDENTITY: Self = Self {
        position: Vec3::ZERO,
        orientation: Quat::IDENTITY,
        scale: Vec3::ONE
    };

    pub fn as_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.orientation, self.position)
    }

    pub fn as_matrix_ignore_scale(&self) -> Mat4 {
        Mat4::from_rotation_translation(self.orientation, self.position)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}