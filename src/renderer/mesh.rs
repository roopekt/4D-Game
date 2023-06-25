pub mod primitives;
pub mod vertex;
mod mesh_edit;

use crate::game::transform::{AffineTransform3D, AffineTransform4D};
use vertex::*;

type GpuIndexT = u32;

#[derive(Debug, Clone)]
pub struct SimpleMesh {
    pub vertices: Vec<CpuVertexSimple>,
    pub indeces: Vec<usize>,
    pub topology: glium::index::PrimitiveType
}
impl SimpleMesh {
    pub fn upload_static(&self, display: &glium::Display) -> StaticUploadedMeshSimple {
        StaticUploadedMeshSimple {
            vertices: get_gpu_vertices(display, &self.vertices),
            indeces: get_gpu_indeces(display, self.topology, &self.indeces)
        }
    }
}
impl From<Mesh3D> for SimpleMesh {
    fn from(mesh_3D: Mesh3D) -> Self {
        Self {
            vertices: mesh_3D.vertices.iter()
                .map(|&v| CpuVertexSimple { position: v.position })
                .collect(),
            indeces: mesh_3D.flat_indeces(),
            topology: glium::index::PrimitiveType::TrianglesList
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mesh3D {
    pub vertices: Vec<CpuVertex3D>,
    pub indeces: Vec<[usize; 3]>
}
impl Mesh3D {
    const EMPTY: Self = Self {
        vertices: Vec::new(),
        indeces: Vec::new()
    };

    pub fn upload_static(&self, display: &glium::Display) -> StaticUploadedMesh3D {
        StaticUploadedMesh3D {
            vertices: get_gpu_vertices(display, &self.vertices),
            indeces: get_gpu_indeces(display, glium::index::PrimitiveType::TrianglesList, &self.flat_indeces())
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

    pub fn flat_indeces(&self) -> Vec<usize> {
        self.indeces.iter().flatten().copied().collect()
    }
}
#[derive(Debug, Clone)]
pub struct Mesh4D {
    pub vertices: Vec<CpuVertex4D>,
    pub indeces: Vec<[usize; 4]>
}
impl Mesh4D {
    const EMPTY: Self = Self {
        vertices: Vec::new(),
        indeces: Vec::new()
    };

    pub fn upload_static(&self, display: &glium::Display) -> StaticUploadedMesh4D {
        StaticUploadedMesh4D {
            vertices: get_gpu_vertices(display, &self.vertices),
            indeces: get_gpu_indeces(display, glium::index::PrimitiveType::LinesListAdjacency, &self.flat_indeces())
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

    pub fn flat_indeces(&self) -> Vec<usize> {
        self.indeces.iter().flatten().copied().collect()
    }
}

#[derive(Debug)]
pub struct StaticUploadedMeshSimple {
    pub vertices: glium::VertexBuffer<GpuVertexSimple>,
    pub indeces: glium::IndexBuffer<GpuIndexT>
}
#[derive(Debug)]
pub struct StaticUploadedMesh3D {
    pub vertices: glium::VertexBuffer<GpuVertex3D>,
    pub indeces: glium::IndexBuffer<GpuIndexT>
}
#[derive(Debug)]
pub struct StaticUploadedMesh4D {
    pub vertices: glium::VertexBuffer<GpuVertex4D>,
    pub indeces: glium::IndexBuffer<GpuIndexT>
}

fn get_gpu_vertices<V, CV>(display: &glium::Display, cpu_vertices: &Vec<CV>) -> glium::VertexBuffer<V>
    where V: glium::Vertex + From<CV>, CV: Copy
{
    let vertices: Vec<V> = cpu_vertices.iter()
        .map(|&v| v.into())
        .collect();

    glium::VertexBuffer::immutable(display, &vertices).unwrap()
}

fn get_gpu_indeces(display: &glium::Display, topology: glium::index::PrimitiveType, cpu_indeces: &Vec<usize>) -> glium::IndexBuffer<GpuIndexT> {
    let indeces: Vec<GpuIndexT> = cpu_indeces.iter()
        .map(|&i| i.try_into().expect(&format!("Failed to convert index {} to {}", i, stringify!(GpuIndexT))))
        .collect();

    glium::IndexBuffer::immutable(display, topology, &indeces).unwrap()
}