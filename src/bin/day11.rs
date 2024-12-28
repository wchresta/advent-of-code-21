use std::collections::HashSet;

use aoc21::matrix::{Ix, Mat};

aoc21::simple_main!(11);

type Input = Mat<u8>;
type Output = usize;

fn part1(inp: &Input) -> Output {
    let mut mat = inp.clone();
    (0..100).map(|_| power_up(&mut mat)).sum()
}

fn part2(inp: &Input) -> Output {
    let mut mat = inp.clone();
    let mut step = 0;
    let want = inp.m * inp.n;
    loop {
        step += 1;
        if power_up(&mut mat) == want {
            break;
        }
    }
    step
}

fn power_up(mat: &mut Input) -> usize {
    let mut flash_stack = Vec::new();
    let inc = |m: &mut Input, f: &mut Vec<Ix>, pos| {
        let val = m.get_mut(pos);
        *val += 1;
        if *val > 9 {
            f.push(pos);
        }
    };

    mat.iter_idx().for_each(|p| inc(mat, &mut flash_stack, p));

    let mut flashed = HashSet::new();
    while let Some(pos) = flash_stack.pop() {
        if flashed.contains(&pos) {
            continue;
        }
        flashed.insert(pos);
        mat.iter_diag_bordering_idx(pos).for_each(|p| {
            inc(mat, &mut flash_stack, p);
        })
    }

    for f in &flashed {
        *mat.get_mut(*f) = 0;
    }

    flashed.len()
}

#[allow(dead_code)]
const TEST_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

aoc21::test_part1!(TEST_INPUT, 1656);
aoc21::test_part2!(TEST_INPUT, 195);
