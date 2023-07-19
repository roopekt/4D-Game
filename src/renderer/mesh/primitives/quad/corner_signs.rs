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
    }

    pub fn map<T: Copy + std::fmt::Debug>(&self, if_neg: T, if_pos: T) -> [T; N] {
        self.0.map(|c| c.map(if_neg, if_pos))
    }
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

fn all_bool_arrays<const BOOL_COUNT: usize>() -> impl Iterator<Item = [bool; BOOL_COUNT]> {
    (0..(1 << BOOL_COUNT))
        .map(|int| int_to_bool_array(int))
}

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
