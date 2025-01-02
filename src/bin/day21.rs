use std::collections::BTreeMap;

use aoc21::input::AoCInput;
use itertools::Itertools;

aoc21::simple_main!(21);

type N = u64;
type Input = Game;
type Output = N;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord)]
struct Game {
    scores: [N; 2],
    pos: [N; 2],
    player_turn: usize,
    roll_count: N,
}

impl AoCInput for Game {
    fn from_input(s: &str) -> Self {
        let (pos_1, pos_2) = s
            .lines()
            .map(|line| line.chars().last().unwrap().to_digit(10).unwrap() as N)
            .collect_tuple()
            .unwrap();
        Self {
            pos: [pos_1, pos_2],
            scores: [0, 0],
            player_turn: 0,
            roll_count: 0,
        }
    }
}

// Ensure Games are ordered according to their minimal score.
// This will make sure we never process an end state before one before.
impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match std::cmp::min(self.scores[0], self.scores[1])
            .partial_cmp(std::cmp::min(&other.scores[0], &other.scores[1]))
        {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.roll_count.partial_cmp(&other.roll_count) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.player_turn.partial_cmp(&other.player_turn) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.pos.partial_cmp(&other.pos)
    }
}

const DIRAC_ROLLS: [(N, N); 7] = [(1, 3), (3, 4), (6, 5), (7, 6), (6, 7), (3, 8), (1, 9)];

impl Game {
    fn deterministic_roll(&mut self) -> bool {
        self.pos[self.player_turn] = mod1(
            self.pos[self.player_turn]
                + mod1(self.roll_count + 1, 100)
                + mod1(self.roll_count + 2, 100)
                + mod1(self.roll_count + 3, 100),
            10,
        );
        self.roll_count += 3;

        self.scores[self.player_turn] += self.pos[self.player_turn];
        if self.scores[self.player_turn] >= 1000 {
            return true;
        }
        self.player_turn = (self.player_turn + 1) % 2;
        false
    }

    fn dirac_roll(&self) -> [(Self, N); 7] {
        let mut new_game = self.clone();
        new_game.roll_count += 3;
        new_game.player_turn = (self.player_turn + 1) % 2;
        let mut games = [(new_game, 0); 7];
        for (i, (game_n, roll)) in DIRAC_ROLLS.iter().enumerate() {
            let (game, n) = &mut games[i];
            game.pos[self.player_turn] = mod1(game.pos[self.player_turn] + roll, 10);
            game.scores[self.player_turn] += game.pos[self.player_turn];
            *n = *game_n;
        }
        games
    }
}

fn mod1(x: N, n: N) -> N {
    ((x - 1) % n) + 1
}

fn part1(game: &Input) -> Output {
    let mut game = game.clone();
    while !game.deterministic_roll() {}
    game.scores[(game.player_turn + 1) % 2] * game.roll_count
}

fn part2(game: &Input) -> Output {
    // PartialOrd of Game ensures that lowest scores are first.
    let mut games = BTreeMap::<Game, N>::new();
    games.insert(game.clone(), 1);
    let mut player_1_wins = 0;
    let mut player_2_wins = 0;
    while let Some((game, n)) = games.pop_first() {
        for (g, dn) in game.dirac_roll() {
            let game_n = n * dn;
            if g.scores[0] >= 21 {
                player_1_wins += game_n;
            } else if g.scores[1] >= 21 {
                player_2_wins += game_n;
            } else {
                *games.entry(g).or_default() += game_n;
            }
        }
    }
    std::cmp::max(player_1_wins, player_2_wins)
}

aoc21::test_part1!(TEST_INPUT, 739785);
aoc21::test_part2!(TEST_INPUT, 444356092776315);

#[allow(dead_code)]
const TEST_INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

/*
Notes:


1,1,1  --> 3
1,1,2  --> 4
1,1,3  --> 5

1,2,1  --> 4
1,2,2  --> 5
1,2,3  --> 6

1,3,1  --> 5
1,3,2  --> 6
1,3,3  --> 7

2,1,1  --> 4
2,1,2  --> 5
2,1,3  --> 6

2,2,1  --> 5
2,2,2  --> 6
2,2,3  --> 7

2,3,1  --> 6
2,3,2  --> 7
2,3,3  --> 8

3,1,1  --> 5
3,1,2  --> 6
3,1,3  --> 7

3,2,1  --> 6
3,2,2  --> 7
3,2,3  --> 8

3,3,1  --> 7
3,3,2  --> 8
3,3,3  --> 9


Total:

1 x 3
3 x 4
6 x 5
7 x 6
6 x 7
3 x 8
1 x 9


*/
