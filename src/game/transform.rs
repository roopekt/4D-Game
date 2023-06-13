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

//to clear the confusion of glam expecting column major order, meaning transpose would need to be done when calling Mat3::from_cols_array
macro_rules! matrix3x3 {
    (
        $e00:expr, $e01:expr, $e02:expr,
        $e10:expr, $e11:expr, $e12:expr,
        $e20:expr, $e21:expr, $e22:expr
    ) => {
        Mat3 {
            x_axis: Vec3::new($e00, $e10, $e20),
            y_axis: Vec3::new($e01, $e11, $e21),
            z_axis: Vec3::new($e02, $e12, $e22)
        }
    };
}
macro_rules! matrix4x4 {
    (
        $e00:expr, $e01:expr, $e02:expr, $e03:expr,
        $e10:expr, $e11:expr, $e12:expr, $e13:expr,
        $e20:expr, $e21:expr, $e22:expr, $e23:expr,
        $e30:expr, $e31:expr, $e32:expr, $e33:expr
    ) => {
        Mat4 {
            x_axis: Vec4::new($e00, $e10, $e20, $e30),
            y_axis: Vec4::new($e01, $e11, $e21, $e31),
            z_axis: Vec4::new($e02, $e12, $e22, $e32),
            w_axis: Vec4::new($e03, $e13, $e23, $e33),
        }
    };
}
pub(crate) use {matrix3x3, matrix4x4};
