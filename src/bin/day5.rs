use std::collections::HashMap;

use aoc21::input::AoCLineInput;
use aoc21::pos::Pos;
use itertools::Itertools;

fn main() {
    aoc21::solve("day5", parse, part1, part2);
}

type N = i16;
type Input = Vec<(Pos<N>, Pos<N>)>;
type Output = usize;

fn parse(s: &str) -> Input {
    s.lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();
            (<Pos<N>>::from_line(from), Pos::from_line(to))
        })
        .collect_vec()
}

fn part1(inp: &Input) -> Output {
    part2(
        &inp.clone()
            .into_iter()
            .filter(|(a, b)| a.x == b.x || a.y == b.y)
            .collect_vec(),
    )
}

fn part2(inp: &Input) -> Output {
    let mut pixels: HashMap<Pos<N>, u8> = HashMap::new();
    inp.into_iter().for_each(|(from, to)| {
        let diff = *to - *from;
        let diff_len = std::cmp::max(diff.x.abs(), diff.y.abs());
        let step = diff.elementwise_clamp(-1, 1);

        let mut curr = from.clone();
        for _ in 0..diff_len + 1 {
            *pixels.entry(curr).or_default() += 1;
            curr += step;
        }
    });

    pixels.into_values().filter(|x| *x >= 2).count()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

aoc21::test_part1!(TEST_INPUT, 5);
aoc21::test_part2!(TEST_INPUT, 12);
