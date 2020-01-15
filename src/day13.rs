use crate::computer::Computer;
use crate::computer::State;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, std::hash::Hash, Copy, Clone)]
struct Coords {
    x: i64,
    y: i64,
}

impl Coords {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub fn dup(original: &Coords) -> Self {
        Self::new(original.x, original.y)
    }
}

#[derive(Debug, Eq, PartialEq, std::hash::Hash, Copy, Clone)]
struct Tile {
    coords: Coords,
    id: TileId,
}

impl Tile {
    pub fn new(coords: Coords, id: TileId) -> Self {
        Self { coords, id }
    }
}

#[derive(Debug, Eq, PartialEq, std::hash::Hash, Copy, Clone)]
enum TileId {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl TileId {
    pub fn new(n: i64) -> Self {
        match n {
            4 => TileId::Ball,
            3 => TileId::Paddle,
            2 => TileId::Block,
            1 => TileId::Wall,
            _ => TileId::Empty,
        }
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|l| FromStr::from_str(l).unwrap())
        .collect()
}

#[aoc(day13, part1)]
pub fn part1(input: &[i64]) -> usize {
    let mut computer = Computer::new(input.to_vec());

    let mut outputs = Vec::new();
    let mut tiles: Vec<Tile> = Vec::new();

    loop {
        match computer.run() {
            State::Idle => return 0,
            State::AwaitingInput => return 0,
            State::Output(value) => outputs.push(value),
            State::Complete => {
                return tiles
                    .iter()
                    .filter(|t| match t.id {
                        TileId::Block => true,
                        _ => false,
                    })
                    .count();
            }
        }

        if outputs.len() == 3 {
            tiles.push(Tile::new(
                Coords::new(outputs[0], outputs[1]),
                TileId::new(outputs[2]),
            ));
            outputs.clear();
        }
    }
}

#[aoc(day13, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let mut mem = input.to_vec();
    mem[0] = 2;
    let mut computer = Computer::new(mem);

    let mut outputs = Vec::new();
    let mut tiles: Vec<Tile> = Vec::new();
    let mut score: i64 = 0;

    loop {
        match computer.run() {
            State::Idle => return 0,
            State::AwaitingInput => {
                let mut ball_x = 0;
                let mut paddle_x = 0;
                for tile in tiles.to_owned() {
                    match tile.id {
                        TileId::Ball => ball_x = tile.coords.x,
                        TileId::Paddle => paddle_x = tile.coords.x,
                        _ => {}
                    }
                }
                computer.add_input(ball_x.cmp(&paddle_x) as i64);
            }
            State::Output(value) => outputs.push(value),
            State::Complete => return score,
        }

        if outputs.len() == 3 {
            if outputs[0] == -1 {
                score = outputs[2];
            } else {
                tiles.push(Tile::new(
                    Coords::new(outputs[0], outputs[1]),
                    TileId::new(outputs[2]),
                ));
            }
            outputs.clear();
        }
    }
}
