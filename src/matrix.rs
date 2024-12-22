use itertools::Itertools;

use crate::input::AoCInput;

pub type Ix = (usize, usize);

trait MatEl {}
impl MatEl for u8 {}
impl MatEl for u16 {}
impl MatEl for u32 {}
impl MatEl for u64 {}
impl MatEl for usize {}
impl MatEl for i8 {}
impl MatEl for i16 {}
impl MatEl for i32 {}
impl MatEl for i64 {}
impl MatEl for isize {}

#[derive(Debug, Clone)]
pub struct Mat<T> {
    pub mat: Vec<Vec<T>>,
    pub n: usize,
    pub m: usize,
}

impl<T> Mat<T> {
    pub fn iter_idx(&self) -> impl Iterator<Item = (Ix, &T)> {
        (0..self.m).into_iter().flat_map(move |i| {
            (0..self.n)
                .into_iter()
                .map(move |j| ((i, j), &self.mat[i][j]))
        })
    }
}

impl<T> AoCInput for Mat<T>
where
    T: MatEl + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    fn from_input(s: &str) -> Self {
        let mat = s
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect_vec()
            })
            .collect_vec();
        let (m, n) = (mat.len(), mat[0].len());
        Self { mat, m, n }
    }
}
