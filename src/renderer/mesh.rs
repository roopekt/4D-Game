pub mod primitives;

use std::ops::{Add, AddAssign};
use std::iter::Sum;
use glam::Vec3;
use crate::game::transform::AffineTransform3D;

type IndexT = u16;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indeces: Vec<IndexT>
}
impl Mesh {
    const EMPTY: Self = Self {
        vertices: Vec::new(),
        indeces: Vec::new()
    };

    pub fn upload_static(&self, display: &glium::Display) -> StaticUploadedMesh {
        StaticUploadedMesh {
            vertices: glium::VertexBuffer::immutable(display, &self.vertices).unwrap(),
            indeces: glium::IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &self.indeces).unwrap()
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

impl AddAssign for Mesh {
    fn add_assign(&mut self, rhs: Self) {
        let index_ofset = self.vertices.len() as IndexT;
        let rhs_indeces: Vec<IndexT> = rhs.indeces
            .iter()
            .map(|i| i + index_ofset)
            .collect();

        self.vertices.extend(rhs.vertices);
        self.indeces.extend(rhs_indeces);
    }
}

impl Add for Mesh {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        return self;
    }
}

impl Sum for Mesh {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::EMPTY, |a, b| a + b)
    }
}

pub struct StaticUploadedMesh {
    pub vertices: glium::VertexBuffer<Vertex>,
    pub indeces: glium::IndexBuffer<IndexT>
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3]
}
glium::implement_vertex!(Vertex, position, normal);

impl Vertex {
    pub fn transform(&mut self, transformation: &AffineTransform3D) {
        let mut pos_vec: Vec3 = self.position.into();
        pos_vec = transformation * &pos_vec;
        self.position = pos_vec.into();

        let mut normal_vec: Vec3 = self.normal.into();
        normal_vec = transformation.point_transform_to_normal_transform() * normal_vec;
        self.normal = normal_vec.into(); 
    }
}
