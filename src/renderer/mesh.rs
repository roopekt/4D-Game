pub mod primitives;

use std::ops::{Add, AddAssign};
use std::iter::Sum;
use combinatorial::Combinations;
use glam::{Vec3, Vec4};
use crate::errors::assert_equal;
use crate::game::transform::{AffineTransform3D, AffineTransform4D};

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

    pub fn subdivide(self) -> Self {
        let mut new_vertices = self.vertices;
        let mut new_indeces = Vec::new();
        for &triangle in &self.indeces {    
            let index_offset = new_vertices.len();
            let edges: Vec<Vec<usize>> = Combinations::of_size(triangle.clone(), 2).collect();
            assert_equal!(edges.len(), 3);
    
            //new vertices
            for edge in &edges {
                let mid_edge_vertex = CpuVertex3D::mean([new_vertices[edge[0]], new_vertices[edge[1]]]);
                new_vertices.push(mid_edge_vertex);
            }
    
            //corner triangles
            for corner_index in triangle {
                let new_relative_vertex_indeces: Vec<usize> = edges.iter()
                    .enumerate()
                    .filter(|(_i, edge)| edge.contains(&corner_index))
                    .map(|(i, _edge)| i)
                    .collect();
                assert_equal!(new_relative_vertex_indeces.len(), 2);
    
                new_indeces.push([
                    corner_index,
                    index_offset + new_relative_vertex_indeces[0],
                    index_offset + new_relative_vertex_indeces[1]
                ]);
            }
    
            //mid triangle
            new_indeces.push([
                index_offset + 0,
                index_offset + 1,
                index_offset + 2
            ]);
        }
    
        Mesh3D {
            vertices: new_vertices,
            indeces: new_indeces
        }
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

impl AddAssign for Mesh3D {
    fn add_assign(&mut self, mut rhs: Self) {
        let index_ofset = self.vertices.len();
        for prim in rhs.indeces.iter_mut() {
            for i in prim.iter_mut() {
                *i += index_ofset;
            }
        }

        self.vertices.extend(rhs.vertices);
        self.indeces.extend(rhs.indeces);
    }
}
impl AddAssign for Mesh4D {
    fn add_assign(&mut self, mut rhs: Self) {
        let index_ofset = self.vertices.len();
        for prim in rhs.indeces.iter_mut() {
            for i in prim.iter_mut() {
                *i += index_ofset;
            }
        }

        self.vertices.extend(rhs.vertices);
        self.indeces.extend(rhs.indeces);
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