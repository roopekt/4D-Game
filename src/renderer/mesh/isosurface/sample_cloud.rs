use std::collections::{HashMap, HashSet};
use glam::IVec3;
use itertools::Itertools;
use crate::combinations::combinations_constsize;
use crate::errors::{assert_equal, assert_less, assert_more};

#[derive(Clone, Debug)]
pub struct SampleCloud3D {
    sample_map: HashMap<IVec3, f32>,//values of normalized_function
    border_pairs: Vec<[IVec3; 2]>//pairs of neighboring points, that get different signs from normalized_function
}
impl SampleCloud3D {
    pub fn new(normalized_function: fn(IVec3) -> f32, negative_point: IVec3, positive_point: IVec3) -> Self {
        let relative_neighbor_sets = [0, 1, 2].map(|i| BorderPairRelativeNeighborSet::new(i));

        let mut sample_map = HashMap::<IVec3, f32>::new();
        let mut all_border_pairs = HashSet::<BorderPair3D>::new();
        let mut unprocessed_border_pairs = vec![get_initial_border_pair_3D(normalized_function, negative_point, positive_point)];

        //depth first search
        while !unprocessed_border_pairs.is_empty() {
            let pair = unprocessed_border_pairs.pop().unwrap();
            let relative_neighbors = &relative_neighbor_sets[pair.axis_index];

            for point in relative_neighbors.absolute_points(pair) {
                sample_map.entry(point).or_insert_with(|| normalized_function(point));
            }

            for border_pair in relative_neighbors.absolute_border_pairs(pair) {
                if all_border_pairs.contains(&border_pair) {
                    continue;
                }

                let A_is_positive = sample_map[&border_pair.A] > 0.0;
                let B_is_positive = sample_map[&border_pair.B] > 0.0;
                if A_is_positive != B_is_positive {
                    all_border_pairs.insert(border_pair);
                    unprocessed_border_pairs.push(border_pair);
                }
            }
        };

        Self {
            sample_map,
            border_pairs: all_border_pairs.iter().map(|&p| p.into()).collect()
        }
    }
}

//should always satisfy A[axis_index] < B[axis_index]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct BorderPair3D {
    A: IVec3,
    B: IVec3,
    axis_index: usize//a line passing through A and B should be parallel to some axis
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
impl From<BorderPair3D> for [IVec3; 2] {
    fn from(value: BorderPair3D) -> Self {
        [value.A, value.B]
    }
}

struct BorderPairRelativeNeighborSet {
    points: Vec<IVec3>,
    border_pairs: Vec<BorderPair3D>
}
impl BorderPairRelativeNeighborSet {
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

fn get_initial_border_pair_3D(normalized_function: fn(IVec3) -> f32, mut negative_point: IVec3, mut positive_point: IVec3) -> BorderPair3D {
    assert_less!(normalized_function(negative_point), 0.0);
    assert_more!(normalized_function(positive_point), 0.0);

    //binary search
    while ivec3_manhattan_magnitude(positive_point - negative_point) > 1 {
        let midpoint = ivec3_midpoint(negative_point, positive_point);
        let midpoint_is_positive = normalized_function(midpoint) > 0.0;

        if midpoint_is_positive {
            (negative_point, positive_point) = (negative_point, midpoint);
        }
        else {
            (negative_point, positive_point) = (midpoint, positive_point);
        }
    }

    assert_less!(normalized_function(negative_point), 0.0);
    assert_more!(normalized_function(positive_point), 0.0);

    BorderPair3D::new(negative_point, positive_point)
}

fn ivec3_midpoint(a: IVec3, b: IVec3) -> IVec3 {
    let delta = b - a;

    //if delta is small, a more accurate algorithm is needed
    if delta.abs().cmple(IVec3::ONE).all() {
        let equality = a.cmpeq(b);
        let first_difference_index = (0..3).find(|&i| !equality.test(i)).unwrap_or(0);

        let mut vec = a;
        vec[first_difference_index] = b[first_difference_index];//vec will differ from a by just one component (or zero if a == b)
        return vec;
    }

    a + delta / 2
}

fn ivec3_manhattan_magnitude(vec: IVec3) -> i32 {
    vec.abs().to_array().iter().sum::<i32>()
}
