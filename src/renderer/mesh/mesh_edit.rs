use std::ops::{Add, AddAssign};
use std::iter::Sum;
use crate::errors::assert_equal;
use super::{Mesh3D, Mesh4D};
use super::primitives::{combinations_csize, all_edges, EdgeIndeces};
use super::vertex::{CpuVertex3D, CpuVertex4D};
use std::collections::{HashMap, HashSet};

impl AddAssign for Mesh3D {
    fn add_assign(&mut self, mut rhs: Self) {
        let index_ofset = self.vertices.len();
        for prim in rhs.indeces.iter_mut() {
            for i in prim.iter_mut() {
                *i += index_ofset;
            }
        }
        for prim in rhs.skeleton_indeces.iter_mut() {
            for i in prim.iter_mut() {
                *i += index_ofset;
            }
        }

        self.vertices.extend(rhs.vertices);
        self.indeces.extend(rhs.indeces);
        self.skeleton_indeces.extend(rhs.skeleton_indeces);
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
        for prim in rhs.skeleton_indeces.iter_mut() {
            for i in prim.iter_mut() {
                *i += index_ofset;
            }
        }

        self.vertices.extend(rhs.vertices);
        self.indeces.extend(rhs.indeces);
        self.skeleton_indeces.extend(rhs.skeleton_indeces);
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
    //doesn't subdivide the skeleton
    pub fn subdivide_surface(self) -> Self {
        let mut vertices = self.vertices;
        let mut indeces = Vec::new();
        let mut edge_vertex_index_from_edge = HashMap::<EdgeIndeces, usize>::new();

        for &triangle in &self.indeces {    
            let primitive_edges: Vec<EdgeIndeces> = all_edges(triangle.clone()).collect();
            assert_equal!(primitive_edges.len(), 3);

            //new vertices and their indeces
            let mut all_edge_vertex_indeces = Vec::new();
            for edge in &primitive_edges {
                let edge_vertex_index = edge_vertex_index_from_edge
                    .entry(*edge)
                    .or_insert_with(|| {
                        vertices.push(CpuVertex3D::mean([vertices[edge.A], vertices[edge.B]]));
                        vertices.len() - 1//index of the just pushed vertex
                    }
                ).clone();
                all_edge_vertex_indeces.push(edge_vertex_index);
            }
            assert_equal!(all_edge_vertex_indeces.len(), 3);

            //corner triangles
            for corner_index in triangle {
                let corner_edge_vertex_indeces: Vec<usize> = primitive_edges.iter()
                    .filter(|edge| edge.has_index(&corner_index))
                    .map(|edge| *edge_vertex_index_from_edge.get(edge).unwrap())
                    .collect();
                assert_equal!(corner_edge_vertex_indeces.len(), 2);

                indeces.push([
                    corner_index,
                    corner_edge_vertex_indeces[0],
                    corner_edge_vertex_indeces[1]
                ]);
            }

            //mid triangle
            indeces.push(all_edge_vertex_indeces.try_into().unwrap());
        }

        Self {
            vertices,
            indeces,
            skeleton_indeces: self.skeleton_indeces
        }
    }
}
impl Mesh4D {
    pub fn subdivide_surface(self) -> Self {
        let mut vertices = self.vertices;
        let mut indeces = Vec::new();
        let mut edge_vertex_index_from_edge = HashMap::<EdgeIndeces, usize>::new();

        for &tetrahedron in &self.indeces {    
            let primitive_edges: Vec<EdgeIndeces> = all_edges(tetrahedron.clone()).collect();
            assert_equal!(primitive_edges.len(), 6);

            //new vertices and their indeces
            let mut all_edge_vertex_indeces = Vec::new();
            for edge in &primitive_edges {
                let edge_vertex_index = edge_vertex_index_from_edge
                    .entry(*edge)
                    .or_insert_with(|| {
                        vertices.push(CpuVertex4D::mean([vertices[edge.A], vertices[edge.B]]));
                        vertices.len() - 1//index of the just pushed vertex
                    }
                ).clone();
                all_edge_vertex_indeces.push(edge_vertex_index);
            }
            assert_equal!(all_edge_vertex_indeces.len(), 6);

            //corner tetrahedra
            for corner_index in tetrahedron {
                let corner_edge_vertex_indeces: Vec<usize> = primitive_edges.iter()
                    .filter(|edge| edge.has_index(&corner_index))
                    .map(|edge| *edge_vertex_index_from_edge.get(edge).unwrap())
                    .collect();
                assert_equal!(corner_edge_vertex_indeces.len(), 3);

                indeces.push([
                    corner_index,
                    corner_edge_vertex_indeces[0],
                    corner_edge_vertex_indeces[1],
                    corner_edge_vertex_indeces[2]
                ]);
            }
            
            //mid octahedron helper data
            let mid_octahedron_indeces = all_edge_vertex_indeces;
            let mid_octahedron_diagonal_A = EdgeIndeces::new(
                *edge_vertex_index_from_edge.get(&EdgeIndeces::new(tetrahedron[0], tetrahedron[1])).unwrap(),
                *edge_vertex_index_from_edge.get(&EdgeIndeces::new(tetrahedron[2], tetrahedron[3])).unwrap()
            );
            let mid_octahedron_diagonal_B = EdgeIndeces::new(
                *edge_vertex_index_from_edge.get(&EdgeIndeces::new(tetrahedron[0], tetrahedron[2])).unwrap(),
                *edge_vertex_index_from_edge.get(&EdgeIndeces::new(tetrahedron[1], tetrahedron[3])).unwrap()
            );
            let mut mid_octahedron_tetrahedralization_edges: HashSet<EdgeIndeces> = all_edges(mid_octahedron_indeces.clone()).collect();
            mid_octahedron_tetrahedralization_edges.remove(&mid_octahedron_diagonal_A);
            mid_octahedron_tetrahedralization_edges.remove(&mid_octahedron_diagonal_B);
            assert_equal!(mid_octahedron_tetrahedralization_edges.len(), 12 + 1);//should now have all outer edges, and one diagonal

            //add mid octahedron tetrahedra
            let mid_octahedron_tetrahedra: Vec<[usize; 4]> = combinations_csize(mid_octahedron_indeces)
                .filter(|&tetrahedron|
                    all_edges(tetrahedron)
                    .all(|edge| mid_octahedron_tetrahedralization_edges.contains(&edge)))
                .collect();
            assert_equal!(mid_octahedron_tetrahedra.len(), 4);
            indeces.extend_from_slice(&mid_octahedron_tetrahedra);
        }

        Self {
            vertices,
            indeces,
            skeleton_indeces: self.skeleton_indeces
        }
    }
}

impl Mesh3D {
    //a full skeleton contains every vertex
    pub fn with_full_skeleton(mut self) -> Self {
        self.skeleton_indeces = (0..self.vertices.len())
            .map(|i| [i])
            .collect();
        self
    }
}
impl Mesh4D {
    //a full skeleton contains every edge
    pub fn with_full_skeleton(mut self) -> Self {
        //collecting to a set removes duplicates
        let edge_set: HashSet<[usize; 2]> = self.indeces.iter()
            .map(|&primitive| combinations_csize::<usize, 2>(primitive))
            .flatten()
            .collect();
        self.skeleton_indeces = edge_set.iter().copied().collect();
        self
    }
}