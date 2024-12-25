// Common input data types
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use itertools::Itertools;

pub trait AoCInput {
    fn from_input(s: &str) -> Self;
}

impl<T> AoCInput for Vec<T>
where
    T: AoCLineInput,
{
    fn from_input(s: &str) -> Self {
        s.lines().map(|line| T::from_line(line)).collect()
    }
}

pub trait AoCLineInput {
    fn from_line(s: &str) -> Self;
}

trait AocLineParsed {}
impl AocLineParsed for String {}
impl AocLineParsed for u8 {}
impl AocLineParsed for u16 {}
impl AocLineParsed for u32 {}
impl AocLineParsed for u64 {}
impl AocLineParsed for usize {}
impl AocLineParsed for i8 {}
impl AocLineParsed for i16 {}
impl AocLineParsed for i32 {}
impl AocLineParsed for i64 {}
impl AocLineParsed for isize {}

impl<T> AoCLineInput for T
where
    T: FromStr + AocLineParsed,
    T::Err: Debug,
{
    fn from_line(s: &str) -> Self {
        s.parse().unwrap()
    }
}

impl<A, B> AoCLineInput for (A, B)
where
    A: FromStr,
    A::Err: Debug,
    B: FromStr,
    B::Err: Debug,
{
    fn from_line(s: &str) -> Self {
        let (a, b) = s.split_once(',').or(s.split_once(' ')).unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    }
}

impl AoCLineInput for Vec<u32> {
    fn from_line(s: &str) -> Self {
        s.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct CSVLine<T>(pub Vec<T>);

impl<T> CSVLine<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl<T> Deref for CSVLine<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for CSVLine<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AoCInput for CSVLine<T>
where
    T: FromStr,
    T::Err: Debug,
{
    fn from_input(s: &str) -> Self {
        Self(split_line_on(s, ','))
    }
}

// Specialized parsers
pub fn split_line_on<T>(s: &str, sep: char) -> Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    s.split(sep).map(|n| n.parse().unwrap()).collect_vec()
}
