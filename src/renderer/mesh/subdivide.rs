use crate::errors::assert_equal;
use super::{Mesh3D, Mesh4D};
use super::primitives::{all_edges, EdgeIndeces};
use crate::combinations::combinations_constsize;
use super::vertex::{CpuVertex3D, CpuVertex4D};
use std::collections::{HashMap, HashSet};

impl Mesh3D {
    //doesn't subdivide the skeleton
    pub fn subdivide_surface(self) -> Self {
        let mut vertices = self.vertices;
        let mut indeces = Vec::new();
        let mut edge_vertex_index_from_edge = HashMap::<EdgeIndeces, usize>::new();

        for &triangle in &self.indeces {    
            let primitive_edges: Vec<EdgeIndeces> = all_edges(&triangle).collect();
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
            let primitive_edges: Vec<EdgeIndeces> = all_edges(&tetrahedron).collect();
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
            let mut mid_octahedron_tetrahedralization_edges: HashSet<EdgeIndeces> = all_edges(&mid_octahedron_indeces).collect();
            mid_octahedron_tetrahedralization_edges.remove(&mid_octahedron_diagonal_A);
            mid_octahedron_tetrahedralization_edges.remove(&mid_octahedron_diagonal_B);
            if mid_octahedron_tetrahedralization_edges.len() == 14 {
                println!("shit");
            }
            assert_equal!(mid_octahedron_tetrahedralization_edges.len(), 12 + 1);//should now have all outer edges, and one diagonal

            //add mid octahedron tetrahedra
            let mid_octahedron_tetrahedra: Vec<[usize; 4]> = combinations_constsize(&mid_octahedron_indeces)
                .filter(|&tetrahedron|
                    all_edges(&tetrahedron)
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