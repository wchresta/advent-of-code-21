use std::collections::{HashMap, HashSet};

use aoc21::{
    input::{self, AoCInput},
    matrix::{Ix, Mat},
};

fn main() {
    aoc21::solve("day4", parse, part1, part2);
}

type Input = (Vec<u32>, Vec<Mat<u32>>);
type Output = u32;

fn parse(s: &str) -> Input {
    let (draw, boards_str) = s.split_once("\n\n").unwrap();
    let boards = boards_str.split("\n\n").map(Mat::from_input).collect();
    (input::split_line_on(draw, ','), boards)
}

fn part1(inp @ (_, boards): &Input) -> Output {
    let (winners, marked) = run_winners(inp, true);
    let (win_draw, win_k) = winners[0];
    let mut sum = 0;
    for (ij, v) in boards[win_k].iter_idx() {
        if !marked.contains(&(win_k, ij)) {
            sum += v;
        }
    }

    sum * win_draw
}

fn run_winners(
    (draw, boards): &Input,
    find_first: bool,
) -> (Vec<(u32, usize)>, HashSet<(usize, Ix)>) {
    let mut num_to_pos = HashMap::new();
    let mut marked = HashSet::new();
    for (k, b) in boards.iter().enumerate() {
        b.iter_idx().for_each(|(ij, n)| {
            num_to_pos.insert((k, n), ij);
        })
    }

    let mut playing: HashSet<usize> = HashSet::from_iter(0..boards.len());
    let mut winners = Vec::new();
    'drawLoop: for d in draw {
        for k in playing.clone().into_iter() {
            let b = &boards[k];
            if let Some((i, j)) = num_to_pos.get(&(k, d)) {
                marked.insert((k, (*i, *j)));

                // Check rows and columns for bingo
                if (0..b.m).all(|i| marked.contains(&(k, (i, *j))))
                    || (0..b.n).all(|j| marked.contains(&(k, (*i, j))))
                {
                    winners.push((*d, k));
                    playing.remove(&k);
                    if find_first {
                        break 'drawLoop;
                    }
                }
            }
        }
    }
    (winners, marked)
}

fn part2(inp @ (_, boards): &Input) -> Output {
    let (winners, marked) = run_winners(inp, false);
    let (win_draw, win_k) = winners[winners.len() - 1];
    let mut sum = 0;
    for (ij, v) in boards[win_k].iter_idx() {
        if !marked.contains(&(win_k, ij)) {
            sum += v;
        }
    }

    sum * win_draw
}

#[allow(dead_code)]
const TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

aoc21::test_part1!(TEST_INPUT, 4512);
aoc21::test_part2!(TEST_INPUT, 1924);
