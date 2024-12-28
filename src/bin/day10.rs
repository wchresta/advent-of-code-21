use itertools::Itertools;

aoc21::simple_main!(10);

type Input = Vec<String>;
type Output = isize;

fn part1(inp: &Input) -> Output {
    inp.into_iter()
        .map(find_error)
        .map(|e| match e {
            Err(')') => 3,
            Err(']') => 57,
            Err('}') => 1197,
            Err('>') => 25137,
            _ => 0,
        })
        .sum()
}

fn find_error(s: &String) -> Result<Vec<char>, char> {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            ')' => {
                if Some('(') != stack.pop() {
                    return Err(')');
                }
            }
            '}' => {
                if Some('{') != stack.pop() {
                    return Err('}');
                }
            }
            ']' => {
                if Some('[') != stack.pop() {
                    return Err(']');
                }
            }
            '>' => {
                if Some('<') != stack.pop() {
                    return Err('>');
                }
            }
            p => {
                stack.push(p);
            }
        }
    }
    Ok(stack)
}

fn part2(inp: &Input) -> Output {
    let scores = inp
        .into_iter()
        .filter_map(|line| match find_error(line) {
            Ok(stack) => Some(stack),
            _ => None,
        })
        .map(|stack| {
            stack
                .into_iter()
                .rev()
                .map(|c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("invalid"),
                })
                .fold(0, |score, add| 5 * score + add)
        })
        .sorted()
        .collect_vec();
    scores[scores.len() / 2]
}

#[allow(dead_code)]
const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

aoc21::test_part1!(TEST_INPUT, 26397);
aoc21::test_part2!(TEST_INPUT, 288957);
