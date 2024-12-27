use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use itertools::Itertools;
use lazy_static::lazy_static;

fn main() {
    aoc21::solve("day8", parse, part1, part2);
}

type Input = Vec<(Vec<String>, Vec<String>)>;
type Output = u32;

fn parse(s: &str) -> Input {
    s.lines()
        .map(|line| {
            let (left, right) = line.split_once(" | ").unwrap();
            (
                left.split_whitespace().map(String::from).collect_vec(),
                right.split_whitespace().map(String::from).collect_vec(),
            )
        })
        .collect_vec()
}

/*
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....
  6       2        5       5      4

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
   5      6       3       7       6

Unique: 2,3,4,7
*/

lazy_static! {
    static ref UNIQUE: HashSet<u8> = HashSet::from([2, 3, 4, 7]);
}
//                  abcdefg
const SEG_A: u8 = 0b1000000;
const SEG_B: u8 = 0b0100000;
const SEG_C: u8 = 0b0010000;
const SEG_D: u8 = 0b0001000;
const SEG_E: u8 = 0b0000100;
const SEG_F: u8 = 0b0000010;
const SEG_G: u8 = 0b0000001;

const BCD_0: u8 = 0b1110111;
const BCD_1: u8 = 0b0010010;
const BCD_2: u8 = 0b1011101;
const BCD_3: u8 = 0b1011011;
const BCD_4: u8 = 0b0111010;
const BCD_5: u8 = 0b1101011;
const BCD_6: u8 = 0b1101111;
const BCD_7: u8 = 0b1010010;
const BCD_8: u8 = 0b1111111;
const BCD_9: u8 = 0b1111011;
const BCD: [u8; 10] = [
    BCD_0, BCD_1, BCD_2, BCD_3, BCD_4, BCD_5, BCD_6, BCD_7, BCD_8, BCD_9,
];

const UNKNOWN: u8 = 0b1111111;

fn part1(inp: &Input) -> Output {
    inp.iter()
        .flat_map(|f| &f.1)
        .filter(|word| UNIQUE.contains(&(word.len() as u8)))
        .count() as u32
}

struct Encodings([u8; 7]);

impl Encodings {
    fn new() -> Self {
        Self([
            UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN, UNKNOWN,
        ])
    }

    fn get_mut(&mut self, in_seg: char) -> &mut u8 {
        &mut self.0[Self::idx(in_seg)]
    }

    fn for_each_by_bit(&mut self, in_bits: u8, mut f: impl FnMut(&mut u8)) {
        let in_bits = 0x7f & in_bits;
        for bit in 1..8 {
            if in_bits & (1 << bit) != 0 {
                f(self.get_by_bit_mut(1 << bit))
            }
        }
    }

    fn get_by_bit_mut(&mut self, in_bit: u8) -> &mut u8 {
        match in_bit {
            SEG_A => &mut self.0[0],
            SEG_B => &mut self.0[1],
            SEG_C => &mut self.0[2],
            SEG_D => &mut self.0[3],
            SEG_E => &mut self.0[4],
            SEG_F => &mut self.0[5],
            SEG_G => &mut self.0[6],
            _ => panic!("non unique bit: {:07b}", in_bit),
        }
    }

    fn seg(c: char) -> u8 {
        match c {
            'a' => SEG_A,
            'b' => SEG_B,
            'c' => SEG_C,
            'd' => SEG_D,
            'e' => SEG_E,
            'f' => SEG_F,
            'g' => SEG_G,
            _ => panic!("err"),
        }
    }

    fn idx(c: char) -> usize {
        match c {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            _ => panic!("unknown c"),
        }
    }

    fn decode(word: &str) -> u8 {
        word.chars().fold(0, |acc, c| acc | Self::seg(c))
    }

    fn decode_table(&self) -> HashMap<u8, u8> {
        HashMap::from_iter((0..10).map(|n| {
            let bcd = BCD[n];
            let mut code = 0u8;
            for seg in 0..7 {
                if bcd & self.0[seg] != 0 {
                    code |= 1 << (6 - seg);
                }
            }
            (code, n as u8)
        }))
    }

    fn encode(v: &u8) -> String {
        let mut out = String::new();
        for c in "abcdefg".chars() {
            if v & Self::seg(c) != 0 {
                out += &c.to_string();
            } else {
                out += " ";
            }
        }
        out
    }

    fn consume(&mut self, word: &str) {
        match word.len() {
            2 => word.chars().for_each(|c| *self.get_mut(c) &= BCD_1),
            3 => word.chars().for_each(|c| *self.get_mut(c) &= BCD_7),
            4 => word.chars().for_each(|c| *self.get_mut(c) &= BCD_4),
            7 => word.chars().for_each(|c| *self.get_mut(c) &= BCD_8),
            _ => {}
        }
    }

