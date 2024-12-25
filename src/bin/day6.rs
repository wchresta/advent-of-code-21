use aoc21::input::CSVLine;
use memoize::memoize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    aoc21::simple("day6", part1, part2);
}

type Input = CSVLine<u8>;
type Output = u64;

fn part1(inp: &Input) -> Output {
    inp.par_iter().map(|f| fish(*f, 80)).sum()
}

#[memoize]
fn fish(state: u8, time: u16) -> Output {
    if time == 0 {
        1
    } else if state == 0 {
        fish(8, time - 1) + fish(6, time - 1)
    } else {
        fish(state - 1, time - 1)
    }
}

fn part2(inp: &Input) -> Output {
    inp.par_iter().map(|f| fish(*f, 256)).sum()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "3,4,3,1,2";

aoc21::simple_test_part1!(TEST_INPUT, 5934);
aoc21::simple_test_part2!(TEST_INPUT, 26984457539);
