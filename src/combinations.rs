use std::fmt::Debug;
use std::iter;
use std::ops::IndexMut;

pub fn combinations_constsize<'a, const COMBINATION_SIZE: usize, T>(source: &'a [T])
    -> impl Iterator<Item = [T; COMBINATION_SIZE]> + 'a
    where T: Copy + Debug + 'static
{
    combinations_constsize_closure(|i| source[i], source.len())
}

pub fn combinations_constsize_owned<const COMBINATION_SIZE: usize, T, I>(source: I)
    -> impl Iterator<Item = [T; COMBINATION_SIZE]>
    where T: Copy + Debug + 'static, I: IntoIterator<Item = T> + IndexMut<usize, Output = T> + Length
{
    let length = source.length();
    combinations_constsize_closure(move |i| source[i], length)
}

pub fn combinations_constsize_range<const COMBINATION_SIZE: usize, T>(range: std::ops::Range<T>)
    -> impl Iterator<Item = [T; COMBINATION_SIZE]>
    where T: Copy + Debug + 'static + num::Integer + num::NumCast
{
    let length = <usize as num::NumCast>::from(range.end - range.start).unwrap();//range.start is inclusive, range.end exclusive
    let getter = move |i: usize| -> T {
        range.start + T::from(i).unwrap()
    };
    combinations_constsize_closure(getter, length)
}

fn combinations_constsize_closure<const COMBINATION_SIZE: usize, T, F>(source: F, source_length: usize)
    -> impl Iterator<Item = [T; COMBINATION_SIZE]>
    where T: Debug + 'static, F: Fn(usize) -> T
{
    fn advance_counter<const N: usize>(counter: &mut [usize; N]) {
        counter[0] += 1;
        let mut i = 0;
        while counter.get(i + 1) == Some(&counter[i]) {
            counter[i] = i;
            i += 1;
            counter[i] += 1;
        }
    }
    fn get_combination<T, const N: usize, F: Fn(usize) -> T>(source: &F, counter: &[usize; N]) -> [T; N] {
        counter.map(|i| source(i))
    }
    
    let mut counter = array_0_to_N::<COMBINATION_SIZE>();
    iter::repeat_with(move || {
            let out = counter.clone();
            advance_counter(&mut counter);
            out
        })
        .take_while(move |counter| counter.last().map_or(false, |&c| c < source_length))
        .map(move |counter| get_combination(&source, &counter))
}

fn array_0_to_N<const N: usize>() -> [usize; N] {
    let mut array = [0; N];
    for i in 0..N {
        array[i] = i;
    }
    array
}

pub trait Length {
    fn length(&self) -> usize;
}
impl<T> Length for Vec<T> {
    fn length(&self) -> usize {
        self.len()
    }
}
impl<T, const N: usize> Length for [T; N] {
    fn length(&self) -> usize {
        self.len()
    }
}