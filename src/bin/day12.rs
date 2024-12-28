use std::collections::{HashMap, HashSet};

use itertools::Itertools;

aoc21::main!(12);

type Input = Vec<(String, String)>;
type Output = usize;

fn parse(s: &str) -> Input {
    s.lines()
        .map(|line| line.split_once('-').unwrap())
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect_vec()
}

fn part1(inp: &Input) -> Output {
    walk_caves(inp, false)
}

fn part2(inp: &Input) -> Output {
    walk_caves(inp, true)
}

fn make_edges<'a>(inp: &'a Input) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in inp {
        edges.entry(a).or_default().insert(b);
        edges.entry(b).or_default().insert(a);
    }
    edges
}

fn walk_caves(inp: &Input, allow_double_cave: bool) -> usize {
    let edges = make_edges(inp);

    let mut stack = Vec::from([("start", HashSet::from(["start"]), allow_double_cave)]);
    let mut paths = 0;
    while let Some((cave, seen, can_double)) = stack.pop() {
        if cave == "end" {
            paths += 1;
            continue;
        }

        for n in edges.get(cave).unwrap() {
            let mut can_double = can_double;
            if seen.contains(n) {
                if can_double && *n != "start" {
                    can_double = false;
                } else {
                    continue;
                }
            }
            let mut new_seen = seen.clone();
            if n.to_lowercase() == *n {
                new_seen.insert(n);
            }
            stack.push((n, new_seen, can_double));
        }
    }
    paths
}

#[allow(dead_code)]
const TEST_INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

aoc21::test_part1!(TEST_INPUT, 226);
aoc21::test_part2!(TEST_INPUT, 3509);
