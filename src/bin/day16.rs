use aoc21::input::AoCInput;
use itertools::Itertools;

aoc21::simple_main!(16);

type Input = Packet;
type Output = usize;

struct Packet(Vec<u8>);

impl AoCInput for Packet {
    fn from_input(s: &str) -> Self {
        let cs = s
            .chars()
            .map(|c| c.to_digit(16).unwrap() as u8)
            .collect_vec();
        let mut bytes = Vec::new();
        for chunk in cs.chunks(2) {
            if let [h, l] = chunk {
                bytes.push((h << 4) | l);
            } else {
                bytes.push(chunk[0] << 4);
            }
        }
        Packet(bytes)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SubType {
    Len(u16),
    PacketCount(u16),
}

impl SubType {
    pub fn sub_packet_rel_start(&self) -> usize {
        match self {
            Self::Len(_) => 7 + 15,
            Self::PacketCount(_) => 7 + 11,
        }
    }
}

type Lit = u64;

impl Packet {
    pub fn bits(&self, from: usize, len: usize) -> u16 {
        if len == 0 {
            panic!("cannot get len 0");
        }
        if len > 16 {
            panic!("requested more than 16 bits");
        }

        let from_chunk = from / 8;
        let mut num = (self.0[from_chunk] as u32) << 24;
        if (from % 8) + len > 8 {
            num |= (self.0[from_chunk + 1] as u32) << 16;
        }
        if (from % 8) + len > 16 {
            num |= (self.0[from_chunk + 2] as u32) << 8;
        }

        // 012345678012345678
        //   2----------12
        num >>= 32 - len - (from % 8);
        num &= (1 << len) - 1;
        num as u16
    }

    pub fn version(&self, packet_start: usize) -> u8 {
        self.bits(packet_start, 3) as u8
    }

    pub fn type_id(&self, packet_start: usize) -> u8 {
        self.bits(packet_start + 3, 3) as u8
    }

    pub fn length_type_id(&self, packet_start: usize) -> u8 {
        self.bits(packet_start + 6, 1) as u8
    }

    pub fn is_literal(&self, packet_start: usize) -> bool {
        self.type_id(packet_start) == 4
    }

    pub fn literal(&self, packet_start: usize) -> (Lit, usize) {
        assert_eq!(self.type_id(packet_start), 4, "Literal type must be 4");

        let mut start = packet_start + 6;
        let mut num: Lit = 0;
        loop {
            let bits = self.bits(start, 5);
            num <<= 4;
            num |= (0xf & bits) as Lit;
            start += 5;
            if 0x10 & bits == 0 {
                break;
            }
        }
        (num, start)
    }

    pub fn sub_packet_type(&self, packet_start: usize) -> SubType {
        assert!(!self.is_literal(packet_start));

        if self.length_type_id(packet_start) == 0 {
            SubType::Len(self.bits(packet_start + 7, 15) as u16)
        } else {
            SubType::PacketCount(self.bits(packet_start + 7, 11) as u16)
        }
    }

    pub fn parse(&self, packet_start: usize) -> (AST, usize) {
        if self.is_literal(packet_start) {
            let (lit, new_start) = self.literal(packet_start);
            (AST::Literal(self.version(packet_start), lit), new_start)
        } else {
            let mut sub_packets = Vec::new();
            let spt = self.sub_packet_type(packet_start);
            let sub_packet_start = packet_start + spt.sub_packet_rel_start();
            let mut new_start = sub_packet_start;
            match spt {
                SubType::Len(len) => {
                    while new_start < sub_packet_start + len as usize {
                        let (p, next_packet_start) = self.parse(new_start);
                        new_start = next_packet_start;
                        sub_packets.push(p);
                    }
                }
                SubType::PacketCount(count) => {
                    let mut p_count = 0;
                    while p_count < count {
                        let (p, next_packet_start) = self.parse(new_start);
                        new_start = next_packet_start;
                        p_count += 1;
                        sub_packets.push(p);
                    }
                }
            }
            (
                AST::Op(
                    self.version(packet_start),
                    Op::from_type_id(self.type_id(packet_start)),
                    sub_packets,
                ),
                new_start,
            )
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    Sum,
    Prod,
    Min,
    Max,
    GT,
    LT,
    EQ,
}

impl Op {
    fn from_type_id(id: u8) -> Self {
        match id {
            0 => Self::Sum,
            1 => Self::Prod,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::GT,
            6 => Self::LT,
            7 => Self::EQ,
            _ => panic!("unknown type id"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AST {
    Literal(u8, Lit),
    Op(u8, Op, Vec<Self>),
}

impl AST {
    pub fn eval(&self) -> usize {
        match self {
            AST::Literal(_, x) => *x as usize,
            AST::Op(_, op, xs_asts) => {
                let mut xs = xs_asts.into_iter().map(AST::eval);
                match op {
                    Op::Sum => xs.sum(),
                    Op::Prod => xs.product(),
                    Op::Min => xs.min().unwrap(),
                    Op::Max => xs.max().unwrap(),
                    Op::GT => {
                        assert_eq!(xs_asts.len(), 2);
                        let (a, b) = xs.next_tuple().unwrap();
                        if a > b {
                            1
                        } else {
                            0
                        }
                    }
                    Op::LT => {
                        assert_eq!(xs_asts.len(), 2);
                        let (a, b) = xs.next_tuple().unwrap();
                        if a < b {
                            1
                        } else {
                            0
                        }
                    }
                    Op::EQ => {
                        assert_eq!(xs_asts.len(), 2);
                        let (a, b) = xs.next_tuple().unwrap();
                        if a == b {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AST::Literal(_, x) => write!(f, "{}", x),
            AST::Op(_, op, sub) => write!(
                f,
                "{:?}({})",
                op,
                sub.into_iter().map(|s| s.to_string()).join(", ")
            ),
        }
    }
}

fn part1(packet: &Input) -> Output {
    let (ast, _) = packet.parse(0);

    let mut version_num_sum = 0;
    let mut to_sum = vec![ast];
    while let Some(ast) = to_sum.pop() {
        match ast {
            AST::Literal(v, _) => {
                version_num_sum += v as usize;
            }
            AST::Op(v, _, sub) => {
                version_num_sum += v as usize;
                to_sum.extend(sub);
            }
        }
    }
    version_num_sum
}

fn part2(packet: &Input) -> Output {
    let (ast, _) = packet.parse(0);
    ast.eval()
}

#[cfg(test)]
mod test {
    use crate::{parse, part1, part2, Input, Op, Packet, SubType, AST};
    use aoc21::input::AoCInput;

    #[test]
    fn test_parse_and_bits() {
        let p = Packet::from_input("3806F45");
        assert_eq!(p.0, vec![0x38, 0x06, 0xf4, 0x50]);
        assert_eq!(p.bits(0, 1), 0b0);
        assert_eq!(p.bits(2, 1), 0b1);
        assert_eq!(p.bits(0, 6), 0b001110);
        assert_eq!(p.bits(3, 3), 0b110);
        assert_eq!(p.bits(0, 8), 0x38);
        assert_eq!(p.bits(8, 8), 0x06);
        assert_eq!(p.bits(4, 8), 0x80);
    }

    #[test]
    fn test_literal() {
        let p = Packet::from_input("D2FE28");
        assert_eq!(p.literal(0).0, 2021);
        let p = Packet::from_input("D3FFFBC");
        assert_eq!(p.literal(0).0, 65535);
    }

    #[test]
    fn test_header() {
        let p = Packet::from_input("D2FE28");
        assert_eq!(p.version(0), 6);
        assert_eq!(p.type_id(0), 4);
        assert_eq!(p.is_literal(0), true);
    }

    #[test]
    fn test_subpacket() {
        let p = Packet::from_input("38006F45291200");
        assert_eq!(p.sub_packet_type(0), SubType::Len(27));
    }

    #[test]
    fn test_parse() {
        let (ast, _) = parse::<Input>("8A004A801A8002F478").parse(0);
        assert_eq!(
            ast,
            AST::Op(
                4,
                Op::Min,
                vec![AST::Op(
                    1,
                    Op::Min,
                    vec![AST::Op(5, Op::Min, vec![AST::Literal(6, 15)])]
                )]
            )
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse("8A004A801A8002F478")), 16);
        assert_eq!(part1(&parse("620080001611562C8802118E34")), 12);
        assert_eq!(part1(&parse("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part1(&parse("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse("C200B40A82")), 3);
        assert_eq!(part2(&parse("04005AC33890")), 54);
        assert_eq!(part2(&parse("880086C3E88112")), 7);
        assert_eq!(part2(&parse("CE00C43D881120")), 9);
        assert_eq!(part2(&parse("D8005AC2A8F0")), 1);
        assert_eq!(part2(&parse("F600BC2D8F")), 0);
        assert_eq!(part2(&parse("9C005AC2F8F0")), 0);
        assert_eq!(part2(&parse("9C0141080250320F1802104A08")), 1);

        assert!(
            part2(&parse(&aoc21::input_string("day16"))) > 26137370600,
            "solution is too low"
        );
    }
}
