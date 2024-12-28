use std::collections::HashMap;

use aoc21::counter::Counter;
use aoc21::input::sections_tuple;
use itertools::Itertools;
use memoize::memoize;

aoc21::main!(14);

type Input = (Vec<char>, Vec<((char, char), char)>);
type Output = u64;

fn parse(s: &str) -> Input {
    sections_tuple(
        s,
        |s| s.chars().collect_vec(),
        |bot| {
            bot.lines()
                .map(|line| {
                    let (i, o) = line.split_once(" -> ").unwrap();
                    (i.chars().next_tuple().unwrap(), o.chars().next().unwrap())
                })
                .collect_vec()
        },
    )
}

fn part1((formula, ins): &Input) -> Output {
    let counts = count_all(ins, formula, 10);
    *counts.max().1 - *counts.min().1
}

fn part2((formula, ins): &Input) -> Output {
    let counts = count_all(ins, formula, 40);
    *counts.max().1 - *counts.min().1
}

fn count_all(ins: &Vec<((char, char), char)>, formula: &Vec<char>, depth: usize) -> Counter<char> {
    let mut count = Counter::new();
    count.count(formula.iter().cloned());
    for (&a, &b) in formula.iter().zip(formula.iter().skip(1)) {
        count.add_from(count_added(ins.clone(), (a, b), depth));
    }
    count
}

#[memoize]
fn count_added(
    ins: Vec<((char, char), char)>,
    formula @ (a, b): (char, char),
    depth: usize,
) -> Counter<char> {
    let mut counter = Counter::new();
    if depth == 0 {
        return counter;
    }

    let lookup = HashMap::<(char, char), char>::from_iter(ins.clone().into_iter());
    if let Some(&mid) = lookup.get(&formula) {
        counter.inc(mid);

        if depth > 1 {
            counter.add_from(count_added(ins.clone(), (a, mid), depth - 1));
            counter.add_from(count_added(ins, (mid, b), depth - 1));
        }
    }

    counter
}

#[allow(dead_code)]
const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

aoc21::test_part1!(TEST_INPUT, 1588);
aoc21::test_part2!(TEST_INPUT, 2188189693529);
