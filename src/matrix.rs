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
    pub fn iter(&self) -> impl Iterator<Item = (Ix, &T)> {
        (0..self.m).into_iter().flat_map(move |i| {
            (0..self.n)
                .into_iter()
                .map(move |j| ((i, j), &self.mat[i][j]))
        })
    }

    pub fn iter_bordering(&self, (x, y): Ix) -> impl Iterator<Item = (Ix, &T)> {
        let mut pos = Vec::new();
        if x > 0 {
            pos.push((x - 1, y))
        }
        if y > 0 {
            pos.push((x, y - 1))
        }
        if x - 1 < self.n {
            pos.push((x + 1, y))
        }
        if y - 1 < self.m {
            pos.push((x, y + 1))
        }
        pos.into_iter().map(|p| (p, &self.mat[p.0][p.1]))
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
                if line.contains(" ") {
                    line.split_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect_vec()
                } else {
                    // Assume one character for every element.
                    line.chars()
                        .map(|num| num.to_string().parse().unwrap())
                        .collect_vec()
                }
            })
            .collect_vec();
        let (m, n) = (mat.len(), mat[0].len());
        Self { mat, m, n }
    }
}
