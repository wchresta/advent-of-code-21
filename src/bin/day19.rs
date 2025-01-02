extern crate nalgebra;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use lazy_static::lazy_static;
use nalgebra::{Matrix3, Vector3};

aoc21::main!(19);

type N = i16;
type Input = Vec<Block>;
type Output = usize;
type V = Vector3<N>;

fn parse(input: &str) -> Input {
    let blocks = input.split("\n\n");
    blocks.into_iter().map(Block::new).collect_vec()
}

fn part1(inp: &Input) -> Output {
    let blocks = align_blocks(inp);

    /*
    for (i, b) in blocks.iter().enumerate() {
        println!("Block {} {:?} {:?}", i, b.rotation, b.shift);
    }
    */

    let beacons: HashSet<V> =
        HashSet::from_iter(blocks.into_iter().flat_map(|b| b.vecs().collect_vec()));
    beacons.len()
}

fn part2(inp: &Input) -> Output {
    let blocks = align_blocks(inp);

    blocks
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a.shift - b.shift).abs().sum())
        .max()
        .unwrap() as usize
}

fn align_blocks(inp: &Input) -> Input {
    let mut blocks = inp.clone();
    let signs = blocks.iter().map(block_signature).collect_vec();

    let mut overlapping = HashMap::<usize, Vec<usize>>::new();
    let mut overlap_vecs = HashMap::<(usize, usize), ((V, V), (V, V))>::new();
    for (i_a, i_b) in (0..signs.len()).tuple_combinations() {
        let mut overlap_size = 0;
        for (a_sgn, vw_a) in &signs[i_a] {
            if let Some(vw_b) = signs[i_b].get(a_sgn) {
                overlap_size += 1;
                if overlap_size < 66 {
                    // 66 is (12 choose 2)
                    continue;
                }

                overlapping.entry(i_a).or_default().push(i_b);
                overlapping.entry(i_b).or_default().push(i_a);
                overlap_vecs.insert((i_a, i_b), (vw_a.clone(), vw_b.clone()));
                overlap_vecs.insert((i_b, i_a), (vw_b.clone(), vw_a.clone()));
                break;
            }
        }
    }

    let mut stack = Vec::from([0]); // Start anywhere
    let mut transformed = HashSet::from([0]);
    while let Some(i) = stack.pop() {
        let a = blocks[i].clone();
        for j in &overlapping[&i] {
            if transformed.contains(&j) {
                continue;
            }

            // Find correct rotation for b:
            let b = &mut blocks[*j];
            let ((v_a, w_a), (v_b, w_b)) = overlap_vecs[&(i, *j)];
            let b_rot = find_rotation(a.rotation * (v_a - w_a), v_b - w_b).unwrap();
            b.rotation = b_rot;
            transformed.insert(*j);
            stack.push(*j);

            // We have:
            // v_real = R_a * v_a + S_a
            // v_real = R_b * v_b + S_b

            // We want to find S_b
            // S_b = R_a * v_a + S_a - R_b * v_b

            let shift = a.to_global(&v_a) - b_rot * v_b;
            if a.to_global(&w_a) == b_rot * w_b + shift {
                b.shift = shift;
                continue;
            }

            let shift = a.to_global(&v_a) - b_rot * w_b;
            if a.to_global(&w_a) == b_rot * v_b + shift {
                b.shift = shift;
                continue;
            }

            panic!("Cannot find shift :(");
        }
    }

    blocks
}

fn find_rotation(a: V, b: V) -> Option<Matrix3<N>> {
    for s in SO_3.iter() {
        if s * b == a || s * b == -a {
            return Some(s.clone());
        }
    }
    None
}

type Sign = Vec<N>;
fn block_signature(b: &Block) -> HashMap<Sign, (V, V)> {
    HashMap::from_iter(b.vecs.iter().tuple_combinations().map(|(v, w)| {
        let mut adiff = (v - w).abs().iter().cloned().collect_vec();
        adiff.sort();
        (adiff, (v.clone(), w.clone()))
    }))
}

#[derive(Clone)]
struct Block {
    vecs: Vec<V>,
    rotation: Matrix3<N>,
    shift: V,
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        self.vecs
            .iter()
            .for_each(|v| writeln!(f, "  {:?}", v).unwrap());
        writeln!(f, "}}")
    }
}

impl Block {
    fn new(s: &str) -> Self {
        Block {
            vecs: s
                .lines()
                .skip(1)
                .map(|line| {
                    V::from_vec(
                        line.split(",")
                            .map(str::parse)
                            .map(Result::unwrap)
                            .collect_vec(),
                    )
                })
                .collect_vec(),
            rotation: Matrix3::identity(),
            shift: V::zeros(),
        }
    }

    fn to_global(&self, v: &V) -> V {
        self.rotation * v + self.shift
    }

    fn vecs<'a>(&'a self) -> impl Iterator<Item = V> + 'a {
        self.vecs.iter().map(|v| self.to_global(v))
    }
}

lazy_static! {
    static ref SO_3: Vec<Matrix3<N>> = vec![
        // (x,y,z)
        Matrix3::identity(),
        Matrix3::new(1, 0, 0, 0, -1, 0, 0, 0, -1),
        Matrix3::new(-1, 0, 0, 0, 1, 0, 0, 0, -1),
        Matrix3::new(-1, 0, 0, 0, -1, 0, 0, 0, 1),
        // (y,z,x)
        Matrix3::new(0, 1, 0, 0, 0, 1, 1, 0, 0),
        Matrix3::new(0, 1, 0, 0, 0, -1, -1, 0, 0),
        Matrix3::new(0, -1, 0, 0, 0, -1, 1, 0, 0),
        Matrix3::new(0, -1, 0, 0, 0, 1, -1, 0, 0),
        // (z,x,y)
        Matrix3::new(0, 0, 1, 1, 0, 0, 0, 1, 0),
        Matrix3::new(0, 0, 1, -1, 0, 0, 0, -1, 0),
        Matrix3::new(0, 0, -1, -1, 0, 0, 0, 1, 0),
        Matrix3::new(0, 0, -1, 1, 0, 0, 0, -1, 0),
        // (z,y,x)
        Matrix3::new(0, 0, 1, 0, 1, 0, -1, 0, 0),
        Matrix3::new(0, 0, 1, 0, -1, 0, 1, 0, 0),
        Matrix3::new(0, 0, -1, 0, 1, 0, 1, 0, 0),
        Matrix3::new(0, 0, -1, 0, -1, 0, -1, 0, 0),
        // (y,x,z)
        Matrix3::new(0, 1, 0, 1, 0, 0, 0, 0, -1),
        Matrix3::new(0, 1, 0, -1, 0, 0, 0, 0, 1),
        Matrix3::new(0, -1, 0, 1, 0, 0, 0, 0, 1),
        Matrix3::new(0, -1, 0, -1, 0, 0, 0, 0, -1),
        // (x,z,y)
        Matrix3::new(1, 0, 0, 0, 0, 1, 0, -1, 0),
        Matrix3::new(1, 0, 0, 0, 0, -1, 0, 1, 0),
        Matrix3::new(-1, 0, 0, 0, 0, 1, 0, 1, 0),
        Matrix3::new(-1, 0, 0, 0, 0, -1, 0, -1, 0),
    ];
}

// Tests

aoc21::test_part1!(TEST_INPUT, 79);
aoc21::test_part2!(TEST_INPUT, 3621);

#[allow(dead_code)]
const TEST_INPUT: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
