use std::collections::HashSet;

use aoc21::{input::AoCInput as _, matrix::Ix};
use itertools::Itertools;

aoc21::main!(13);

type Input = (Vec<Ix>, Vec<(bool, usize)>);
type Output = usize;

fn parse(s: &str) -> Input {
    let (dots_str, folds_str) = s.split_once("\n\n").unwrap();
    let dots: Vec<Ix> = Vec::<Ix>::from_input(dots_str);
    let folds = folds_str
        .lines()
        .map(|line| {
            let (along, coord) = line.split_once("=").unwrap();
            (along.ends_with("x"), coord.parse().unwrap())
        })
        .collect_vec();
    (dots, folds)
}

fn part1((dots, folds): &Input) -> Output {
    perform_folds(dots, vec![folder(folds[0])]).len()
}

fn perform_folds(dots: &Vec<Ix>, folds: Vec<impl Fn(Ix) -> Ix>) -> HashSet<Ix> {
    let mut dots: HashSet<Ix> = HashSet::from_iter(dots.clone().into_iter());
    folds.iter().for_each(|f| {
        dots = HashSet::from_iter(dots.clone().into_iter().map(f));
    });
    dots
}

fn folder((fold_x, at): (bool, usize)) -> impl Fn(Ix) -> Ix {
    move |(x, y)| {
        if fold_x && x > at {
            (2 * at - x, y)
        } else if !fold_x && y > at {
            (x, 2 * at - y)
        } else {
            (x, y)
        }
    }
}

fn part2((dots, folds): &Input) -> String {
    let folders = folds.iter().map(|f| folder(*f)).collect_vec();
    let dots = perform_folds(dots, folders);
    let (w, h) = (
        dots.iter().map(|p| p.0).max().unwrap(),
        dots.iter().map(|p| p.1).max().unwrap(),
    );
    let mut out = String::new();
    for y in 0..h + 1 {
        for x in 0..w + 1 {
            if dots.contains(&(x, y)) {
                out += "#";
            } else {
                out += " ";
            }
        }
        out += "\n";
    }
    out
}

#[allow(dead_code)]
const TEST_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

aoc21::test_part1!(TEST_INPUT, 17);
aoc21::test_part2!(
    TEST_INPUT,
    "#####\n#   #\n#   #\n#   #\n#####\n".to_string()
);
