// Common input data types
use std::fmt::Debug;
use std::str::FromStr;

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
