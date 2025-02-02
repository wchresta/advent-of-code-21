use iter::{ParallelBridge, ParallelExtend, ParallelIterator};
use itertools::Itertools;
use rayon::*;
use std::collections::HashSet;

aoc21::main!(20);

type Input = (Vec<char>, Sparse);
type Output = usize;

fn parse(inp: &str) -> Input {
    let (top, bot) = inp.split_once("\n\n").unwrap();
    let mut img = Sparse::new(false);
    for (i, line) in bot.lines().enumerate() {
        for (j, char) in line.char_indices() {
            match char {
                '#' => img.set((i as isize, j as isize), true),
                '.' => {}
                _ => panic!("Unexpected char"),
            }
        }
    }

    (top.chars().collect_vec(), img)
}

fn part1((alg, img): &Input) -> Output {
    image_enhance(alg, img, 2).iter_idx().count()
}

fn part2((alg, img): &Input) -> Output {
    image_enhance(alg, img, 50).iter_idx().count()
}

fn image_enhance(alg: &Vec<char>, img: &Sparse, n: usize) -> Sparse {
    let mut img = image_improve(alg, img);
    for _ in 1..n {
        img = image_improve(alg, &img)
    }
    img
}

fn image_improve(alg: &Vec<char>, img: &Sparse) -> Sparse {
    let mut min = (0, 0);
    let mut max = (0, 0);

    for (i, j) in img.iter_idx() {
        min.0 = std::cmp::min(min.0, *i);
        min.1 = std::cmp::min(min.1, *j);
        max.0 = std::cmp::max(max.0, *i);
        max.1 = std::cmp::max(max.1, *j);
    }

    Sparse::from_par_iter(
        if img.empty {
            alg[0x1ff] == '#'
        } else {
            alg[0] == '#'
        },
        (min.0 - 1..max.0 + 2)
            .cartesian_product(min.1 - 1..max.1 + 2)
            .par_bridge()
            .map(|(i, j)| {
                let mut kernel = 0usize;
                for (ki, kj) in (i - 1..i + 2).cartesian_product(j - 1..j + 2) {
                    kernel <<= 1;
                    if img.get((ki, kj)) {
                        kernel |= 1;
                    }
                }
                ((i, j), alg[kernel] == '#')
            }),
    )
}

type Ix = (isize, isize);
#[derive(Debug, Clone, PartialEq, Eq)]
struct Sparse {
    els: HashSet<Ix>,
    empty: bool,
}

impl Sparse {
    fn new(empty: bool) -> Self {
        Self {
            els: HashSet::new(),
            empty,
        }
    }

    fn from_par_iter(empty: bool, iter: impl ParallelIterator<Item = (Ix, bool)>) -> Self {
        let mut els = HashSet::new();
        els.par_extend(iter.filter_map(|(ix, c)| if c != empty { Some(ix) } else { None }));
        Self { els, empty }
    }

    fn get(&self, ix: Ix) -> bool {
        self.els.contains(&ix) ^ self.empty
    }

    fn set(&mut self, ix: Ix, el: bool) {
        if el == self.empty {
            self.els.remove(&ix);
        } else {
            self.els.insert(ix);
        }
    }

    #[allow(dead_code)]
    fn show(&self, (li, lj): Ix, (hi, hj): Ix) -> Vec<String> {
        let mut lines = Vec::new();
        for i in li..hi + 1 {
            let mut line = String::new();
            for j in lj..hj + 1 {
                line += if self.get((i, j)) { "#" } else { "." };
            }
            lines.push(line);
        }
        lines
    }

    fn iter_idx(&self) -> impl Iterator<Item = &Ix> {
        self.els.iter()
    }
}

aoc21::test_part1!(TEST_INPUT, 35);
aoc21::test_part2!(TEST_INPUT, 3351);

#[test]
fn test_enhance1() {
    let (alg, img) = parse(TEST_INPUT);

    let want1 = "...............
...............
...............
...............
.....##.##.....
....#..#.#.....
....##.#..#....
....####..#....
.....#..##.....
......##..#....
.......#.#.....
...............
...............
...............
...............";
    let want2 = "...............
...............
...............
..........#....
....#..#.#.....
...#.#...###...
...#...##.#....
...#.....#.#...
....#.#####....
.....#.#####...
......##.##....
.......###.....
...............
...............
...............";

    let improve1 = image_improve(&alg, &img);
    let improve2 = image_improve(&alg, &improve1);

    assert_eq!(improve1.show((-5, -5), (9, 9)).join("\n"), want1);
    assert_eq!(improve2.show((-5, -5), (9, 9)).join("\n"), want2);
}

#[allow(dead_code)]
const TEST_INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
