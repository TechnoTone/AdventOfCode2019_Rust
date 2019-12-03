use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Line(char, u32);
pub type Wire = Vec<Line>;

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Pos {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum State {
    Empty,
    Start,
    Horizontal,
    Vertical,
    Corner,
    Cross,
}

fn parse_line(input: &str) -> Line {
    let p = input.split_at(1);
    Line(
        input.chars().next().unwrap(),
        FromStr::from_str(p.1).unwrap(),
    )
}

fn parse_wire(input: &str) -> Wire {
    input.split(",").map(parse_line).collect()
}

fn add_to_circuit(state: State, circuit: &mut HashMap<Pos, State>, pos: &mut Pos) {
    // println!("{:?}", pos);

    match (circuit.get(&pos), state) {
        (Some(State::Horizontal), State::Vertical) | (Some(State::Vertical), State::Horizontal) => {
            circuit.insert(*pos, State::Cross);
        }
        (_, _) => {
            circuit.insert(*pos, state);
        }
    }
}

fn add_line_to_circuit(line: &Line, circuit: &mut HashMap<Pos, State>, pos: &mut Pos) {
    let direction = line.0;
    let mut count = line.1;
    match direction {
        'U' => {
            while count > 0 {
                pos.y -= 1;
                add_to_circuit(State::Vertical, circuit, pos);
                count -= 1;
            }
        }
        'D' => {
            while count > 0 {
                pos.y += 1;
                add_to_circuit(State::Vertical, circuit, pos);
                count -= 1;
            }
        }
        'L' => {
            while count > 0 {
                pos.x -= 1;
                add_to_circuit(State::Horizontal, circuit, pos);
                count -= 1;
            }
        }
        'R' => {
            while count > 0 {
                pos.x += 1;
                add_to_circuit(State::Horizontal, circuit, pos);
                count -= 1;
            }
        }
        _ => {}
    }
    add_to_circuit(State::Corner, circuit, pos);
}

fn add_wire_to_circuit(wire: &Wire, circuit: &mut HashMap<Pos, State>, pos: &mut Pos) {
    for line in wire {
        add_line_to_circuit(line, circuit, pos);
    }
}

fn manhattan_distance(pos: Pos) -> i32 {
    println!("{:?} {}", pos, pos.x.abs() + pos.y.abs());
    pos.x.abs() + pos.y.abs()
}

fn print_circuit(circuit: &mut HashMap<Pos, State>) {}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Wire> {
    input.lines().map(parse_wire).collect()
}

#[aoc(day3, part1)]
pub fn part1<'a>(wires: &'a Vec<Wire>) -> i32 {
    let test = &mut Vec::new();
    test.push(parse_wire("R8,U5,L5,D3"));
    test.push(parse_wire("U7,R6,D4,L4"));

    let circuit: &mut HashMap<Pos, State> = &mut HashMap::new();
    let pos = &mut Pos { x: 0, y: 0 };

    add_to_circuit(State::Start, circuit, pos);

    for wire in test {
        add_wire_to_circuit(wire, circuit, pos);
    }

    print_circuit(circuit);

    circuit
        .iter()
        .filter(|&(_, &v)| v == State::Cross)
        .map(|(&k, _)| manhattan_distance(k))
        .min()
        .unwrap()
}

// #[aoc(day3, part1)]
// pub fn part1<'a>(wires: &'a Vec<Wire>) -> i32 {
//     let circuit: &mut HashMap<Pos, State> = &mut HashMap::new();
//     let pos = &mut Pos { x: 0, y: 0 };

//     add_to_circuit(State::Start, circuit, pos);

//     for wire in wires {
//         add_wire_to_circuit(wire, circuit, pos);
//     }

//     // println!("{:?}", circuit);

//     circuit
//         .iter()
//         .filter(|&(_, &v)| v == State::Cross)
//         .map(|(&k, _)| manhattan_distance(k))
//         .min()
//         .unwrap()
// }

// #[aoc(day3, part2)]
// pub fn part2<'a>(wires: &'a Vec<Wire>) -> u32 {
//     0
// }
