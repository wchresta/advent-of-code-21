use aoc21::pos::Pos;

fn main() {
    aoc21::simple("day2", part1, part2);
}

type Input = Vec<(String, i32)>;
type Output = i32;

fn part1(inp: &Input) -> Output {
    inp.iter()
        .map(|(m, n)| match m.as_str() {
            "forward" => Pos::from_x(*n),
            "up" => Pos::from_y(-*n),
            "down" => Pos::from_y(*n),
            _ => panic!("unknown"),
        })
        .sum::<Pos<Output>>()
        .mul()
}

fn part2(inp: &Input) -> Output {
    let (x, y, _) = inp
        .iter()
        .fold((0, 0, 0), |(x, y, a), (m, n)| match m.as_str() {
            "forward" => (x + n, y + n * a, a),
            "up" => (x, y, a - n),
            "down" => (x, y, a + n),
            _ => panic!("unknown"),
        });
    x * y
}

#[allow(dead_code)]
const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

aoc21::simple_test_part1!(TEST_INPUT, 150);
aoc21::simple_test_part2!(TEST_INPUT, 900);
