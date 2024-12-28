aoc21::simple_main!(1);

type Input = Vec<usize>;
type Output = usize;

fn part1(inp: &Input) -> Output {
    count_increases(inp.to_owned().into_iter())
}

fn count_increases<'a>(iter: impl Iterator<Item = usize> + Clone) -> usize {
    itertools::multizip((iter.clone(), iter.skip(1)))
        .filter(|(a, b)| a < b)
        .count()
}

fn part2(inp: &Input) -> Output {
    count_increases(
        itertools::multizip((inp.iter(), inp.iter().skip(1), inp.iter().skip(2)))
            .map(|(a, b, c)| *a + *b + *c),
    )
}

#[allow(dead_code)]
const TEST_INPUT: &str = "199
200
208
210
200
207
240
269
260
263";

aoc21::test_part1!(TEST_INPUT, 7);
aoc21::test_part2!(TEST_INPUT, 5);
