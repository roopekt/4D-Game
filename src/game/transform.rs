use glam::{Vec3, Mat3, Affine3A};
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Transform3D {
    pub position: Vec3,
    pub orientation: Mat3,//not a quaternion, because matrices are easier to generalize to 4D
    pub scale: Vec3
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

impl Default for Transform3D {
    fn default() -> Self {
        Self::IDENTITY
    }
}

//glam doesn't support 5x5 matrices, so custom affine transformations are used instead (in 3D aswell for similarity)
#[derive(Debug, Clone, Copy)]
pub struct AffineTransform3D {
    pub linear_transform: Mat3,
    pub translation: Vec3
}

impl AffineTransform3D {
    pub const IDENTITY: Self = Self {
        linear_transform: Mat3::IDENTITY,
        translation: Vec3::ZERO
    };

    pub fn from_transform3D_ignore_scale(transform: &Transform3D) -> Self {
        Self {
            linear_transform: transform.orientation,
            translation: transform.position
        }
    }

    pub fn inverse(&self) -> Self {
        let inverse_linear_transform = self.linear_transform.inverse();
        Self {
            linear_transform: inverse_linear_transform,
            translation: -inverse_linear_transform * self.translation
        }
    }

    pub fn point_transform_to_normal_transform(&self) -> Mat3 {
        self.linear_transform.inverse().transpose() //http://www.lighthouse3d.com/tutorials/glsl-12-tutorial/the-normal-matrix/
    }
}

impl From<&Transform3D> for AffineTransform3D {
    fn from(transform: &Transform3D) -> Self {
        Self {
            linear_transform: transform.orientation * Mat3::from_diagonal(transform.scale),
            translation: transform.position
        }
    }
}
impl From<Transform3D> for AffineTransform3D{
    fn from(transform: Transform3D) -> Self {
        (&transform).into()
    }
}
impl From<Affine3A> for AffineTransform3D{
    fn from(glam_matrix: Affine3A) -> Self {
        Self {
            linear_transform: glam_matrix.matrix3.into(),
            translation: glam_matrix.translation.into()
        }
    }
}
impl From<Mat3> for AffineTransform3D{
    fn from(matrix: Mat3) -> Self {
        Self {
            linear_transform: matrix,
            translation: Vec3::ZERO
        }
    }
}

impl Mul<&AffineTransform3D> for &AffineTransform3D {
    type Output = AffineTransform3D;

    fn mul(self, rhs: &AffineTransform3D) -> Self::Output {
        AffineTransform3D {
            linear_transform: self.linear_transform * rhs.linear_transform,
            translation: self.linear_transform * rhs.translation + self.translation
        }
    }
}
impl Mul<AffineTransform3D> for AffineTransform3D {
    type Output = AffineTransform3D;

    fn mul(self, rhs: AffineTransform3D) -> Self::Output {
        AffineTransform3D {
            linear_transform: self.linear_transform * rhs.linear_transform,
            translation: self.linear_transform * rhs.translation + self.translation
        }
    }
}
impl Mul<&Vec3> for &AffineTransform3D {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        self.linear_transform * *rhs + self.translation
    }
}
