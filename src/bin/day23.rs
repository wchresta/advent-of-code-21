use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc21::input::AoCInput;
use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

aoc21::simple_main!(23);

type Input = String;
type Output = u32;

fn part1(s: &Input) -> Output {
    let state = State::from_input(s);
    find_shortest(&state).unwrap().cost
}


fn part2(s: &Input) -> Output {
    let mut extended_input = s.lines().collect_vec();
    extended_input.insert(3, "  #D#C#B#A#");
    extended_input.insert(4, "  #D#B#A#C#");

    let state = State::from_input(&extended_input.join("\n"));

    find_shortest(&state).unwrap().cost
}

fn find_shortest(state: &State) -> Option<State> {
    let mut states = BinaryHeap::new();
    let mut seen = HashSet::new();
    states.push(state.clone());
    seen.insert(state.clone());
    while let Some(state) = states.pop() {
        if state.is_final() {
            return Some(state);
        }
        let moves = state.moves();
        for new_state in moves
            .par_iter()
            .filter_map(|m| {
                let mut new_state = state.clone();
                new_state.make_move(*m);
                if seen.contains(&new_state) {
                    None
                } else {
                    Some(new_state)
                }
            })
            .collect::<Vec<State>>()
        {
            seen.insert(new_state.clone());
            states.push(new_state);
        }
    }
    None
}

type Pod = u8;
type Energy = u32;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pos {
    Room(u8),
    Hallway(u8),
}

impl Pos {
    fn is_hallway(&self) -> bool {
        match self {
            Pos::Room(_) => false,
            Pos::Hallway(_) => true,
        }
    }
}

/*
Hallway
 01 2 3 4 56
   0 1 2 3
 */

