mod border_pair;
mod initial_border_pair;

use self::border_pair::*;
use self::initial_border_pair::*;
use std::collections::{HashMap, HashSet};
use glam::{IVec3, IVec4};

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

fn is_positive(x: f32) -> bool {
    x > 0.0
}
