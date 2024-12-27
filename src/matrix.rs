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
    pub m: usize,
    pub n: usize,
}

impl<T> Mat<T> {
    pub fn get(&self, ix: Ix) -> &T {
        &self.mat[ix.0][ix.1]
    }

    pub fn get_mut(&mut self, ix: Ix) -> &mut T {
        &mut self.mat[ix.0][ix.1]
    }

    pub fn iter_idx(&self) -> impl Iterator<Item = Ix> {
        let (m, n) = (self.m, self.n);
        (0..m).flat_map(move |i| (0..n).map(move |j| (i, j)))
    }

    pub fn iter(&self) -> impl Iterator<Item = (Ix, &T)> {
        self.iter_idx().map(|pos| (pos, self.get(pos)))
    }

    pub fn iter_bordering_idx(&self, (i, j): Ix) -> impl Iterator<Item = Ix> {
        let mut pos = Vec::new();
        if i > 0 {
            pos.push((i - 1, j))
        }
        if j > 0 {
            pos.push((i, j - 1))
        }
        if i + 1 < self.m {
            pos.push((i + 1, j))
        }
        if j + 1 < self.n {
            pos.push((i, j + 1))
        }
        pos.into_iter()
    }

    pub fn iter_bordering(&self, pos: Ix) -> impl Iterator<Item = (Ix, &T)> {
        self.iter_bordering_idx(pos).map(|ix| (ix, self.get(ix)))
    }

    pub fn iter_bordering_el(&self, pos: Ix) -> impl Iterator<Item = &T> {
        self.iter_bordering_idx(pos).map(|pos| self.get(pos))
    }

    pub fn iter_diag_bordering_idx(&self, (i, j): Ix) -> impl Iterator<Item = Ix> {
        let mut pos = Vec::new();
        if i > 0 {
            pos.push((i - 1, j));
            if j > 0 {
                pos.push((i - 1, j - 1));
            }
            if j + 1 < self.n {
                pos.push((i - 1, j + 1));
            }
        }
        if j > 0 {
            pos.push((i, j - 1));
        }
        if i + 1 < self.m {
            pos.push((i + 1, j));
            if j > 0 {
                pos.push((i + 1, j - 1));
            }
            if j + 1 < self.n {
                pos.push((i + 1, j + 1));
            }
        }
        if j + 1 < self.n {
            pos.push((i, j + 1));
        }
        pos.into_iter()
    }

    pub fn iter_diag_bordering(&self, pos: Ix) -> impl Iterator<Item = (Ix, &T)> {
        self.iter_diag_bordering_idx(pos)
            .map(|ix| (ix, self.get(ix)))
    }

    pub fn iter_diag_bordering_el(&self, pos: Ix) -> impl Iterator<Item = &T> {
        self.iter_diag_bordering_idx(pos).map(|pos| self.get(pos))
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
