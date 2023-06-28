mod quad;
mod cube;
mod sphere;

pub use quad::*;
pub use cube::*;
pub use sphere::*;

use super::{Mesh3D, Mesh4D, CpuVertex3D, CpuVertex4D, CpuVertexSimple, SimpleMesh};
use glam::Vec3;
use crate::game::transform::Transform3D;
use std::{fmt::Debug as DebugTrait, hash::Hash};
use combinatorial::Combinations;

pub fn blit_quad() -> SimpleMesh {
    quad_3D()
        .as_transformed(&Transform3D { scale: 2.0 * Vec3::ONE, ..Default::default() }.into())
        .into()
}

pub fn vertical_line() -> SimpleMesh {
    SimpleMesh {
        vertices: vec![
            CpuVertexSimple { position: Vec3::new(0.0, -1.0, 0.0) },
            CpuVertexSimple { position: Vec3::new(0.0,  1.0, 0.0) }
        ],
        indeces: vec![0, 1],
        topology: glium::index::PrimitiveType::LinesList
    }
}

pub fn index_of<T: PartialEq + DebugTrait>(element: T, vec: &Vec<T>) -> usize {
    vec.iter().position(|e| *e == element).expect(&format!("Didn't find {:?}", element))
}

pub fn combinations_csize<T: Ord + Clone + DebugTrait, const COMBINATION_SIZE: usize>(source: impl IntoIterator<Item = T>) -> impl Iterator<Item = [T; COMBINATION_SIZE]> {
    Combinations::of_size(source, COMBINATION_SIZE)
        .map(|c| c.try_into().expect("Combinations::of_size should return vectors of compatible length."))
}

pub fn all_edges(source: impl IntoIterator<Item = usize>) -> impl Iterator<Item = EdgeIndeces> {
    combinations_csize(source)
        .map(|array_edge: [usize; 2]| array_edge.into())
}

//should always satisfy A <= B
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct EdgeIndeces {
    pub A: usize,
    pub B: usize
}
impl EdgeIndeces {
    pub fn new(a: usize, b: usize) -> Self {
        if a <= b {
            Self { A: a, B: b }
        }
        else {
            Self { A: b, B: a }
        }
    }

    pub fn has_index(&self, i: &usize) -> bool {
        self.A == *i || self.B == *i
    }
}
impl From<[usize; 2]> for EdgeIndeces {
    fn from(value: [usize; 2]) -> Self {
        Self { A: value[0], B: value[1] }
    }
}
impl From<EdgeIndeces> for [usize; 2] {
    fn from(value: EdgeIndeces) -> Self {
        [value.A, value.B]
    }
}
