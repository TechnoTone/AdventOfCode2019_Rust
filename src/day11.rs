use crate::computer::Computer;
use crate::computer::State;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, std::hash::Hash, Copy, Clone)]
struct Coords {
    x: isize,
    y: isize,
}

impl Coords {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
    pub fn dup(original: &Coords) -> Self {
        Self::new(original.x, original.y)
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    direction: Direction,
    coords: Coords,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            direction: Direction::Up,
            coords: Coords::new(0, 0),
            x_min: -2,
            x_max: 2,
            y_min: -2,
            y_max: 2,
        }
    }

    pub fn camera(self, panels: &HashMap<Coords, bool>) -> i64 {
        let result = match panels.get(&self.coords) {
            Some(true) => 1,
            _ => 0,
        };
        // println!("camera returned {}", result);
        result
    }

    pub fn turn(&mut self, direction: i64) {
        match (self.direction, direction) {
            (Direction::Up, 0) => self.direction = Direction::Left,
            (Direction::Up, 1) => self.direction = Direction::Right,
            (Direction::Left, 0) => self.direction = Direction::Down,
            (Direction::Left, 1) => self.direction = Direction::Up,
            (Direction::Right, 0) => self.direction = Direction::Up,
            (Direction::Right, 1) => self.direction = Direction::Down,
            (Direction::Down, 0) => self.direction = Direction::Right,
            (Direction::Down, 1) => self.direction = Direction::Left,
            _ => {}
        }

        match (self.direction) {
            Direction::Up => self.coords.y -= 1,
            Direction::Down => self.coords.y += 1,
            Direction::Left => self.coords.x -= 1,
            Direction::Right => self.coords.x += 1,
        }

        if self.coords.x > self.x_max {
            self.x_max = self.coords.x;
        }
        if self.coords.x < self.x_min {
            self.x_min = self.coords.x;
        }
        if self.coords.y > self.y_max {
            self.y_max = self.coords.y;
        }
        if self.coords.y < self.y_min {
            self.y_min = self.coords.y;
        }
    }

    pub fn run(&mut self, program: Vec<i64>, init_white: bool) -> (usize, String) {
        let mut computer = Computer::new(program);
        let panels: &mut HashMap<Coords, bool> = &mut HashMap::new();
        let mut painting = true;

        if (init_white) {
            panels.insert(self.coords, true);
        }

        println!("loop start");

        loop {
            let result = computer.run();
            // println!("{:?}", self);
            // println!("{:?}", self.current_state(panels));
            // println!("{:?}", result);
            match result {
                State::AwaitingInput => computer.add_input(self.camera(panels)),
                State::Output(output) => match painting {
                    true => {
                        panels.insert(self.coords, output == 1);
                        painting = false;
                    }
                    false => {
                        self.turn(output);
                        painting = true;
                    }
                },
                State::Complete => return (panels.len(), self.current_state(panels)),
                _ => {}
            }
        }
    }

    pub fn current_state(self, mut panels: &HashMap<Coords, bool>) -> String {
        let mut state = "".to_string();

        for y in self.y_min..self.y_max + 1 {
            for x in self.x_min..self.x_max + 1 {
                if (self.coords.x == x) & (self.coords.y == y) {
                    match self.direction {
                        Direction::Up => state.push_str("^"),
                        Direction::Right => state.push_str(">"),
                        Direction::Down => state.push_str("v"),
                        Direction::Left => state.push_str("<"),
                    }
                } else {
                    match panels.get(&Coords::new(x, y)) {
                        Some(true) => state.push_str("#"),
                        _ => state.push_str("."),
                    }
                }
            }
            state.push_str("\n");
        }

        state
    }
}

// #[test]
// pub fn test1() {
//     let program = "3,8,1005,8,350,1106,0,11,0,0,0,104,1,104,0,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,102,1,8,29,1006,0,82,1006,0,40,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,1002,8,1,57,1,102,15,10,1,1005,14,10,1006,0,33,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,102,1,8,90,1,1008,14,10,2,3,19,10,1006,0,35,1006,0,21,3,8,102,-1,8,10,1001,10,1,10,4,10,108,1,8,10,4,10,1002,8,1,125,1,1105,11,10,2,1105,9,10,1,4,1,10,2,1,4,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,0,10,4,10,101,0,8,164,1006,0,71,3,8,102,-1,8,10,101,1,10,10,4,10,1008,8,0,10,4,10,1002,8,1,189,1006,0,2,1,5,17,10,1006,0,76,1,1002,7,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,1001,8,0,224,1,3,5,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,1,8,10,4,10,101,0,8,250,1,1,20,10,1,102,13,10,2,101,18,10,3,8,1002,8,-1,10,101,1,10,10,4,10,108,0,8,10,4,10,102,1,8,284,2,105,0,10,1,105,20,10,3,8,1002,8,-1,10,101,1,10,10,4,10,1008,8,1,10,4,10,1002,8,1,315,1006,0,88,1,2,4,10,2,8,17,10,2,6,2,10,101,1,9,9,1007,9,1056,10,1005,10,15,99,109,672,104,0,104,1,21102,1,847069688728,1,21101,0,367,0,1106,0,471,21102,386577216404,1,1,21102,378,1,0,1105,1,471,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,3,10,104,0,104,1,3,10,104,0,104,0,3,10,104,0,104,1,21101,97952923867,0,1,21102,425,1,0,1106,0,471,21101,0,29033143319,1,21102,436,1,0,1105,1,471,3,10,104,0,104,0,3,10,104,0,104,0,21102,1,868410614628,1,21101,0,459,0,1105,1,471,21101,837896909672,0,1,21101,0,470,0,1105,1,471,99,109,2,22102,1,-1,1,21101,40,0,2,21102,502,1,3,21102,492,1,0,1106,0,535,109,-2,2105,1,0,0,1,0,0,1,109,2,3,10,204,-1,1001,497,498,513,4,0,1001,497,1,497,108,4,497,10,1006,10,529,1102,1,0,497,109,-2,2105,1,0,0,109,4,2101,0,-1,534,1207,-3,0,10,1006,10,552,21101,0,0,-3,22101,0,-3,1,22101,0,-2,2,21102,1,1,3,21101,571,0,0,1106,0,576,109,-4,2106,0,0,109,5,1207,-3,1,10,1006,10,599,2207,-4,-2,10,1006,10,599,21202,-4,1,-4,1105,1,667,21202,-4,1,1,21201,-3,-1,2,21202,-2,2,3,21102,1,618,0,1106,0,576,21201,1,0,-4,21101,0,1,-1,2207,-4,-2,10,1006,10,637,21102,0,1,-1,22202,-2,-1,-2,2107,0,-3,10,1006,10,659,21202,-1,1,1,21101,659,0,0,106,0,534,21202,-2,-1,-2,22201,-4,-2,-4,109,-5,2105,1,0";
//     let mut robot = Robot::new();

//     let result = robot.run(input_generator(program));
//     assert_eq!(result, ".....\n..<#.\n...#.\n.##..\n.....");
// }

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|l| FromStr::from_str(l).unwrap())
        .collect()
}

#[aoc(day11, part1)]
pub fn part1(input: &[i64]) -> usize {
    let mut robot = Robot::new();
    let result = robot.run(input.to_vec(), false);
    result.0
}

#[aoc(day11, part2)]
pub fn part2(input: &[i64]) -> String {
    let mut robot = Robot::new();
    let result = robot.run(input.to_vec(), true);
    result.1
}
