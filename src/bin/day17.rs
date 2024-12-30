use std::cmp::max;

aoc21::main!(17);

type Input = Target;
type Output = i32;

fn parse(s: &str) -> Input {
    let (xs, ys) = s.split_once(": x=").unwrap().1.split_once(", y=").unwrap();
    let (x0, x1) = xs.split_once("..").unwrap();
    let (y0, y1) = ys.split_once("..").unwrap();
    Target {
        x_from: x0.parse().unwrap(),
        x_to: x1.parse().unwrap(),
        y_from: y0.parse().unwrap(),
        y_to: y1.parse().unwrap(),
    }
}

struct Target {
    x_from: i32,
    x_to: i32,
    y_from: i32,
    y_to: i32,
}

impl Target {
    pub fn contains(&self, pos: &V2) -> bool {
        self.x_from <= pos.x && pos.x <= self.x_to && self.y_from <= pos.y && pos.y <= self.y_to
    }
}

struct V2 {
    x: i32,
    y: i32,
}

impl V2 {
    pub fn new(x: i32, y: i32) -> Self {
        V2 { x, y }
    }
}

struct Probe {
    pos: V2,
    vel: V2,
    max_y: i32,
}

impl Probe {
    pub fn new(vx: i32, vy: i32) -> Self {
        Probe {
            pos: V2::new(0, 0),
            vel: V2::new(vx, vy),
            max_y: 0,
        }
    }

    pub fn step(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        if self.pos.y > self.max_y {
            self.max_y = self.pos.y;
        }
        if self.vel.x > 0 {
            self.vel.x -= 1;
        } else if self.vel.x < 0 {
            self.vel.x += 1;
        }
        self.vel.y -= 1;
    }

    pub fn shoot(&mut self, target: &Target) -> Option<u32> {
        let mut time = 0;
        while self.pos.x <= target.x_to && self.pos.y >= target.y_from {
            if target.contains(&self.pos) {
                return Some(time);
            }
            time += 1;
            self.step();
        }
        None
    }
}

fn part1(target: &Input) -> Output {
    let mut max_y = 0;
    for vx in 1..200 {
        for vy in 0..1000 {
            let mut p = Probe::new(vx, vy);
            if p.shoot(target).is_some() {
                max_y = max(p.max_y, max_y);
            }
        }
    }
    max_y
}

fn part2(target: &Input) -> Output {
    let mut count = 0;
    for vx in 1..target.x_to + 1 {
        for vy in target.y_from..target.x_to {
            let mut p = Probe::new(vx, vy);
            if p.shoot(target).is_some() {
                count += 1
            }
        }
    }
    count
}

#[allow(dead_code)]
const TEST_INPUT: &str = "target area: x=20..30, y=-10..-5";

aoc21::test_part1!(TEST_INPUT, 45);
aoc21::test_part2!(TEST_INPUT, 112);
