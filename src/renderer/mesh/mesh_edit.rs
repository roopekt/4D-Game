use std::ops::{Add, AddAssign};
use std::iter::Sum;
use combinatorial::Combinations;
use crate::errors::assert_equal;
use super::{Mesh3D, Mesh4D};
use super::vertex::{CpuVertex3D};

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

impl Mesh3D {
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