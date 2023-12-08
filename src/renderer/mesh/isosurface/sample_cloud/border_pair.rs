use glam::{IVec3, IVec4};
use itertools::Itertools;
use crate::combinations::combinations_constsize;
use crate::errors::*;

//should always satisfy A[axis_index] < B[axis_index]
//should always satisfy sign(normalized_function(A)) != sign(normalized_function(B))
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BorderPair3D {
    pub A: IVec3,
    pub B: IVec3,
    pub axis_index: usize//a line passing through A and B should be parallel to some axis
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BorderPair4D {
    pub A: IVec4,
    pub B: IVec4,
    pub axis_index: usize//a line passing through A and B should be parallel to some axis
}

impl BorderPair3D {
    pub fn new(mut A: IVec3, mut B: IVec3) -> Self {
        assert_equal!(ivec3_manhattan_magnitude(B - A), 1);

        let equality = A.cmpeq(B);
        let first_difference_index = (0..3).find(|&i| !equality.test(i)).unwrap();
        let axis_index = first_difference_index;

        if A[axis_index] > B[axis_index] {
            (A, B) = (B, A);
        }

        Self { A, B, axis_index }
    }
}
impl BorderPair4D {
    pub fn new(mut A: IVec4, mut B: IVec4) -> Self {
        assert_equal!(ivec4_manhattan_magnitude(B - A), 1);

        let equality = A.cmpeq(B);
        let first_difference_index = (0..4).find(|&i| !equality.test(i)).unwrap();
        let axis_index = first_difference_index;

        if A[axis_index] > B[axis_index] {
            (A, B) = (B, A);
        }

        Self { A, B, axis_index }
    }
}

impl From<BorderPair3D> for [IVec3; 2] {
    fn from(value: BorderPair3D) -> Self {
        [value.A, value.B]
    }
}
impl From<BorderPair4D> for [IVec4; 2] {
    fn from(value: BorderPair4D) -> Self {
        [value.A, value.B]
    }
}

pub struct BorderPair3DRelativeNeighborSet {
    points: Vec<IVec3>,
    border_pairs: Vec<BorderPair3D>
}
pub struct BorderPair4DRelativeNeighborSet {
    points: Vec<IVec4>,
    border_pairs: Vec<BorderPair4D>
}

impl BorderPair3DRelativeNeighborSet {
    pub fn new(axis_index: usize) -> Self {
        let mut point_offsets = Vec::<IVec3>::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in 0..=1 {
                    let mut p = IVec3::new(x, y, z);
                    (p[axis_index], p.z) = (p.z, p[axis_index]);
                    point_offsets.push(p);
                }
            }
        }
    
        let mut v = IVec3::ZERO;
        v[axis_index] = 1;
        let parent_pair = BorderPair3D::new(IVec3::ZERO, v);
    
        let possible_border_pairs = combinations_constsize::<2,_>(&point_offsets)
            .filter(|&pair| ivec3_manhattan_magnitude(pair[1] - pair[0]) == 1)
            .map(|pair| BorderPair3D::new(pair[0], pair[1]))
            .unique()
            .filter(|&pair| pair != parent_pair)
            .collect_vec();
    
        point_offsets = point_offsets.iter()
            .filter(|&&p| p != parent_pair.A && p != parent_pair.B)
            .copied().collect();

        Self {
            points: point_offsets,
            border_pairs: possible_border_pairs
        }
    }

    pub fn absolute_points(&self, parent_pair: BorderPair3D) -> impl Iterator<Item = IVec3> + '_ {
        self.points.iter().map(move |&point| point + parent_pair.A)
    }

    pub fn absolute_border_pairs(&self, parent_pair: BorderPair3D) -> impl Iterator<Item = BorderPair3D> + '_ {
        self.border_pairs.iter().map(move |&pair| {
            let mut out = pair;
            out.A += parent_pair.A;
            out.B += parent_pair.A;
            out
        })
    }
}
impl BorderPair4DRelativeNeighborSet {
    pub fn new(axis_index: usize) -> Self {
        let mut point_offsets = Vec::<IVec4>::new();
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in 0..=1 {
                        let mut p = IVec4::new(x, y, z, w);
                        (p[axis_index], p.w) = (p.w, p[axis_index]);
                        point_offsets.push(p);
                    }
                }
            }
        }
    
        let mut v = IVec4::ZERO;
        v[axis_index] = 1;
        let parent_pair = BorderPair4D::new(IVec4::ZERO, v);
    
        let possible_border_pairs = combinations_constsize::<2,_>(&point_offsets)
            .filter(|&pair| ivec4_manhattan_magnitude(pair[1] - pair[0]) == 1)
            .map(|pair| BorderPair4D::new(pair[0], pair[1]))
            .unique()
            .filter(|&pair| pair != parent_pair)
            .collect_vec();
    
        point_offsets = point_offsets.iter()
            .filter(|&&p| p != parent_pair.A && p != parent_pair.B)
            .copied().collect();

        Self {
            points: point_offsets,
            border_pairs: possible_border_pairs
        }
    }

    pub fn absolute_points(&self, parent_pair: BorderPair4D) -> impl Iterator<Item = IVec4> + '_ {
        self.points.iter().map(move |&point| point + parent_pair.A)
    }

    pub fn absolute_border_pairs(&self, parent_pair: BorderPair4D) -> impl Iterator<Item = BorderPair4D> + '_ {
        self.border_pairs.iter().map(move |&pair| {
            let mut out = pair;
            out.A += parent_pair.A;
            out.B += parent_pair.A;
            out
        })
    }
}

pub fn ivec3_manhattan_magnitude(vec: IVec3) -> i32 {
    vec.abs().to_array().iter().sum::<i32>()
}
pub fn ivec4_manhattan_magnitude(vec: IVec4) -> i32 {
    vec.abs().to_array().iter().sum::<i32>()
}