use std::{
    collections::{BinaryHeap, HashSet},
    ops::Deref,
};

use aoc21::matrix::{Ix, Mat, MatGet};

aoc21::simple_main!(15);

type Input = Mat<u8>;
type Output = u32;

struct ExtMat<'a> {
    mat: &'a Mat<u8>,
    size_mult: usize,
}

impl<'a> Deref for ExtMat<'a> {
    type Target = &'a Mat<u8>;
    fn deref(&self) -> &Self::Target {
        &self.mat
    }
}

impl<'a> MatGet<'a, u8> for ExtMat<'a> {
    fn width(&self) -> usize {
        self.n * self.size_mult
    }

    fn height(&self) -> usize {
        self.m * self.size_mult
    }

    fn get(&self, (i, j): Ix) -> u8 {
        let (quad_i, quad_j) = ((i / self.n) as u8, (j / self.m) as u8);
        (self.mat.get((i % self.n, j % self.n)) + quad_i + quad_j - 1) % 9 + 1
    }
}

fn part1(mat: &Input) -> Output {
    find_shortest_path(mat, 1)
}

fn part2(mat: &Input) -> Output {
    find_shortest_path(mat, 5)
}

fn find_shortest_path(mat: &Input, size_mult: usize) -> Output {
    let mat = ExtMat { mat, size_mult };

    let mut queue = BinaryHeap::new();
    queue.push((0, (0, 0)));
    let mut seen = HashSet::new();
    seen.insert((0, 0));

    while let Some((cost, pos)) = queue.pop() {
        if pos == (mat.height() - 1, mat.width() - 1) {
            return -cost as u32;
        }

        for (p, c) in mat.iter_bordering(pos) {
            if seen.contains(&p) {
                continue;
            }
            seen.insert(p);

            queue.push((cost - c as i32, p));
        }
    }
    panic!("No path found");
}

#[allow(dead_code)]
const TEST_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

aoc21::test_part1!(TEST_INPUT, 40);
aoc21::test_part2!(TEST_INPUT, 315);
