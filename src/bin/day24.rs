use aoc21::input::AoCInput;
use itertools::Itertools;
use memoize::memoize;
use rayon::iter::{IntoParallelIterator, ParallelIterator as _};

aoc21::main!(24);

type Input = Vec<Block>;
type Output = N;

fn parse(s: &str) -> Input {
    s.split("inp w\n")
        .skip(1)
        .map(Block::from_input)
        .collect_vec()
}

fn part1(blocks: &Input) -> Output {
    find_largest_input(blocks, 0)
        .unwrap()
        .into_iter()
        .fold(0, |acc, n| acc * 10 + n as Output)
}

fn part2(blocks: &Input) -> Output {
    find_smallest_input(blocks, 0)
        .unwrap()
        .into_iter()
        .fold(0, |acc, n| acc * 10 + n as Output)
}

fn find_largest_input(blocks: &[Block], want_z: N) -> Option<Vec<u8>> {
    if blocks.len() == 0 && want_z == 0 {
        return Some(Vec::new());
    }

    let sub_blocks = &blocks[..blocks.len() - 1];
    let possible_outs = cached_reverse(blocks.last().unwrap().clone(), want_z)
        .into_iter()
        .flat_map(|(z, w)| {
            //println!("Finding depth={} z={:6} w={}", blocks.len(), z, w);
            if let Some(mut num) = find_largest_input(sub_blocks, z) {
                num.push(w);
                return Some(num);
            }
            None
        });
    possible_outs.max()
}

fn find_smallest_input(blocks: &[Block], want_z: N) -> Option<Vec<u8>> {
    if blocks.len() == 0 && want_z == 0 {
        return Some(Vec::new());
    }

    let sub_blocks = &blocks[..blocks.len() - 1];
    let possible_outs = cached_reverse(blocks.last().unwrap().clone(), want_z)
        .into_iter()
        .flat_map(|(z, w)| {
            if let Some(mut num) = find_smallest_input(sub_blocks, z) {
                num.push(w);
                return Some(num);
            }
            None
        });
    possible_outs.min()
}

type N = i64;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Block {
    a: N,
    b: N,
    div_26: bool,
}

impl Block {
    fn run(&self, mut z: N, input: u8) -> N {
        let test = (z % 26) + self.a != (input as N);
        if self.div_26 {
            z /= 26;
        }
        if test {
            z = 26 * z + (input as N) + self.b;
        }
        z
    }

    fn reverse(&self, want_z: N) -> Vec<(N, u8)> {
        let mut nums: Vec<(N, u8)> = (1..10)
            .into_par_iter()
            .flat_map(|w| {
                // We want high number last.
                let mut nums = Vec::new();
                for z in 0..350_000 {
                    if self.run(z, w) == want_z {
                        nums.push((z, w));
                    }
                }
                nums
            })
            .collect();
        nums.sort_by_key(|(_, n)| 255 - n);
        nums
    }
}

#[memoize]
fn cached_reverse(block: Block, want_z: N) -> Vec<(N, u8)> {
    block.reverse(want_z)
}

impl AoCInput for Block {
    fn from_input(s: &str) -> Self {
        let mut lines = s.lines().skip(3);
        let div_26 = match lines.next().unwrap() {
            "div z 1" => false,
            "div z 26" => true,
            _ => panic!("expected div z"),
        };
        let a = lines
            .next()
            .unwrap()
            .split(" ")
            .skip(2)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let mut lines = lines.skip(9);
        let b = lines
            .next()
            .unwrap()
            .split(" ")
            .skip(2)
            .next()
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(lines.count(), 2);
        Self { a, b, div_26 }
    }
}
