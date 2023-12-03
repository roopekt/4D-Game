use std::ops::{Add, AddAssign};
use std::iter::Sum;
use super::{Mesh3D, Mesh4D};
use crate::combinations::combinations_constsize_owned;
use std::collections::HashSet;

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
            .map(|&primitive| combinations_constsize_owned::<2,_,_>(primitive))
            .flatten()
            .collect();
        self.skeleton_indeces = edge_set.iter().copied().collect();
        self
    }
}

impl Mesh3D {
    pub fn attach_skeleton(&mut self, mut skeleton_source: Self) {
        skeleton_source = skeleton_source.with_full_skeleton();
        skeleton_source.indeces.clear();
        
        self.skeleton_indeces.clear();
        *self += skeleton_source;
    }
}
impl Mesh4D {
    pub fn attach_skeleton(&mut self, mut skeleton_source: Self) {
        skeleton_source = skeleton_source.with_full_skeleton();
        skeleton_source.indeces.clear();
        
        self.skeleton_indeces.clear();
        *self += skeleton_source;
    }
}
