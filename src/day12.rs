use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl Vector {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
    pub fn energy(self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Moon {
    pos: Vector,
    vel: Vector,
}

impl Moon {
    pub fn new(pos: Vector) -> Self {
        Self {
            pos,
            vel: Vector::new(0, 0, 0),
        }
    }
    pub fn energy(self) -> usize {
        self.pos.energy() * self.vel.energy()
    }
}

fn calc_acceleration(moons: Vec<Moon>) -> Vec<Vector> {
    let mut gs = Vec::new();
    for m1 in moons.to_owned() {
        let mut g = Vector::new(0, 0, 0);
        for m2 in moons.to_owned() {
            if (m2.pos.x > m1.pos.x) {
                g.x += 1;
            } else if (m2.pos.x < m1.pos.x) {
                g.x -= 1;
            }
            if (m2.pos.y > m1.pos.y) {
                g.y += 1;
            } else if (m2.pos.y < m1.pos.y) {
                g.y -= 1;
            }
            if (m2.pos.z > m1.pos.z) {
                g.z += 1;
            } else if (m2.pos.z < m1.pos.z) {
                g.z -= 1;
            }
        }
        gs.push(g);
    }
    gs
}

fn apply_acceleration(moons: &mut Vec<Moon>, acceleration: Vec<Vector>) {
    for i in 0..moons.len() {
        moons[i].vel.x += acceleration[i].x;
        moons[i].vel.y += acceleration[i].y;
        moons[i].vel.z += acceleration[i].z;
        moons[i].pos.x += moons[i].vel.x;
        moons[i].pos.y += moons[i].vel.y;
        moons[i].pos.z += moons[i].vel.z;
    }
}

fn init_moons(vectors: Vec<Vector>) -> Vec<Moon> {
    vectors.iter().map(|&v| Moon::new(v)).collect()
}

fn calc_energy(moons: Vec<Moon>) -> usize {
    moons.iter().map(|&m| m.energy()).sum()
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Vector> {
    input
        .replace("<", "")
        .replace(">", "")
        .lines()
        .map(parse_vector)
        .collect()
}

fn test_vectors() -> Vec<Vector> {
    input_generator(
        "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
    )
}

#[test]
pub fn test1() {
    let vector = test_vectors();

    assert_eq!(vector.len(), 4);
    assert_eq!(vector[0], Vector::new(-1, 0, 2));
    assert_eq!(vector[1], Vector::new(2, -10, -7));
    assert_eq!(vector[2], Vector::new(4, -8, 8));
    assert_eq!(vector[3], Vector::new(3, 5, -1));
}

#[test]
pub fn test2() {
    let moons = init_moons(test_vectors());
    let acceleration = calc_acceleration(moons);

    assert_eq!(acceleration.len(), 4);
    assert_eq!(acceleration[0], Vector::new(3, -1, -1));
    assert_eq!(acceleration[1], Vector::new(1, 3, 3));
    assert_eq!(acceleration[2], Vector::new(-3, 1, -3));
    assert_eq!(acceleration[3], Vector::new(-1, -3, 1));
}

#[test]
pub fn test3() {
    let moons = &mut init_moons(test_vectors());
    let acceleration = calc_acceleration(moons.to_owned());

    apply_acceleration(moons, acceleration);

    assert_eq!(moons.len(), 4);
    assert_eq!(moons[0].pos, Vector::new(2, -1, 1));
    assert_eq!(moons[1].pos, Vector::new(3, -7, -4));
    assert_eq!(moons[2].pos, Vector::new(1, -7, 5));
    assert_eq!(moons[3].pos, Vector::new(2, 2, 0));
}

#[test]
pub fn test4() {
    let moons = &mut init_moons(test_vectors());

    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));

    assert_eq!(moons[0].pos, Vector::new(5, -3, -1));
    assert_eq!(moons[1].pos, Vector::new(1, -2, 2));
    assert_eq!(moons[2].pos, Vector::new(1, -4, -1));
    assert_eq!(moons[3].pos, Vector::new(1, -4, 2));
}

#[test]
pub fn test5() {
    let moons = &mut init_moons(test_vectors());

    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));

    assert_eq!(moons[0].pos, Vector::new(2, 1, -3));
    assert_eq!(moons[1].pos, Vector::new(1, -8, 0));
    assert_eq!(moons[2].pos, Vector::new(3, -6, 1));
    assert_eq!(moons[3].pos, Vector::new(2, 0, 4));
}

