use std::str::FromStr;

#[derive(Debug)]
pub struct Line(char, u32);
pub type Wire = Vec<Line>;

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

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Wire> {
    input.lines().map(parse_wire).collect()
}

#[aoc(day3, part1)]
pub fn part1<'a>(wires: &'a Vec<Wire>) -> usize {
    for wire in wires {
        println!("{:?}", wire);
    }
    0
}

#[aoc(day3, part2)]
pub fn part2<'a>(wires: &'a Vec<Wire>) -> usize {
    0
}
