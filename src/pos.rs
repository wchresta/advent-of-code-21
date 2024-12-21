use std::{
    iter::Sum,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct Pos<T> {
    x: T,
    y: T,
}

impl<T: Default> Pos<T> {
    pub fn from_x(x: T) -> Self {
        Self { x, y: T::default() }
    }

    pub fn from_y(y: T) -> Self {
        Self { x: T::default(), y }
    }

    pub fn zero() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
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
