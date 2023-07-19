use std::ops::{Neg, Index};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    NEG,//negative
    POS //positive
}
impl Sign {
    pub fn map<T>(&self, if_neg: T, if_pos: T) -> T {
        match self {
            Self::NEG => if_neg,
            Self::POS => if_pos
        }
    }
}
impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::NEG => Self::POS,
            Self::POS => Self::NEG
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CornerSigns<const N: usize>(pub [Sign; N]);
impl<const N: usize> CornerSigns<N> {
    pub const NEG: Self = Self([Sign::NEG; N]);
    pub const POS: Self = Self([Sign::POS; N]);

    pub fn all() -> Vec<Self> {
        all_bool_arrays()
            .map(|bools| bools.into())
            .collect()
    }

    pub fn match_count(&self, other: &Self) -> usize {
        self.0.iter().zip(other.0.iter())
            .filter(|(a, b)| a == b)
            .count()
        // (0..2)
        //     .map(|i| a.0.test(i) == b.0.test(i))
        //     .filter(|&x| x)
        //     .count()
    }

    pub fn map<T: Copy + std::fmt::Debug>(&self, if_neg: T, if_pos: T) -> [T; N] {
        self.0.map(|c| c.map(if_neg, if_pos))
    }
    // pub fn to_vec2(&self, if_false: f32, if_true: f32) -> Vec2 {
    //     Vec2::new(
    //         if self.0.x {if_true} else {if_false},
    //         if self.0.y {if_true} else {if_false}
    //     )
    // }
}
impl<const N: usize> From<[bool; N]> for CornerSigns<N> {
    fn from(value: [bool; N]) -> Self {
        Self(value.map(|b| if b {Sign::NEG} else {Sign::POS}))
    }
}
impl<const N: usize> Index<usize> for CornerSigns<N> {
    type Output = Sign;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
// impl<const N: usize> std::ops::Neg for CornerSigns<N> {
//     type Output = Self;

//     fn neg(self) -> Self::Output {
//         Self(self.0.map(|c| -c))
//     }
// }
// impl From<[bool; 2]> for CornerSigns2 {
//     fn from(value: [bool; 2]) -> Self {
//         Self::new()
//     }
// }

// fn all_BVec2() -> Vec<BVec2> {
//     vec![
//         BVec2::new(false, false),
//         BVec2::new(false, true ),
//         BVec2::new(true , false),
//         BVec2::new(true , true )
//     ]
// }
// fn all_BVec3() -> Vec<BVec3> {
//     vec![
//         BVec3::new(false, false, false),
//         BVec3::new(false, false, true ),
//         BVec3::new(false, true , false),
//         BVec3::new(false, true , true ),
//         BVec3::new(true , false, false),
//         BVec3::new(true , false, true ),
//         BVec3::new(true , true , false),
//         BVec3::new(true , true , true )
//     ]
// }

// fn BVec2_match_count(a: BVec2, b: BVec2) -> usize {
//     (0..2)
//         .map(|i| a.test(i) == b.test(i))
//         .filter(|&x| x)
//         .count()
// }
// fn BVec3_match_count(a: BVec3, b: BVec3) -> usize {
//     (0..3)
//         .map(|i| a.test(i) == b.test(i))
//         .filter(|&x| x)
//         .count()
// }

fn all_bool_arrays<const BOOL_COUNT: usize>() -> impl Iterator<Item = [bool; BOOL_COUNT]> {
    (0..(1 << BOOL_COUNT))
        .map(|int| int_to_bool_array(int))
}

// fn bool_array_match_count<const BOOL_COUNT: usize>(A: &[bool; BOOL_COUNT], B: &[bool; BOOL_COUNT]) -> usize {
//     A.iter().zip(B.iter())
//         .filter(|(&a, &b)| a == b)
//         .count()
// }

fn int_to_bool_array<const N: usize>(int: u32) -> [bool; N] {
    array_0_to_N().map(|i| ((int >> i) & 1) == 1)
}

fn array_0_to_N<const N: usize>() -> [usize; N] {
    let mut array = [0; N];
    for i in 0..N {
        array[i] = i;
    }
    array
}

// fn collect_array<T, const N: usize>(iter: impl Iterator<Item = T>) -> [T; N]
//     where T: std::fmt::Debug
// {
//     iter.collect::<Vec<T>>().try_into().unwrap()
// }
