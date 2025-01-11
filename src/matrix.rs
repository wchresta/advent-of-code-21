use itertools::Itertools;

use crate::input::AoCInput;

pub type Ix = (usize, usize);

trait MatEl {}
impl MatEl for char {}
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

pub trait MatGet<'a, T: 'a> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn get(&self, ix: Ix) -> T;

    fn iter_idx(&self) -> impl Iterator<Item = Ix> {
        let (m, n) = (self.height(), self.width());
        (0..m).flat_map(move |i| (0..n).map(move |j| (i, j)))
    }

    fn iter(&'a self) -> impl Iterator<Item = (Ix, T)> {
        self.iter_idx().map(|pos| (pos, self.get(pos)))
    }

    fn iter_bordering_idx(&self, (i, j): Ix) -> impl Iterator<Item = Ix> {
        let mut pos = Vec::new();
        if i > 0 {
            pos.push((i - 1, j))
        }
        if j > 0 {
            pos.push((i, j - 1))
        }
        if i + 1 < self.height() {
            pos.push((i + 1, j))
        }
        if j + 1 < self.width() {
            pos.push((i, j + 1))
        }
        pos.into_iter()
    }

    fn iter_bordering(&'a self, pos: Ix) -> impl Iterator<Item = (Ix, T)> {
        self.iter_bordering_idx(pos).map(|ix| (ix, self.get(ix)))
    }

    fn iter_bordering_el(&'a self, pos: Ix) -> impl Iterator<Item = T> {
        self.iter_bordering_idx(pos).map(|pos| self.get(pos))
    }

    fn iter_diag_bordering_idx(&self, (i, j): Ix) -> impl Iterator<Item = Ix> {
        let mut pos = Vec::new();
        if i > 0 {
            pos.push((i - 1, j));
            if j > 0 {
                pos.push((i - 1, j - 1));
            }
            if j + 1 < self.width() {
                pos.push((i - 1, j + 1));
            }
        }
        if j > 0 {
            pos.push((i, j - 1));
        }
        if i + 1 < self.height() {
            pos.push((i + 1, j));
            if j > 0 {
                pos.push((i + 1, j - 1));
            }
            if j + 1 < self.width() {
                pos.push((i + 1, j + 1));
            }
        }
        if j + 1 < self.width() {
            pos.push((i, j + 1));
        }
        pos.into_iter()
    }

    fn iter_diag_bordering(&'a self, pos: Ix) -> impl Iterator<Item = (Ix, T)> {
        self.iter_diag_bordering_idx(pos)
            .map(|ix| (ix, self.get(ix)))
    }

    fn iter_diag_bordering_el(&'a self, pos: Ix) -> impl Iterator<Item = T> {
        self.iter_diag_bordering_idx(pos).map(|pos| self.get(pos))
    }
}

pub trait MatMut<'a, T: 'a> {
    fn get_mut(&'a mut self, ix: Ix) -> &'a mut T;
}

#[derive(Debug, Clone)]
pub struct Mat<T> {
    pub mat: Vec<Vec<T>>,
    pub m: usize,
    pub n: usize,
}

impl<'a, T: Copy + 'a> MatGet<'a, T> for Mat<T> {
    fn height(&self) -> usize {
        self.m
    }

    fn width(&self) -> usize {
        self.n
    }

    fn get(&self, (i, j): Ix) -> T {
        self.mat[i][j]
    }
}

impl<'a, T: 'a> MatMut<'a, T> for Mat<T> {
    fn get_mut(&mut self, (i, j): Ix) -> &mut T {
        &mut self.mat[i][j]
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

#[derive(Debug, Clone)]
pub struct Torus<T> {
    pub mat: Vec<Vec<T>>,
    pub m: usize,
    pub n: usize,
}

impl<T> Torus<T> {
    pub fn east(&self, (i, j): Ix) -> Ix {
        (i, (j + 1) % self.n)
    }

    pub fn west(&self, (i, j): Ix) -> Ix {
        (i, (j + self.n - 1) % self.n)
    }

    pub fn south(&self, (i, j): Ix) -> Ix {
        ((i + 1) % self.m, j)
    }

    pub fn north(&self, (i, j): Ix) -> Ix {
        ((i + self.m - 1) % self.m, j)
    }
}

impl<'a, T: Copy + 'a> MatGet<'a, T> for Torus<T> {
    fn height(&self) -> usize {
        self.m
    }

    fn width(&self) -> usize {
        self.n
    }

    fn get(&self, (i, j): Ix) -> T {
        self.mat[i][j]
    }

    fn iter_bordering_idx(&self, (i, j): Ix) -> impl Iterator<Item = Ix> {
        [
            ((i + self.m - 1) % self.m, j),
            ((i + 1) % self.m, j),
            (i, (j + self.n - 1) % self.n),
            (i, (j + 1) % self.n),
        ]
        .into_iter()
    }

    fn iter_diag_bordering_idx(&self, (i, j): Ix) -> impl Iterator<Item = Ix> {
        let (il, ir) = ((i + self.m - 1) % self.m, (i + 1) % self.m);
        let (jl, jr) = ((j + self.n - 1) % self.n, (j + 1) % self.n);
        [
            (il, jl),
            (il, j),
            (il, jr),
            (i, jl),
            (i, jr),
            (ir, jl),
            (ir, j),
            (ir, jr),
        ]
        .into_iter()
    }
}

impl<'a, T: 'a> MatMut<'a, T> for Torus<T> {
    fn get_mut(&mut self, (i, j): Ix) -> &mut T {
        &mut self.mat[i][j]
    }
}

impl<T> AoCInput for Torus<T>
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

impl<T: std::fmt::Display> std::fmt::Display for Torus<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.m {
            for j in 0..self.n {
                write!(f, "{}", self.mat[i][j])?
            }
            writeln!(f)?
        }
        Ok(())
    }
}
