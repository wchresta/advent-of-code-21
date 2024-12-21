// Common input data types
use std::fmt::Debug;
use std::str::FromStr;

pub trait AoCInput {
    fn from_input(input: &str) -> Self;
}

impl<T> AoCInput for Vec<T>
where
    T: FromStr,
    T::Err: Debug,
{
    fn from_input(input: &str) -> Self {
        input.lines().map(|line| line.parse().unwrap()).collect()
    }
}
