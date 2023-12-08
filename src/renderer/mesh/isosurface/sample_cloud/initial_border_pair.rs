use glam::{IVec3, IVec4};
use crate::errors::*;
use super::border_pair::*;
use super::is_positive;

pub fn get_initial_border_pair_3D<F: Fn(IVec3) -> f32>(normalized_function: &F, mut negative_point: IVec3, mut positive_point: IVec3) -> BorderPair3D {
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
pub fn get_initial_border_pair_4D<F: Fn(IVec4) -> f32>(normalized_function: &F, mut negative_point: IVec4, mut positive_point: IVec4) -> BorderPair4D {
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
