use glam::{Vec3, Vec4};
use crate::game::transform::{AffineTransform3D, AffineTransform4D};

#[derive(Copy, Clone, Debug)]
pub struct CpuVertexSimple {
    pub position: Vec3
}
#[derive(Copy, Clone, Debug)]
pub struct CpuVertex3D {
    pub position: Vec3,
    pub normal: Vec3
}
#[derive(Copy, Clone, Debug)]
pub struct CpuVertex4D {
    pub position: Vec4,
    pub normal: Vec4
}

#[derive(Copy, Clone, Debug)]
pub struct GpuVertexSimple {
    pub position: [f32; 3]
}
#[derive(Copy, Clone, Debug)]
pub struct GpuVertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3]
}
#[derive(Copy, Clone, Debug)]
pub struct GpuVertex4D {
    pub position: [f32; 4],
    pub normal: [f32; 4]
}

impl From<CpuVertexSimple> for GpuVertexSimple {
    fn from(value: CpuVertexSimple) -> Self {
        Self {
            position: value.position.into()
        }
    }
}
impl From<CpuVertex3D> for GpuVertex3D {
    fn from(value: CpuVertex3D) -> Self {
        Self {
            position: value.position.into(),
            normal: value.normal.into()
        }
    }
}
impl From<CpuVertex4D> for GpuVertex4D {
    fn from(value: CpuVertex4D) -> Self {
        Self {
            position: value.position.into(),
            normal: value.normal.into()
        }
    }
}

glium::implement_vertex!(GpuVertexSimple, position);
glium::implement_vertex!(GpuVertex3D, position, normal);
glium::implement_vertex!(GpuVertex4D, position, normal);

impl CpuVertexSimple {
    pub fn transform(&mut self, transformation: &AffineTransform3D) {
        self.position = transformation * &self.position;
    }
}
impl CpuVertex3D {
    pub fn transform(&mut self, transformation: &AffineTransform3D) {
        self.position = transformation * &self.position;
        self.normal = transformation.point_transform_to_normal_transform() * self.normal;
    }

    pub fn mean<const N: usize>(vertices: [Self; N]) -> Self {
        let count = vertices.len() as f32;
        Self {
            position: vertices.iter()
                .map(|&v| v.position)
                .sum::<Vec3>() / count,
            normal: (vertices.iter()
                .map(|&v| v.normal)
                .sum::<Vec3>() / count)
                .normalize(),
        }
    }
}
impl CpuVertex4D {
    pub fn transform(&mut self, transformation: &AffineTransform4D) {
        self.position = transformation * &self.position;
        self.normal = transformation.point_transform_to_normal_transform() * self.normal;
    }

    pub fn mean<const N: usize>(vertices: [Self; N]) -> Self {
        let count = vertices.len() as f32;
        Self {
            position: vertices.iter()
                .map(|&v| v.position)
                .sum::<Vec4>() / count,
            normal: (vertices.iter()
                .map(|&v| v.normal)
                .sum::<Vec4>() / count)
                .normalize(),
        }
    }
}