lazy_static! {
    static ref EDGES: HashMap<Pos, Vec<(Pos, Energy)>> = {
        [
            // Room depth is later added to cost
            (Pos::Room(0), (Pos::Hallway(1), 1)),
            (Pos::Room(0), (Pos::Hallway(2), 1)),
            (Pos::Room(1), (Pos::Hallway(2), 1)),
            (Pos::Room(1), (Pos::Hallway(3), 1)),
            (Pos::Room(2), (Pos::Hallway(3), 1)),
            (Pos::Room(2), (Pos::Hallway(4), 1)),
            (Pos::Room(3), (Pos::Hallway(4), 1)),
            (Pos::Room(3), (Pos::Hallway(5), 1)),
            (Pos::Hallway(0), (Pos::Hallway(1), 1)),
            (Pos::Hallway(1), (Pos::Hallway(2), 2)),
            (Pos::Hallway(2), (Pos::Hallway(3), 2)),
            (Pos::Hallway(3), (Pos::Hallway(4), 2)),
            (Pos::Hallway(4), (Pos::Hallway(5), 2)),
            (Pos::Hallway(5), (Pos::Hallway(6), 1)),
        ]
        .into_iter()
        .fold(HashMap::new(), |mut acc, (from, (to, cost))| {
            acc.entry(from).or_default().push((to, cost));
            acc.entry(to).or_default().push((from, cost));
            acc
        })
    };
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct State {
    // We have 7 hallway positions and 16 rooms = 23 tiles.
    // Each of them requires 3 bits to encode Pod.
    // So in total we need 69 bits.
    // In part 2, we need 16 more rooms, so 39 tiles,
    // which results in 117 bits.
    // Thus, a state can be represented with u128
    state: u128,
    cost: u32,
    depth: u8,
}

impl State {
    fn new(flat_pods: &[char]) -> Self {
        let mut s = Self { state: 0, cost: 0, depth: 2 };
        if flat_pods.len() > 8 {
            s.depth = 4;
        }

        for i in 0..7+8*s.depth {
            s.set_tile(i, None);
        } 

        for (room_idx, &pod) in flat_pods.into_iter().enumerate().rev() {
            let room = Pos::Room(room_idx as u8 % 4);
            s.put(room, pod as u8 - 'A' as u8);
        }

        s
    }

    fn get_tile(&self, tile: u8) -> Option<Pod> {
        match (self.state >> (3 * tile)) & 0b111 {
            0 => Some(0),
            1 => Some(1),
            2 => Some(2),
            3 => Some(3),
            7 => None,
            _ => panic!("invalid pod"),
        }
    }

    fn get_tile_char(&self, tile: u8) -> char {
        match self.get_tile(tile) {
            Some(0) => 'A',
            Some(1) => 'B',
            Some(2) => 'C',
            Some(3) => 'D',
            None => ' ',
            _ => panic!("not valid tile"),
        }
    }

    fn set_tile(&mut self, tile: u8, pod: Option<Pod>) {
        let bits = match pod {
            Some(0) => 0,
            Some(1) => 1,
            Some(2) => 2,
            Some(3) => 3,
            None => 7,
            _ => panic!("invalid pod"),
        };

        let mask = 0b111 << (3 * tile);
        self.state &= !mask;
        self.state |= bits << (3 * tile);
    }

    fn get(&self, pos: Pos) -> Option<Pod> {
        match pos {
            Pos::Hallway(n) => self.get_tile(n),
            Pos::Room(n) => {
                let base = 7 + 4 * n;
                for i in 0..self.depth {
                    let t = self.get_tile(base + i);
                    if t.is_some() {
                        return t;
                    }
                }
                None
            }
        }
    }

    fn remove(&mut self, pos: Pos) {
        match pos {
            Pos::Hallway(n) => self.set_tile(n, None),
            Pos::Room(n) => {
                let base = 7 + 4 * n;
                for i in 0..self.depth {
                    if self.get_tile(base + i).is_some() {
                        self.set_tile(base + i, None);
                        return;
                    }
                }
            }
        }
    }

    fn put(&mut self, pos: Pos, pod: Pod) {
        match pos {
            Pos::Hallway(n) => self.set_tile(n, Some(pod)),
            Pos::Room(n) => {
                let base = 7 + 4 * n;
                for i in 0..self.depth-1 {
                    if self.get_tile(base + i+1).is_some() {
                        self.set_tile(base + i, Some(pod));
                        return;
                    }
                }
                self.set_tile(base+self.depth-1, Some(pod));
            }
        }
    }

    fn free_room_spots(&self, room: u8) -> u32 {
        let base = 7 + 4 * room;
        for i in 0..self.depth {
            if self.get_tile(base + i).is_some() {
                return i as u32;
            }
        }
        self.depth as u32
    }

    fn pods<'a>(&'a self) -> impl Iterator<Item = (Pod, Pos)> + 'a {
        [
            Pos::Hallway(0),
            Pos::Hallway(1),
            Pos::Hallway(2),
            Pos::Hallway(3),
            Pos::Hallway(4),
            Pos::Hallway(5),
            Pos::Hallway(6),
            Pos::Room(0),
            Pos::Room(1),
            Pos::Room(2),
            Pos::Room(3),
        ]
        .into_iter()
        .filter_map(|pos| {
            if let Some(pod) = self.get(pos) {
                Some((pod, pos))
            } else {
                None
            }
        })
    }

    fn moves(&self) -> Vec<(Pos, Pos, Pod, Energy)> {
        let mut moves = Vec::new();

        'pod_moves:
        for (pod, pos) in self.pods() {
            //println!("Check {} at {:?}", pod, pos);
            if let Pos::Room(room) = pos {
                if room == pod && self.room_has_only(room, pod) {
                    // Is already in the target room and there is nobody behind.
                    continue;
                }
            }

            let mut stack = vec![(pos, 0)];
            let mut seen = HashSet::from([pos]);
            let mut pod_moves = Vec::new();
            while let Some((cur_pos, cur_cost)) = stack.pop() {
                let mut room_exit_cost = 0;
                if let Pos::Room(room) = cur_pos {
                    let free_spots = self.free_room_spots(room);
                    assert_ne!(free_spots, self.depth as u32, "Pod {} tried to exit {:?} originally coming from {:?} but there are 2 free spots??:\n{}", pod, cur_pos, pos, self);
                    room_exit_cost += free_spots+1;
                }

                for &(next_pos, mut new_cost) in &EDGES[&cur_pos] {
                    if seen.contains(&next_pos) {
                        continue;
                    }
                    seen.insert(next_pos);

                    if let Pos::Room(room) = next_pos {
                        // If room is closed or the current pod does not belong in the room, we don't care.
                        if !self.room_open(room) || room != pod {
                            continue;
                        }

                        // For every free spot, we need to take one more step going into a room
                        let free_spots = self.free_room_spots(room);
                        assert_ne!(free_spots, 0, "Pod {} tried to enter {} but there are no free spots:\n{}", pod, room, self);
                        new_cost += free_spots;
                    }

                    if next_pos.is_hallway() && self.get(next_pos).is_some() {
                        // Occupied
                        continue;
                    }

                    new_cost += room_exit_cost;
                    let new_cost = match pod {
                        0 => new_cost,
                        1 => 10 * new_cost,
                        2 => 100 * new_cost,
                        3 => 1000 * new_cost,
                        _ => panic!("unknown pod"),
                    };

                    if next_pos.is_hallway() {
                        // Entering a room is the last move. We never enter and exit a room in the same move.
                        stack.push((next_pos, cur_cost + new_cost));
                    }

                    // Cannot move into hallway if we are already in the hallway
                    if pos.is_hallway() && next_pos.is_hallway() {
                        //println!("Move {:?} not moving into hallway", next_pos);
                        continue;
                    }

                    pod_moves.push((pos, next_pos, pod, cur_cost + new_cost));
                }
            }

            // Check if any of the moves can move home
            // If so, ignore all other moves.
            for m@(_, next_pos, pod, _) in &pod_moves {
                if let Pos::Room(room) = next_pos {
                    if pod == room {
                        moves.push(*m);
                        continue 'pod_moves;
                    }
                }
            }
            moves.extend(pod_moves.into_iter());
        }
        moves
    }

    fn make_move(&mut self, (frm, to, pod, cost): (Pos, Pos, Pod, Energy)) {
        self.cost += cost;
        self.remove(frm);
        self.put(to, pod);
    }

    fn room_open(&self, room: u8) -> bool {
        self.room_has_only(room, room)
    }

    fn room_has_only(&self, room: u8, pod: Pod) -> bool {
        for i in 0..self.depth {
            if let Some(found) = self.get_tile(7 + 4 * room + i) {
                if found != pod {
                    return false;
                }
            }
        }
        return true;
    }


    fn is_final(&self) -> bool {
        // Ensure hallway is empty
        for i in 0..7 {
            if self.get(Pos::Hallway(i)).is_some() {
                return false;
            }
        }

        // Ensure all rooms are clean
        for i in 0..4 {
            if !self.room_has_only(i, i) {
                return false;
            }
        }
        true
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "#{}{} {} {} {} {}{}#\n###{}#{}#{}#{}###\n###{}#{}#{}#{}###\n###{}#{}#{}#{}###\n###{}#{}#{}#{}###",
            self.get_tile_char(0),
            self.get_tile_char(1),
            self.get_tile_char(2),
            self.get_tile_char(3),
            self.get_tile_char(4),
            self.get_tile_char(5),
            self.get_tile_char(6),
 
            self.get_tile_char(7),
            self.get_tile_char(11),
            self.get_tile_char(15),
            self.get_tile_char(19),           

            self.get_tile_char(8),
            self.get_tile_char(12),
            self.get_tile_char(16),
            self.get_tile_char(20),

            self.get_tile_char(9),
            self.get_tile_char(13),
            self.get_tile_char(17),
            self.get_tile_char(21),

            self.get_tile_char(10),
            self.get_tile_char(14),
            self.get_tile_char(18),
            self.get_tile_char(22),
        )
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.cost.partial_cmp(&other.cost) {
            Some(core::cmp::Ordering::Equal) => {}
            Some(ord) => return Some(ord.reverse()),
            None => return None,
        }
        self.state.partial_cmp(&other.state)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.cost.cmp(&other.cost) {
            std::cmp::Ordering::Equal => {}
            ord => return ord.reverse(),
        }

        self.state.cmp(&other.state)
    }
}

impl AoCInput for State {
    fn from_input(s: &str) -> Self {
        let lines = s.lines().skip(2);
        let mut elements = Vec::new();
        for line in lines {
            line.chars().filter(|&c| c != '#' && c != ' ').for_each(|c| elements.push(c))
        }
        Self::new(&elements)
    }
}

aoc21::test_part1!(TEST_INPUT, 12521);
aoc21::test_part2!(TEST_INPUT, 44169);

#[allow(dead_code)]
const TEST_INPUT: &str = "#############
#01.2.3.4.56#
###B#C#B#D###
  #A#D#C#A#
  #########";
