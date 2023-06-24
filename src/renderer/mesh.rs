pub mod primitives;

use std::ops::{Add, AddAssign};
use std::iter::Sum;
use glam::{Vec3, Vec4};
use crate::game::transform::{AffineTransform3D, AffineTransform4D};

type GpuIndexT = u16;

#[derive(Debug, Clone)]
pub struct SimpleMesh {
    pub vertices: Vec<SimpleVertex>,
    pub indeces: Vec<usize>,
    pub topology: glium::index::PrimitiveType
}
impl SimpleMesh {
    pub fn upload_static(&self, display: &glium::Display) -> StaticUploadedMeshSimple {
        StaticUploadedMeshSimple {
            vertices: glium::VertexBuffer::immutable(display, &self.vertices).unwrap(),
            indeces: get_gpu_indeces(display, self.topology, &self.indeces)
        }
    }
}
impl From<Mesh3D> for SimpleMesh {
    fn from(mesh_3D: Mesh3D) -> Self {
        Self {
            vertices: mesh_3D.vertices.iter()
                .map(|&v| SimpleVertex { position: v.position })
                .collect(),
            indeces: mesh_3D.indeces,
            topology: glium::index::PrimitiveType::TrianglesList
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mesh3D {
    pub vertices: Vec<Vertex3D>,
    pub indeces: Vec<usize>
}
impl Mesh3D {
    const EMPTY: Self = Self {
        vertices: Vec::new(),
        indeces: Vec::new()
    };

    pub fn upload_static(&self, display: &glium::Display) -> StaticUploadedMesh3D {
        StaticUploadedMesh3D {
            vertices: glium::VertexBuffer::immutable(display, &self.vertices).unwrap(),
            indeces: get_gpu_indeces(display, glium::index::PrimitiveType::TrianglesList, &self.indeces)
        }
    }

    pub fn transform(&mut self, transformation: &AffineTransform3D) {
        for vertex in self.vertices.iter_mut() {
            vertex.transform(transformation);
        };
    }

    pub fn as_transformed(mut self, transformation: &AffineTransform3D) -> Self {
        self.transform(transformation);
        return self;
    }
}
#[derive(Debug, Clone)]
pub struct Mesh4D {
    pub vertices: Vec<Vertex4D>,
    pub indeces: Vec<usize>
}
impl Mesh4D {
    const EMPTY: Self = Self {
        vertices: Vec::new(),
        indeces: Vec::new()
    };

    pub fn upload_static(&self, display: &glium::Display) -> StaticUploadedMesh4D {
        StaticUploadedMesh4D {
            vertices: glium::VertexBuffer::immutable(display, &self.vertices).unwrap(),
            indeces: get_gpu_indeces(display, glium::index::PrimitiveType::LinesListAdjacency, &self.indeces)
        }
    }

    pub fn transform(&mut self, transformation: &AffineTransform4D) {
        for vertex in self.vertices.iter_mut() {
            vertex.transform(transformation);
        };
    }

    pub fn as_transformed(mut self, transformation: &AffineTransform4D) -> Self {
        self.transform(transformation);
        return self;
    }
}

impl AddAssign for Mesh3D {
    fn add_assign(&mut self, rhs: Self) {
        let index_ofset = self.vertices.len();
        let rhs_indeces: Vec<usize> = rhs.indeces
            .iter()
            .map(|i| i + index_ofset)
            .collect();

        self.vertices.extend(rhs.vertices);
        self.indeces.extend(rhs_indeces);
    }
}
impl AddAssign for Mesh4D {
    fn add_assign(&mut self, rhs: Self) {
        let index_ofset = self.vertices.len();
        let rhs_indeces: Vec<usize> = rhs.indeces
            .iter()
            .map(|i| i + index_ofset)
            .collect();

        self.vertices.extend(rhs.vertices);
        self.indeces.extend(rhs_indeces);
    }
}

impl Add for Mesh3D {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        return self;
    }
}
impl Add for Mesh4D {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        return self;
    }
}

impl Sum for Mesh3D {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::EMPTY, |a, b| a + b)
    }
}
impl Sum for Mesh4D {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::EMPTY, |a, b| a + b)
    }
}

#[derive(Debug)]
pub struct StaticUploadedMeshSimple {
    pub vertices: glium::VertexBuffer<SimpleVertex>,
    pub indeces: glium::IndexBuffer<GpuIndexT>
}
#[derive(Debug)]
pub struct StaticUploadedMesh3D {
    pub vertices: glium::VertexBuffer<Vertex3D>,
    pub indeces: glium::IndexBuffer<GpuIndexT>
}
#[derive(Debug)]
pub struct StaticUploadedMesh4D {
    pub vertices: glium::VertexBuffer<Vertex4D>,
    pub indeces: glium::IndexBuffer<GpuIndexT>
}

#[derive(Copy, Clone, Debug)]
pub struct SimpleVertex {
    pub position: [f32; 3]
}
#[derive(Copy, Clone, Debug)]
pub struct Vertex3D {
    pub position: [f32; 3],
    pub normal: [f32; 3]
}
#[derive(Copy, Clone, Debug)]
pub struct Vertex4D {
    pub position: [f32; 4],
    pub normal: [f32; 4]
}

glium::implement_vertex!(SimpleVertex, position);
impl SimpleVertex {
    pub fn transform(&mut self, transformation: &AffineTransform3D) {
        let mut pos_vec: Vec3 = self.position.into();
        pos_vec = transformation * &pos_vec;
        self.position = pos_vec.into();
    }
}
glium::implement_vertex!(Vertex3D, position, normal);
impl Vertex3D {
    pub fn transform(&mut self, transformation: &AffineTransform3D) {
        let mut pos_vec: Vec3 = self.position.into();
        pos_vec = transformation * &pos_vec;
        self.position = pos_vec.into();

        let mut normal_vec: Vec3 = self.normal.into();
        normal_vec = transformation.point_transform_to_normal_transform() * normal_vec;
        self.normal = normal_vec.into(); 
    }
}
glium::implement_vertex!(Vertex4D, position, normal);
impl Vertex4D {
    pub fn transform(&mut self, transformation: &AffineTransform4D) {
        let mut pos_vec: Vec4 = self.position.into();
        pos_vec = transformation * &pos_vec;
        self.position = pos_vec.into();

        let mut normal_vec: Vec4 = self.normal.into();
        normal_vec = transformation.point_transform_to_normal_transform() * normal_vec;
        self.normal = normal_vec.into(); 
    }
}

fn get_gpu_indeces(display: &glium::Display, topology: glium::index::PrimitiveType, cpu_indeces: &Vec<usize>) -> glium::IndexBuffer<GpuIndexT> {
    let indeces: Vec<GpuIndexT> = cpu_indeces.iter()
        .map(|&i| i.try_into().expect(&format!("Failed to convert index {} to {}", i, stringify!(GpuIndexT))))
        .collect();

    glium::IndexBuffer::immutable(display, topology, &indeces).unwrap()
}