    /*
      0:      1:      2:      3:      4:
     aaaa    ....    aaaa    aaaa    ....
    b    c  .    c  .    c  .    c  b    c
    b    c  .    c  .    c  .    c  b    c
     ....    ....    dddd    dddd    dddd
    e    f  .    f  e    .  .    f  .    f
    e    f  .    f  e    .  .    f  .    f
     gggg    ....    gggg    gggg    ....
      6       2        5       5      4

      5:      6:      7:      8:      9:
     aaaa    aaaa    aaaa    aaaa    aaaa
    b    .  b    .  .    c  b    c  b    c
    b    .  b    .  .    c  b    c  b    c
     dddd    dddd    ....    dddd    dddd
    .    f  e    f  .    f  e    f  .    f
    .    f  e    f  .    f  e    f  .    f
     gggg    gggg    ....    gggg    gggg
       5      6       3       7       6

    Unique: 2,3,4,7
    */

    /// All characters in `word` must map to one of `options`.
    fn solve(&mut self) -> bool {
        let mut progress = false;

        // Check if we have loners
        for i in 0..7 {
            let a = self.0[i];
            if a.count_ones() == 1 {
                // Nobody else can have this bit.
                for j in 0..7 {
                    if i == j {
                        continue;
                    }

                    if self.0[j] & a != 0 {
                        progress = true;
                    }
                    self.0[j] &= !a;
                }
            }
        }

        // Check if we have two encodings that must encode the same two bits.
        for ((i, a), (j, b)) in self.0.clone().iter().enumerate().tuple_combinations() {
            if a == b && a.count_ones() == 2 {
                // a and b MUST have the two bits; nobody else can have it.
                let mask = !a;
                for k in 0..7 {
                    if k == i || k == j {
                        continue;
                    }
                    if self.0[k] & !mask != 0 {
                        progress = true;
                    }
                    self.0[k] &= mask;
                }
            }
        }

        progress
    }
}

impl std::fmt::Debug for Encodings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(Self::encode).join("|"))
    }
}

fn part2(inp: &Input) -> Output {
    let by_word_len = |words: &Vec<String>, len| {
        words
            .iter()
            .filter(move |w| w.len() == len)
            .map(|e| Encodings::decode(e))
            .collect_vec()
    };

    inp.iter()
        .map(|(ins, outs)| {
            let mut encodings = Encodings::new();
            let words = [ins.clone(), outs.clone()].concat();
            words.iter().for_each(|word| encodings.consume(word));

            let encoded_adg = by_word_len(&words, 5)
                .into_iter()
                .reduce(|x, y| x & y)
                .unwrap();
            encodings.for_each_by_bit(encoded_adg, |b| *b &= SEG_A | SEG_D | SEG_G);
            encodings.for_each_by_bit(!encoded_adg, |b| *b &= !(SEG_A | SEG_D | SEG_G));

            let encoded_seven = by_word_len(&words, 3)[0];
            for five in by_word_len(&words, 5) {
                // "3" is the only 5-segment number that leaves 2 segments if we remove the 7-segments.
                let encoded_dg = five & !encoded_seven;
                if encoded_dg.count_ones() == 2 {
                    // We know left_segments are d and g.
                    encodings.for_each_by_bit(encoded_dg, |b| *b &= SEG_D | SEG_G);
                    // We also know nobody else can be d or g.
                    encodings.for_each_by_bit(!encoded_dg, |b| *b &= !(SEG_D | SEG_G));
                }
            }

            let encoded_one = by_word_len(&words, 2)[0];
            for six in by_word_len(&words, 6) {
                let encoded_f = six & encoded_one;
                if encoded_f.count_ones() == 1 {
                    // We know we have combined 1 and 6, so the segment must be f.
                    *encodings.get_by_bit_mut(encoded_f) = SEG_F;
                    // And nobody else can be f.
                    encodings.for_each_by_bit(!encoded_f, |b| *b &= !SEG_F);
                }
            }

            let encoded_a = encoded_seven & !encoded_one;
            *encodings.get_by_bit_mut(encoded_a) = SEG_A;
            encodings.for_each_by_bit(!encoded_a, |b| *b &= !SEG_A);

            while encodings.solve() {}

            let decode_table = encodings.decode_table();
            assert_eq!(decode_table.len(), 10);
            let mut val = 0;
            for out in outs {
                val = 10 * val + *decode_table.get(&Encodings::decode(out)).unwrap() as u32;
            }
            val
        })
        .sum()
}

#[allow(dead_code)]
const TEST_INPUT: &str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

aoc21::test_part1!(TEST_INPUT, 26);
aoc21::test_part2!(TEST_INPUT, 61229);
