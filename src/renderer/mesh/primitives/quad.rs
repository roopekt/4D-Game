use super::{Mesh3D, Mesh4D, CpuVertex3D, CpuVertex4D};
use glam::{Vec3, Vec4};
use crate::errors::assert_equal;
use std::fmt::Debug as DebugTrait;

//yes, this is a very convoluted way to get a quad, but this is easier to generalize to 4D
pub fn quad_3D() -> Mesh3D {
    type CornerSigns = [bool; 2];
    let corners: Vec<CornerSigns> = all_bool_arrays();
    assert_equal!(corners.len(), 4);

    let mut corners_of_diagonal: Vec<CornerSigns> = vec!{[false, false]};
    for corner in &corners {
        let match_count = bool_array_match_count(&corners[0], corner);
        let is_new_diagonal = match match_count {
            0 => true,
            1 | 2 => false,
            _ => panic!("impossible match count {}", match_count)
        };
        if is_new_diagonal {
            corners_of_diagonal.push(*corner);
        }
    }
    assert_equal!(corners_of_diagonal.len(), 2);

    let mut indeces: Vec<[usize; 3]> = Vec::new();
    for i in 0..corners.len() {
        let corner_i = &corners[i];
        if !corners_of_diagonal.contains(corner_i) {
            indeces.push([
                i,
                index_of([!corner_i[0],  corner_i[1]], &corners),
                index_of([ corner_i[0], !corner_i[1]], &corners)
            ]);
        }
    }
    assert_equal!(indeces.len(), 2);

    let normal = Vec3::Z;
    let corner_signs_to_vertex = |signs: &CornerSigns| -> CpuVertex3D {
        let corner_sign_to_number = |sign: bool| {
            match sign {
                false => -0.5,
                true => 0.5
            }
        };

        CpuVertex3D {
            position: Vec3::new(
                corner_sign_to_number(signs[0]),
                corner_sign_to_number(signs[1]),
                0.0
            ),
            normal
        }
    };

    Mesh3D {
        vertices: corners
            .iter()
            .map(|&c| corner_signs_to_vertex(&c))
            .collect(),
        indeces
    }
}

pub fn cube_4D() -> Mesh4D {
    type CornerSigns = [bool; 3];
    let corners: Vec<CornerSigns> = all_bool_arrays();
    assert_equal!(corners.len(), 8);

    let mut corners_of_central: Vec<CornerSigns> = vec![[false, false, false]];
    for corner in &corners {
        let match_count = bool_array_match_count(&corners[0], corner);
        let is_new_diagonal = match match_count {
            1 => true,
            0 | 2 | 3 => false,
            _ => panic!("impossible match count {}", match_count)
        };
        if is_new_diagonal {
            corners_of_central.push(*corner);
        }
    }
    assert_equal!(corners_of_central.len(), 4);

    let mut indeces: Vec<[usize; 4]> = vec![corners_of_central.iter()
        .map(|corner| index_of(*corner, &corners))
        .collect::<Vec<usize>>().try_into().unwrap()];
    for i in 0..corners.len() {
        let corner_i = &corners[i];
        if !corners_of_central.contains(corner_i) {
            indeces.push([
                i,
                index_of([!corner_i[0],  corner_i[1],  corner_i[2]], &corners),
                index_of([ corner_i[0], !corner_i[1],  corner_i[2]], &corners),
                index_of([ corner_i[0],  corner_i[1], !corner_i[2]], &corners)
            ]);
        }
    }
    assert_equal!(indeces.len(), 5);

    let normal = Vec4::W;
    let corner_signs_to_vertex = |signs: &CornerSigns| -> CpuVertex4D {
        let corner_sign_to_number = |sign: bool| {
            match sign {
                false => -0.5,
                true => 0.5
            }
        };

        CpuVertex4D {
            position: Vec4::new(
                corner_sign_to_number(signs[0]),
                corner_sign_to_number(signs[1]),
                corner_sign_to_number(signs[2]),
                0.0
            ),
            normal
        }
    };

    Mesh4D {
        vertices: corners
            .iter()
            .map(|&c| corner_signs_to_vertex(&c))
            .collect(),
        indeces
    }
}

fn all_bool_arrays<const BOOL_COUNT: usize>() -> Vec<[bool; BOOL_COUNT]> {
    (0..(1 << BOOL_COUNT))
        .map(|int| int_to_bool_array(int))
        .collect()
}

fn bool_array_match_count<const BOOL_COUNT: usize>(a: &[bool; BOOL_COUNT], b: &[bool; BOOL_COUNT]) -> usize {
    let mut match_count = 0;
    for i in 0..BOOL_COUNT {
        if a[i] == b[i] {
            match_count += 1;
        }
    }
    match_count
}

fn int_to_bool_array<const COUNT: usize>(int: u32) -> [bool; COUNT] {
    let vec: Vec<bool> = (0..COUNT)
        .map(|i| ((int >> i) & 1) == 1)
        .collect();
    vec.try_into().unwrap()
}

fn index_of<T: PartialEq + DebugTrait>(element: T, vec: &Vec<T>) -> usize {
    vec.iter().position(|e| *e == element).expect(&format!("Didn't find {:?}", element))
}