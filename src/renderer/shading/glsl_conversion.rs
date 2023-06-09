use std140;
use glam;
use crate::game::transform::{AffineTransform3D, AffineTransform4D};

pub trait ToStd140<T> {
    fn std140(&self) -> T;
}

#[std140::repr_std140]
#[derive(Debug, Clone, Copy)]
pub struct Std140AffineTransform3D {
    pub matrix: std140::mat3x3,
    pub translation: std140::vec3
}
#[std140::repr_std140]
#[derive(Debug, Clone, Copy)]
pub struct Std140AffineTransform4D {
    pub matrix: std140::mat4x4,
    pub translation: std140::vec4
}

impl ToStd140<std140::float> for f32 {
    fn std140(&self) -> std140::float {
        std140::float(*self)
    }
}

impl ToStd140<std140::vec3> for glam::Vec3 {
    fn std140(&self) -> std140::vec3 {
        std140::vec3(self.x, self.y, self.z)
    }
}

impl ToStd140<std140::vec4> for glam::Vec4 {
    fn std140(&self) -> std140::vec4 {
        std140::vec4(self.x, self.y, self.z, self.w)
    }
}

impl ToStd140<std140::mat3x3> for glam::Mat3 {
    fn std140(&self) -> std140::mat3x3 {
        std140::mat3x3(
            self.x_axis.std140(),
            self.y_axis.std140(),
            self.z_axis.std140()
        )
    }
}

impl ToStd140<std140::mat4x4> for glam::Mat4 {
    fn std140(&self) -> std140::mat4x4 {
        std140::mat4x4(
            self.x_axis.std140(),
            self.y_axis.std140(),
            self.z_axis.std140(),
            self.w_axis.std140()
        )
    }
}

impl ToStd140<Std140AffineTransform3D> for AffineTransform3D {
    fn std140(&self) -> Std140AffineTransform3D {
        Std140AffineTransform3D {
            matrix: self.linear_transform.std140(),
            translation: self.translation.std140()
        }
    }
}
impl ToStd140<Std140AffineTransform4D> for AffineTransform4D {
    fn std140(&self) -> Std140AffineTransform4D {
        Std140AffineTransform4D {
            matrix: self.linear_transform.std140(),
            translation: self.translation.std140()
        }
    }
}