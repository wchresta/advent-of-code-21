use std::cmp::max;

use itertools::Itertools as _;
use nom::{bytes::complete::tag, character::complete::u32, IResult};

aoc21::main!(18);

type Input = Vec<Pair>;
type Output = u32;

fn pair<'a>(input: &'a str) -> IResult<&'a str, Pair> {
    (|input: &'a str| {
        let (input, _) = tag("[")(input)?;
        let (input, left) = pair(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, right) = pair(input)?;
        let (input, _) = tag("]")(input)?;
        Ok((input, Pair::Pair(Box::new((left, right)))))
    })(input)
    .or_else(|_: nom::Err<_>| {
        let (input, num) = u32(input)?;
        Ok((input, Pair::Num(num)))
    })
}

fn parse(s: &str) -> Input {
    s.lines().map(|line| pair(line).unwrap().1).collect_vec()
}

#[derive(Clone)]
enum Pair {
    Num(u32),
    Pair(Box<(Pair, Pair)>),
}

#[derive(Debug)]
enum Put {
    Left(u32),
    Right(u32),
    Done,
}

impl Pair {
    pub fn pair(left: Self, right: Self) -> Self {
        Self::Pair(Box::new((left, right)))
    }

    pub fn add(self, other: Self) -> Self {
        let mut result = Self::pair(self, other);
        result.reduce();
        result
    }

    pub fn is_num(&self) -> bool {
        match self {
            Pair::Num(_) => true,
            Pair::Pair(_) => false,
        }
    }

    pub fn as_pair(&self) -> &(Pair, Pair) {
        if let Self::Pair(bx) = self {
            return bx.as_ref();
        }
        panic!("not a pair");
    }

    pub fn as_mut_pair(&mut self) -> &mut (Pair, Pair) {
        if let Self::Pair(bx) = self {
            return bx.as_mut();
        }
        panic!("not a pair");
    }

    pub fn as_num_pair(&self) -> (u32, u32) {
        if let Self::Pair(bx) = self {
            if let &(Self::Num(l), Self::Num(r)) = bx.as_ref() {
                return (l, r);
            }
            panic!("not a num pair");
        }
        panic!("not a pair");
    }

    pub fn reduce(&mut self) {
        loop {
            if self.explode(0).is_some() {
                //println!("after explode:  {:?}", self);
                continue;
            }
            if self.split() {
                //println!("after split:    {:?}", self);
                continue;
            }
            //println!("after reducing: {:?}", self);
            break;
        }
    }

    fn explode(&mut self, depth: u8) -> Option<Put> {
        if self.is_num() {
            return None;
        }

        let (self_left, self_right) = self.as_mut_pair();
        if depth >= 3 {
            // We need to explode all parent pairs.
            if !self_left.is_num() {
                let (ex_l, ex_r) = self_left.as_num_pair();
                *self_left = Pair::Num(0);
                self_right.push_left(ex_r);
                return Some(Put::Left(ex_l));
            }

            if !self_right.is_num() {
                let (ex_l, ex_r) = self_right.as_num_pair();
                *self_right = Pair::Num(0);
                self_left.push_right(ex_l);
                return Some(Put::Right(ex_r));
            }
            return None;
        }

        if let Some(put) = self_left.explode(depth + 1) {
            if let Put::Right(ex_r) = put {
                self_right.push_left(ex_r);
                return Some(Put::Done);
            }
            return Some(put);
        }

        if let Some(put) = self_right.explode(depth + 1) {
            if let Put::Left(ex_l) = put {
                self_left.push_right(ex_l);
                return Some(Put::Done);
            }
            return Some(put);
        }

        None
    }

    /// Push n to the leftmost regular number.
    fn push_left(&mut self, n: u32) {
        match self {
            Pair::Num(num) => *num += n,
            pair => {
                let (left, _) = pair.as_mut_pair();
                left.push_left(n);
            }
        }
    }

    /// Push n to the rightmost regular number.
    fn push_right(&mut self, n: u32) {
        match self {
            Pair::Num(num) => *num += n,
            pair => {
                let (_, right) = pair.as_mut_pair();
                right.push_right(n);
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Pair::Num(num) => {
                if *num >= 10 {
                    *self = Self::pair(Self::Num(*num / 2), Self::Num(num.div_ceil(2)));
                    return true;
                }
                false
            }
            Pair::Pair(bx) => {
                let (left, right) = bx.as_mut();
                left.split() || right.split()
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Pair::Num(num) => *num,
            pair => {
                let (left, right) = pair.as_pair();
                3 * left.magnitude() + 2 * right.magnitude()
            }
        }
    }
}

impl std::fmt::Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(num) => num.fmt(f),
            Self::Pair(pair) => {
                let (a, b) = pair.as_ref();
                write!(f, "[{:?},{:?}]", a, b)
            }
        }
    }
}

fn sum(pairs: &Vec<Pair>) -> Pair {
    pairs.iter().cloned().reduce(|a, b| a.add(b)).unwrap()
}

fn part1(inp: &Input) -> Output {
    sum(inp).magnitude()
}

fn part2(inp: &Input) -> Output {
    let mut hi = 0;
    for (a, b) in inp.iter().tuple_combinations() {
        hi = max(hi, a.clone().add(b.clone()).magnitude());
        hi = max(hi, b.clone().add(a.clone()).magnitude());
    }
    hi
}

#[allow(dead_code)]
const TEST_INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

aoc21::test_part1!(TEST_INPUT, 4140);
aoc21::test_part2!(TEST_INPUT, 3993);

#[test]
fn test_reduce() {
    let mut pair = parse("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]")
        .first()
        .unwrap()
        .clone();
    pair.reduce();
    assert_eq!(format!("{:?}", pair), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
}

#[test]
fn test_sum() {
    assert_eq!(
        format!(
            "{:?}",
            sum(&parse(
                "[1,1]
[2,2]
[3,3]
[4,4]"
            ))
        ),
        "[[[[1,1],[2,2]],[3,3]],[4,4]]"
    );

    assert_eq!(
        format!(
            "{:?}",
            sum(&parse(
                "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]"
            ))
        ),
        "[[[[3,0],[5,3]],[4,4]],[5,5]]"
    );

    assert_eq!(
        format!(
            "{:?}",
            sum(&parse(
                "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]"
            ))
        ),
        "[[[[5,0],[7,4]],[5,5]],[6,6]]"
    );

    assert_eq!(
        format!(
            "{:?}",
            sum(&parse(
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"
            ))
        ),
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
    );
}
