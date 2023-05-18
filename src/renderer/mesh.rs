pub mod primitives;

use std::ops::{Add, AddAssign};
use std::iter::Sum;
use glam::{Mat4, Vec3};

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

    pub fn transform(&mut self, matrix: Mat4) {
        for vertex in self.vertices.iter_mut() {
            vertex.transform(matrix);
        };
    }

    pub fn as_transformed(mut self, matrix: Mat4) -> Self {
        self.transform(matrix);
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
    pub position: [f32; 3]
}
glium::implement_vertex!(Vertex, position);

impl Vertex {
    pub fn transform(&mut self, matrix: Mat4) {
        let mut pos_vec: Vec3 = self.position.into();
        pos_vec = matrix.transform_point3(pos_vec);
        self.position = pos_vec.into();
    }
}
