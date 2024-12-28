use aoc21::input::CSVLine;
use itertools::Itertools;

aoc21::simple_main!(7);

type N = u32;
type Input = CSVLine<N>;
type Output = N;

fn part1(inp: &Input) -> Output {
    let len = inp.len();
    let median = inp.iter().sorted().skip(len / 2 - 1).next().unwrap();
    inp.iter().map(|x| x.abs_diff(*median)).sum()
}

fn part2(inp: &Input) -> Output {
    let min = *inp.iter().min().unwrap();
    let max = *inp.iter().max().unwrap();
    (min..max + 1)
        .map(|m| {
            inp.iter()
                .map(|t| {
                    let diff = t.abs_diff(m);
                    (diff * (diff + 1)) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

aoc21::test_part1!(TEST_INPUT, 37);
aoc21::test_part2!(TEST_INPUT, 168);
