pub mod affine_transform;
pub use affine_transform::{AffineTransform3D, AffineTransform4D};

use glam::{Vec3, Mat3, Vec4, Mat4};

#[derive(Debug, Clone, Copy)]
pub struct Transform3D {
    pub position: Vec3,
    pub orientation: Mat3,//not a quaternion, because matrices are easier to generalize to 4D
    pub scale: Vec3
}
#[derive(Debug, Clone, Copy)]
pub struct Transform4D {
    pub position: Vec4,
    pub orientation: Mat4,
    pub scale: Vec4
}

impl Transform3D {
    pub const IDENTITY: Self = Self {
        position: Vec3::ZERO,
        orientation: Mat3::IDENTITY,
        scale: Vec3::ONE
    };

    pub fn as_matrix(&self) -> AffineTransform3D {
        self.into()
    }

    pub fn as_matrix_ignore_scale(&self) -> AffineTransform3D {
        AffineTransform3D::from_transform3D_ignore_scale(self)
    }
}
impl Transform4D {
    pub const IDENTITY: Self = Self {
        position: Vec4::ZERO,
        orientation: Mat4::IDENTITY,
        scale: Vec4::ONE
    };

    pub fn as_matrix(&self) -> AffineTransform4D {
        self.into()
    }

    pub fn as_matrix_ignore_scale(&self) -> AffineTransform4D {
        AffineTransform4D::from_transform4D_ignore_scale(self)
    }
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::IDENTITY
    }
}
impl Default for Transform4D {
    fn default() -> Self {
        Self::IDENTITY
    }
}
