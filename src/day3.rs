use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Pos {
    x: i32,
    y: i32,
}

pub type Line = Vec<Pos>;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum State {
    Empty,
    Start,
    Horizontal,
    Vertical,
    Corner,
    Cross,
}

fn parse_line(input: &str, start_at: Pos) -> Line {
    let p = input.split_at(1);
    let direction = input.chars().next().unwrap();
    let mut length: u32 = FromStr::from_str(p.1).unwrap();

    let mut line = Line::new();
    let mut x = start_at.x;
    let mut y = start_at.y;

    match direction {
        'U' => {
            while length > 0 {
                y += 1;
                line.push(Pos { x, y });
                length -= 1;
            }
        }
        'D' => {
            while length > 0 {
                y -= 1;
                line.push(Pos { x, y });
                length -= 1;
            }
        }
        'L' => {
            while length > 0 {
                x -= 1;
                line.push(Pos { x, y });
                length -= 1;
            }
        }
        'R' => {
            while length > 0 {
                x += 1;
                line.push(Pos { x, y });
                length -= 1;
            }
        }
        _ => {}
    }
    line
}

fn parse_wire(input: &str) -> Line {
    let lines: Vec<&str> = input.split(",").collect();
    let mut pos = Pos { x: 0, y: 0 };
    let mut result = Line::new();

    for line in lines {
        result.extend(parse_line(line, pos));
        pos = *result.last().unwrap();
    }

    result
}

// fn add_to_circuit(state: State, circuit: &mut HashMap<Pos, State>, pos: &mut Pos) {
//     // println!("{:?}", pos);

//     match (circuit.get(&pos), state) {
//         (Some(State::Horizontal), State::Vertical) | (Some(State::Vertical), State::Horizontal) => {
//             circuit.insert(*pos, State::Cross);
//         }
//         (_, _) => {
//             circuit.insert(*pos, state);
//         }
//     }
// }

// fn add_line_to_circuit(line: &Line, circuit: &mut HashMap<Pos, State>, pos: &mut Pos) {
//     let direction = line.0;
//     let mut count = line.1;
//     match direction {
//         'U' => {
//             while count > 0 {
//                 pos.y -= 1;
//                 add_to_circuit(State::Vertical, circuit, pos);
//                 count -= 1;
//             }
//         }
//         'D' => {
//             while count > 0 {
//                 pos.y += 1;
//                 add_to_circuit(State::Vertical, circuit, pos);
//                 count -= 1;
//             }
//         }
//         'L' => {
//             while count > 0 {
//                 pos.x -= 1;
//                 add_to_circuit(State::Horizontal, circuit, pos);
//                 count -= 1;
//             }
//         }
//         'R' => {
//             while count > 0 {
//                 pos.x += 1;
//                 add_to_circuit(State::Horizontal, circuit, pos);
//                 count -= 1;
//             }
//         }
//         _ => {}
//     }
//     add_to_circuit(State::Corner, circuit, pos);
// }

// fn add_wire_to_circuit(wire: &Wire, circuit: &mut HashMap<Pos, State>, pos: &mut Pos) {
//     for line in wire {
//         add_line_to_circuit(line, circuit, pos);
//     }
// }

fn manhattan_distance(pos: Pos) -> i32 {
    // println!("{:?} {}", pos, pos.x.abs() + pos.y.abs());
    pos.x.abs() + pos.y.abs()
}

fn timings(pos: &Pos, left: Vec<Pos>, right: Vec<Pos>) -> usize {
    left.iter().position(|&p| p == *pos).unwrap() + right.iter().position(|&p| p == *pos).unwrap()
}

fn intersect(left: Vec<Pos>, right: Vec<Pos>) -> Line {
    let mut common = Vec::new();

    for item in left {
        if right.contains(&item) {
            common.push(item);
        }
    }

    common
}

#[test]
pub fn test1() {
    let test1 = parse_wire("R8,U5,L5,D3");
    let test2 = parse_wire("U7,R6,D4,L4");

    println!("{:?}", test1);
    println!("{:?}", test2);

    let intersections = intersect(test1, test2);

    println!("{:?}", intersections);

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

    println!("{:?}", test1);
    println!("{:?}", test2);

    let intersections = intersect(test1, test2);

    println!("{:?}", intersections);

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

    println!("{:?}", test1);
    println!("{:?}", test2);

    let intersections = intersect(test1, test2);

    println!("{:?}", intersections);

    let result = intersections
        .iter()
        .map(|&p| manhattan_distance(p))
        .min()
        .unwrap();

    assert!(result == 135)
}

#[aoc(day3, part1)]
pub fn part1<'a>(input: &str) -> i32 {
    let mut iter = input.lines().map(|l| parse_wire(l));
    let line1 = iter.next().unwrap().to_vec();
    let line2 = iter.next().unwrap().to_vec();

    let intersections = intersect(line1, line2);

    intersections
        .iter()
        .map(|&p| manhattan_distance(p))
        .min()
        .unwrap()
}

#[aoc(day3, part2)]
pub fn part2<'a>(input: &str) -> usize {
    let mut iter = input.lines().map(|l| parse_wire(l));
    let line1 = iter.next().unwrap();
    let line2 = iter.next().unwrap();

    let intersections = intersect(line1, line2);

    intersections
        .to_vec()
        .iter()
        .map(|p| timings(p, line1.to_vec(), line2.to_vec()))
        .min()
        .unwrap()
}
