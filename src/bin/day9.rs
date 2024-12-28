use std::collections::HashSet;

use aoc21::matrix::{Ix, Mat};
use itertools::Itertools;

aoc21::simple_main!(9);

type Input = Mat<u8>;
type Output = u32;

fn part1(inp: &Input) -> Output {
    low_points(inp).map(|(_, h)| *h as u32 + 1).sum()
}

fn low_points(mat: &Input) -> impl Iterator<Item = (Ix, &u8)> {
    mat.iter()
        .filter(|(pos, val)| *val < mat.iter_bordering_el(*pos).min().unwrap())
}

fn part2(inp: &Input) -> Output {
    low_points(inp)
        .map(|(low_pos, low_height)| {
            let mut stack = vec![(low_pos, low_height)];
            let mut seen = HashSet::from([low_pos]);
            let mut size = 0;
            while let Some((pos, height)) = stack.pop() {
                size += 1;

                for (np, nh) in inp.iter_bordering(pos) {
                    if *nh == 9 || nh <= height || seen.contains(&np) {
                        continue;
                    }
                    seen.insert(np);
                    stack.push((np, nh));
                }
            }
            size
        })
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

aoc21::test_part1!(TEST_INPUT, 15);
aoc21::test_part2!(TEST_INPUT, 1134);
