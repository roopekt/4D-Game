use std::ops::{Add, AddAssign};
use std::iter::Sum;
use crate::errors::assert_equal;
use super::{Mesh3D, Mesh4D};
use super::primitives::{index_of, combinations_csize};
use super::vertex::{CpuVertex3D, CpuVertex4D};
use std::collections::HashSet;

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
            let edges: Vec<[usize; 2]> = combinations_csize(triangle.clone()).collect();
            assert_equal!(edges.len(), 3);

            //new vertices
            for edge in &edges {
                let mid_edge_vertex = CpuVertex3D::mean([new_vertices[edge[0]], new_vertices[edge[1]]]);
                new_vertices.push(mid_edge_vertex);//DUPLICATE, edge is shared
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

        Self {
            vertices: new_vertices,
            indeces: new_indeces
        }
    }
}
impl Mesh4D {
    pub fn subdivide(self) -> Self {
        let mut new_vertices = self.vertices;
        let mut new_indeces = Vec::new();
        for &tetrahedron in &self.indeces {    
            let index_offset = new_vertices.len();
            let edges: Vec<[usize; 2]> = combinations_csize(tetrahedron.clone()).collect();
            assert_equal!(edges.len(), 6);

            //new vertices
            for edge in &edges {
                let mid_edge_vertex = CpuVertex4D::mean([new_vertices[edge[0]], new_vertices[edge[1]]]);
                new_vertices.push(mid_edge_vertex);//DUPLICATE, edge is shared
            }

            //corner tetrahedra
            for corner_index in tetrahedron {
                let new_relative_vertex_indeces: Vec<usize> = edges.iter()
                    .enumerate()
                    .filter(|(_i, edge)| edge.contains(&corner_index))
                    .map(|(i, _edge)| i)
                    .collect();
                assert_equal!(new_relative_vertex_indeces.len(), 3);

                new_indeces.push([
                    corner_index,
                    index_offset + new_relative_vertex_indeces[0],
                    index_offset + new_relative_vertex_indeces[1],
                    index_offset + new_relative_vertex_indeces[2]
                ]);
            }
            
            //mid octahedron helper data
            let mid_octahedron_indeces = index_offset .. index_offset + edges.len();//indeces of new vertices
            let mid_octahedron_diagonal_A = [
                index_offset + index_of([tetrahedron[0], tetrahedron[1]], &edges),
                index_offset + index_of([tetrahedron[2], tetrahedron[3]], &edges),
            ];
            let mid_octahedron_diagonal_B = [
                index_offset + index_of([tetrahedron[0], tetrahedron[2]], &edges),
                index_offset + index_of([tetrahedron[1], tetrahedron[3]], &edges),
            ];
            let mut mid_octahedron_tetrahedralization_edges: HashSet<[usize; 2]> = combinations_csize(mid_octahedron_indeces.clone()).collect();
            assert!(mid_octahedron_tetrahedralization_edges.remove(&mid_octahedron_diagonal_A));
            assert!(mid_octahedron_tetrahedralization_edges.remove(&mid_octahedron_diagonal_B));
            assert_equal!(mid_octahedron_tetrahedralization_edges.len(), 12 + 1);//should now have all outer edges, and one diagonal

            //add mid octahedron tetrahedra
            let mid_octahedron_tetrahedra: Vec<[usize; 4]> = combinations_csize(mid_octahedron_indeces)
                .filter(|&tetrahedron|
                    combinations_csize(tetrahedron)
                    .all(|edge: [usize; 2]| mid_octahedron_tetrahedralization_edges.contains(&edge)))
                .collect();
            assert_equal!(mid_octahedron_tetrahedra.len(), 4);
            new_indeces.extend_from_slice(&mid_octahedron_tetrahedra);
        }

        Self {
            vertices: new_vertices,
            indeces: new_indeces
        }
    }
}