use itertools::Itertools;

fn main() {
    aoc21::simple("day3", part1, part2);
}

type Input = Vec<Vec<u32>>;
type Output = u32;

fn part1(inp: &Input) -> Output {
    let n = inp[0].len();
    let gamma = bits_to_num(&most_common(inp));
    gamma * (gamma ^ ((1 << n) - 1))
}

fn bits_to_num(bits: &Vec<u32>) -> u32 {
    bits.into_iter().fold(0, |acc, b| (acc << 1) + b)
}

fn most_common(inp: &Input) -> Vec<u32> {
    let n = inp.len() as u32;
    elementwise_sum(inp)
        .iter()
        .map(|sum| if *sum > n / 2 { 1 } else { 0 })
        .collect_vec()
}

fn elementwise_sum(inp: &Input) -> Vec<u32> {
    (0..inp[0].len()).map(|row| sum_row(inp, row)).collect_vec()
}

fn sum_row(inp: &Input, row: usize) -> u32 {
    inp.into_iter().map(|v| v[row]).sum()
}

fn part2(inp: &Input) -> Output {
    let oxygen_r = rating(inp, false);
    let co2_scrubber_r = rating(inp, true);
    oxygen_r * co2_scrubber_r
}

fn rating(inp: &Input, invert: bool) -> u32 {
    let mut candidates = inp.clone();
    let n = candidates[0].len();
    for i in 0..n {
        let sum = sum_row(&candidates, i);
        let keep = if (2 * sum >= candidates.len() as u32) ^ invert {
            1
        } else {
            0
        };
        candidates = candidates
            .into_iter()
            .filter(|c| c[i] == keep)
            .collect_vec();
        if candidates.len() == 1 {
            return bits_to_num(&candidates[0]);
        }
    }
    panic!("No candidate found")
}

#[allow(dead_code)]
const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

aoc21::simple_test_part1!(TEST_INPUT, 198);
aoc21::simple_test_part2!(TEST_INPUT, 230);