#[test]
pub fn test6() {
    let moons = &mut init_moons(test_vectors());

    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    apply_acceleration(moons, calc_acceleration(moons.to_owned()));

    let energy = calc_energy(moons.to_owned());

    assert_eq!(energy, 179);
}

fn parse_vector(s: &str) -> Vector {
    let v: Vec<isize> = s
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| FromStr::from_str(x.split("=").collect::<Vec<&str>>()[1]).unwrap())
        .collect();

    Vector::new(v[0], v[1], v[2])
}

#[aoc(day12, part1)]
pub fn part1(input: &Vec<Vector>) -> usize {
    let moons = &mut init_moons(input.to_owned());

    for i in 0..1000 {
        apply_acceleration(moons, calc_acceleration(moons.to_owned()));
    }

    calc_energy(moons.to_owned())
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PatternData {
    i: usize,
    start_value: isize,
    last_matched: usize,
    first_step_counts: Vec<usize>,
    step_counts: Vec<usize>,
    i_history: Vec<usize>,
}

impl PatternData {
    pub fn new(start_value: isize) -> Self {
        Self {
            i: 0,
            start_value,
            last_matched: 0,
            first_step_counts: Vec::new(),
            step_counts: Vec::new(),
            i_history: Vec::new(),
        }
    }
    pub fn complete(self) -> bool {
        self.step_counts.len() == 10 && self.step_counts == self.first_step_counts
    }
    pub fn result(self) -> usize {
        self.i_history[0] - self.first_step_counts[0]
    }
    pub fn add(self, value: isize) -> Self {
        let mut slf = self.to_owned();
        slf.i += 1;
        if value == slf.start_value {
            let step = slf.i - slf.last_matched;
            if (!slf.to_owned().complete()) {
                // println!("add: {:?}", value);
                if slf.first_step_counts.len() < 10 {
                    slf.first_step_counts.push(step);
                } else {
                    slf.step_counts.push(step);
                    slf.i_history.push(slf.i);
                    if slf.step_counts.len() > 10 {
                        slf.step_counts.remove(0);
                        slf.i_history.remove(0);
                    }
                }
                // println!("{:?}", slf);
            }
            slf.last_matched = slf.i;
        }
        slf
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct MoonPatternData {
    x: PatternData,
    y: PatternData,
    z: PatternData,
}

impl MoonPatternData {
    pub fn new(start_location: Vector) -> Self {
        let x = PatternData::new(start_location.x);
        let y = PatternData::new(start_location.y);
        let z = PatternData::new(start_location.z);
        Self { x, y, z }
    }
    pub fn complete(self) -> bool {
        self.x.complete() && self.y.complete() && self.z.complete()
    }
    pub fn result(self) -> (usize, usize, usize) {
        (self.x.result(), self.y.result(), self.z.result())
    }
    pub fn add(self, value: Vector) -> Self {
        let mut slf = self;
        // println!("add: {:?}", value);
        slf.x = slf.x.add(value.x);
        slf.y = slf.y.add(value.y);
        slf.z = slf.z.add(value.z);

        slf
    }
}

fn is_divisible_by(n: usize, factor: usize) -> bool {
    let x: usize = (n / factor);
    x * factor == n
}

#[aoc(day12, part2)]
pub fn part2(input: &Vec<Vector>) -> usize {
    let moons = &mut init_moons(input.to_owned());

    let mut i: usize = 0;
    let mut i_: usize = 0;
    // println!("Initial state : {:?}", moons[0]);

    let mut pattern = MoonPatternData::new(input[0]);

    loop {
        i += 1;
        apply_acceleration(moons, calc_acceleration(moons.to_owned()));

        pattern = pattern.to_owned().add(moons[0].pos);

        if pattern.to_owned().complete() {
            let mut result = pattern.to_owned().result();

            let mut n = 2;
            let mut divisions = Vec::new();
            while n < result.0 && n < result.1 && n < result.1 {
                let mut divided = false;
                if is_divisible_by(result.0, n) {
                    result.0 = result.0 / n;
                    divided = true;
                }
                if is_divisible_by(result.1, n) {
                    result.1 = result.1 / n;
                    divided = true;
                }
                if is_divisible_by(result.2, n) {
                    result.2 = result.2 / n;
                    divided = true;
                }

                if divided {
                    divisions.push(n);
                } else {
                    n += 1;
                }
            }

            divisions.push(result.0);
            divisions.push(result.1);
            divisions.push(result.2);

            let lcm: usize = divisions.iter().fold(1, |acc, x| acc * x);

            return lcm;
        }

        if i > 9999999 {
            println!("ABORT!!");
            println!("{:?}", pattern);
            return 0;
        }
    }
}
