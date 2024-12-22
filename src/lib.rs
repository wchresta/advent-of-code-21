use input::AoCInput;

pub mod input;
pub mod matrix;
pub mod pos;

pub fn input_string(day: &str) -> String {
    let path = format!("inputs/{}", day);
    std::fs::read_to_string(&path).expect(&format!("input file {} not found", &path))
}

pub fn simple<T, O1, O2>(day: &str, part1: impl Fn(&T) -> O1, part2: impl Fn(&T) -> O2)
where
    T: AoCInput,
    O1: std::fmt::Display,
    O2: std::fmt::Display,
{
    solve(day, T::from_input, part1, part2)
}

pub fn solve<T, O1, O2>(
    day: &str,
    parser: impl Fn(&str) -> T,
    part1: impl Fn(&T) -> O1,
    part2: impl Fn(&T) -> O2,
) where
    O1: std::fmt::Display,
    O2: std::fmt::Display,
{
    let inp = input_string(day);
    let t = parser(&inp);

    let start = std::time::Instant::now();
    let out1 = part1(&t);
    println!(
        "Part1 in {:.5} seconds:\n{}\n",
        start.elapsed().as_secs_f32(),
        out1,
    );

    let start = std::time::Instant::now();
    let out2 = part2(&t);
    println!(
        "Part2 in {:.5} seconds:\n{}",
        start.elapsed().as_secs_f32(),
        out2,
    );
}

pub fn simple_test<T, O>(inp: &str, want: O, part: impl Fn(&T) -> O)
where
    T: AoCInput,
    O: std::fmt::Display + Eq + std::fmt::Debug,
{
    test(inp, want, T::from_input, part)
}

pub fn test<T, O>(inp: &str, want: O, parser: impl Fn(&str) -> T, part: impl Fn(&T) -> O)
where
    O: std::fmt::Display + Eq + std::fmt::Debug,
{
    let t = parser(inp);
    assert_eq!(want, part(&t));
}

#[macro_export]
macro_rules! simple_test_part1 {
    ( $inp:expr, $want:expr ) => {
        #[test]
        fn simple_test_part1() {
            aoc21::simple_test($inp, $want, part1);
        }
    };
}

#[macro_export]
macro_rules! simple_test_part2 {
    ( $inp:expr, $want:expr ) => {
        #[test]
        fn simple_test_part2() {
            aoc21::simple_test($inp, $want, part2);
        }
    };
}

#[macro_export]
macro_rules! test_part1 {
    ( $inp:expr, $want:expr ) => {
        #[test]
        fn simple_test_part1() {
            aoc21::test($inp, $want, parse, part1);
        }
    };
}

#[macro_export]
macro_rules! test_part2 {
    ( $inp:expr, $want:expr ) => {
        #[test]
        fn simple_test_part2() {
            aoc21::test($inp, $want, parse, part2);
        }
    };
}
