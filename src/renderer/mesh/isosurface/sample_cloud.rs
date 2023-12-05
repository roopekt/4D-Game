use std::collections::{HashMap, HashSet};
use glam::{IVec3, IVec4};
use itertools::Itertools;
use crate::combinations::combinations_constsize;
use crate::errors::*;

#[derive(Clone, Debug)]
pub struct SampleCloud3D {
    pub sample_map: HashMap<IVec3, f32>,//values of normalized_function
    pub border_pairs: Vec<BorderPair3D>//pairs of neighboring points, that get different signs from normalized_function
}
#[derive(Clone, Debug)]
pub struct SampleCloud4D {
    pub sample_map: HashMap<IVec4, f32>,//values of normalized_function
    pub border_pairs: Vec<BorderPair4D>//pairs of neighboring points, that get different signs from normalized_function
}

impl SampleCloud3D {
    pub fn new<F: Fn(IVec3) -> f32>(normalized_function: &F, negative_point: IVec3, positive_point: IVec3) -> Self {
        let relative_neighbor_sets = [0, 1, 2].map(|i| BorderPair3DRelativeNeighborSet::new(i));
        let initial_border_pair = get_initial_border_pair_3D(normalized_function, negative_point, positive_point);

        let mut sample_map = HashMap::from([
            (initial_border_pair.A, normalized_function(initial_border_pair.A)),
            (initial_border_pair.B, normalized_function(initial_border_pair.B))
        ]);
        let mut unprocessed_border_pairs = vec![initial_border_pair];
        let mut all_border_pairs: HashSet<BorderPair3D> = unprocessed_border_pairs.iter().copied().collect();

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

                let A_is_positive = is_positive(sample_map[&border_pair.A]);
                let B_is_positive = is_positive(sample_map[&border_pair.B]);
                if A_is_positive != B_is_positive {
                    all_border_pairs.insert(border_pair);
                    unprocessed_border_pairs.push(border_pair);
                }
            }
        };

        Self {
            sample_map,
            border_pairs: all_border_pairs.iter().copied().collect()
        }
    }
}
impl SampleCloud4D {
    pub fn new<F: Fn(IVec4) -> f32>(normalized_function: &F, negative_point: IVec4, positive_point: IVec4) -> Self {
        let relative_neighbor_sets = [0, 1, 2, 3].map(|i| BorderPair4DRelativeNeighborSet::new(i));
        let initial_border_pair = get_initial_border_pair_4D(normalized_function, negative_point, positive_point);

        let mut sample_map = HashMap::from([
            (initial_border_pair.A, normalized_function(initial_border_pair.A)),
            (initial_border_pair.B, normalized_function(initial_border_pair.B))
        ]);
        let mut unprocessed_border_pairs = vec![initial_border_pair];
        let mut all_border_pairs: HashSet<BorderPair4D> = unprocessed_border_pairs.iter().copied().collect();

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

                let A_is_positive = is_positive(sample_map[&border_pair.A]);
                let B_is_positive = is_positive(sample_map[&border_pair.B]);
                if A_is_positive != B_is_positive {
                    all_border_pairs.insert(border_pair);
                    unprocessed_border_pairs.push(border_pair);
                }
            }
        };

        Self {
            sample_map,
            border_pairs: all_border_pairs.iter().copied().collect()
        }
    }
}

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

struct BorderPair3DRelativeNeighborSet {
    points: Vec<IVec3>,
    border_pairs: Vec<BorderPair3D>
}
struct BorderPair4DRelativeNeighborSet {
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

fn get_initial_border_pair_3D<F: Fn(IVec3) -> f32>(normalized_function: &F, mut negative_point: IVec3, mut positive_point: IVec3) -> BorderPair3D {
    assert_false!(is_positive(normalized_function(negative_point)), format_args!("value: {}", normalized_function(negative_point)));
    assert_true!( is_positive(normalized_function(positive_point)), format_args!("value: {}", normalized_function(positive_point)));

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

    assert_false!(is_positive(normalized_function(negative_point)), format_args!("value: {}", normalized_function(negative_point)));
    assert_true!( is_positive(normalized_function(positive_point)), format_args!("value: {}", normalized_function(positive_point)));

    BorderPair3D::new(negative_point, positive_point)
}
fn get_initial_border_pair_4D<F: Fn(IVec4) -> f32>(normalized_function: &F, mut negative_point: IVec4, mut positive_point: IVec4) -> BorderPair4D {
    assert_false!(is_positive(normalized_function(negative_point)), format_args!("value: {}", normalized_function(negative_point)));
    assert_true!( is_positive(normalized_function(positive_point)), format_args!("value: {}", normalized_function(positive_point)));

    //binary search
    while ivec4_manhattan_magnitude(positive_point - negative_point) > 1 {
        let midpoint = ivec4_midpoint(negative_point, positive_point);
        let midpoint_is_positive = normalized_function(midpoint) > 0.0;

        if midpoint_is_positive {
            (negative_point, positive_point) = (negative_point, midpoint);
        }
        else {
            (negative_point, positive_point) = (midpoint, positive_point);
        }
    }

    assert_false!(is_positive(normalized_function(negative_point)), format_args!("value: {}", normalized_function(negative_point)));
    assert_true!( is_positive(normalized_function(positive_point)), format_args!("value: {}", normalized_function(positive_point)));

    BorderPair4D::new(negative_point, positive_point)
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
fn ivec4_midpoint(a: IVec4, b: IVec4) -> IVec4 {
    let delta = b - a;

    //if delta is small, a more accurate algorithm is needed
    if delta.abs().cmple(IVec4::ONE).all() {
        let equality = a.cmpeq(b);
        let first_difference_index = (0..4).find(|&i| !equality.test(i)).unwrap_or(0);

        let mut vec = a;
        vec[first_difference_index] = b[first_difference_index];//vec will differ from a by just one component (or zero if a == b)
        return vec;
    }

    a + delta / 2
}

fn ivec3_manhattan_magnitude(vec: IVec3) -> i32 {
    vec.abs().to_array().iter().sum::<i32>()
}
fn ivec4_manhattan_magnitude(vec: IVec4) -> i32 {
    vec.abs().to_array().iter().sum::<i32>()
}

fn is_positive(x: f32) -> bool {
    x > 0.0
}
