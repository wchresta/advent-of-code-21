use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use crate::input::AoCLineInput;

#[derive(Debug, Clone, Copy)]
pub struct Pos<T> {
    pub x: T,
    pub y: T,
}

impl<T: Default> Pos<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from_x(x: T) -> Self {
        Self::new(x, T::default())
    }

    pub fn from_y(y: T) -> Self {
        Self::new(T::default(), y)
    }

    pub fn zero() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<T> Pos<T>
where
    T: Mul<Output = T> + Copy,
{
    pub fn mul(&self) -> T {
        self.x * self.y
    }
}

impl<T: Display> Display for Pos<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl<T: PartialEq> PartialEq for Pos<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
    }
}

impl<T: Eq> Eq for Pos<T> {}

impl<T: std::hash::Hash> std::hash::Hash for Pos<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T: Ord + Copy> Pos<T> {
    pub fn elementwise_clamp(&self, min: T, max: T) -> Self {
        Self {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }
}

impl<T: Default> Default for Pos<T> {
    fn default() -> Self {
        Pos::zero()
    }
}

impl<T> Add for Pos<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Pos<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Pos<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> SubAssign for Pos<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Sum for Pos<T>
where
    T: AddAssign,
    T: Default,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut tot = Pos::default();
        for t in iter {
            tot += t;
        }
        tot
    }
}

impl<T> AoCLineInput for Pos<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    fn from_line(s: &str) -> Self {
        let (x, y) = <(T, T)>::from_line(s);
        Pos { x, y }
    }
}
