use aoc21::matrix::{MatGet as _, MatMut as _, Torus};

aoc21::simple_main!(25);

type Input = Torus<char>;
type Output = usize;

fn part1(mat: &Input) -> Output {
    let mut tor = mat.clone();

    let mut moved = true;
    let mut steps = 0;

    while moved {
        let prev_tor = tor.clone();

        moved = false;
        steps += 1;
        // East
        for i in 0..tor.m {
            for j in 0..tor.n {
                let o = tor.east((i, j));
                if prev_tor.get((i, j)) == '>' && prev_tor.get(o) == '.' {
                    moved = true;
                    *tor.get_mut((i, j)) = '.';
                    *tor.get_mut(o) = '>';
                }
            }
        }

        let prev_tor = tor.clone();
        // South
        for i in 0..tor.m {
            for j in 0..tor.n {
                let o = tor.south((i, j));
                if prev_tor.get((i, j)) == 'v' && prev_tor.get(o) == '.' {
                    moved = true;
                    *tor.get_mut((i, j)) = '.';
                    *tor.get_mut(o) = 'v';
                }
            }
        }
    }

    steps
}

fn part2(_mat: &Input) -> Output {
    // There does not exist a part 2.
    0
}

aoc21::test_part1!(TEST_INPUT, 58);
aoc21::test_part2!(TEST_INPUT, 0);

#[allow(dead_code)]
const TEST_INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
