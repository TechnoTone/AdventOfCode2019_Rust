use std::collections::HashMap;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Pos {
    x: i32,
    y: i32,
}

pub type Wire = HashMap<Pos, u32>;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum State {
    Empty,
    Start,
    Horizontal,
    Vertical,
    Corner,
    Cross,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn to_path(segment: &str) -> String {
    let d = segment.chars().next().unwrap() as u8;
    let length: usize = FromStr::from_str(segment.split_at(1).1).unwrap();
    String::from_utf8(vec![d; length]).unwrap()
}

fn to_direction(d: char) -> Direction {
    match d {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        _ => Direction::Right,
    }
}

fn parse_wire(input: &str) -> Wire {
    let path: String = input.split(",").map(to_path).collect();
    let directions: Vec<Direction> = path.chars().map(to_direction).collect();

    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    let mut result = Wire::new();

    for d in directions {
        match d {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x += 1,
            Direction::Right => x -= 1,
        }
        count += 1;
        result.insert(Pos { x, y }, count);
    }

    result
}

fn manhattan_distance(pos: Pos) -> i32 {
    pos.x.abs() + pos.y.abs()
}

fn timings(pos: &Pos, left: &Wire, right: &Wire) -> u32 {
    left.get(pos).unwrap() + right.get(pos).unwrap()
}

fn intersect(left: &Wire, right: &Wire) -> Vec<Pos> {
    let mut common = Vec::new();

    for item in left.keys() {
        if right.contains_key(&item) {
            common.push(*item);
        }
    }

    common
}

// fn in_order(wire: &Wire) -> Vec<(Pos, u32)> {
//     let mut vec: Vec<(&Pos, &u32)> = wire.iter().collect();
//     vec.sort_by(|a, b| b.1.cmp(a.1));
//     vec.iter().map(|(&k, &v)| (k, v)).collect()
// }

#[test]
pub fn test1() {
    let test1 = parse_wire("R8,U5,L5,D3");
    let test2 = parse_wire("U7,R6,D4,L4");

    // println!("{:?}", in_order(&test1));
    // println!("{:?}", in_order(&test2));

    let intersections = intersect(&test1, &test2);

    // println!("Intersections: {:?}", intersections);

    let result = intersections
        .iter()
        .map(|&p| manhattan_distance(p))
        .min()
        .unwrap();

    assert!(result == 6)
}

#[test]
pub fn test2() {
    let test1 = parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let test2 = parse_wire("U62,R66,U55,R34,D71,R55,D58,R83");

    // println!("{:?}", test1);
    // println!("{:?}", test2);

    let intersections = intersect(&test1, &test2);

    // println!("{:?}", intersections);

    let result = intersections
        .iter()
        .map(|&p| manhattan_distance(p))
        .min()
        .unwrap();

    assert!(result == 159)
}

#[test]
pub fn test3() {
    let test1 = parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let test2 = parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

    // println!("{:?}", test1);
    // println!("{:?}", test2);

    let intersections = intersect(&test1, &test2);

    // println!("{:?}", intersections);

    let result = intersections
        .iter()
        .map(|&p| manhattan_distance(p))
        .min()
        .unwrap();

    assert!(result == 135)
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> (Wire, Wire) {
    let mut lines = input.lines();
    (
        parse_wire(lines.next().unwrap()),
        parse_wire(lines.next().unwrap()),
    )
}

#[aoc(day3, part1)]
pub fn part1<'a>((w1, w2): &(Wire, Wire)) -> i32 {
    let intersections = intersect(&w1, &w2);

    intersections
        .iter()
        .map(|&p| manhattan_distance(p))
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn part2<'a>((w1, w2): &(Wire, Wire)) -> u32 {
    let intersections = intersect(&w1, &w2);

    intersections
        .to_vec()
        .iter()
        .map(|p| timings(p, &w1, &w2))
        .min()
        .unwrap()
}
