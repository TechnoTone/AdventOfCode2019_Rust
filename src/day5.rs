use crate::computer::Computer;
use crate::computer::State;
use std::str::FromStr;

fn immediate_test(before: &[i64], after: &[i64]) {
    let mut computer = Computer::new(before.to_owned());
    computer.run();
    assert!(
        computer.memory == after,
        "memory didn't reach expected final state"
    );
}

fn run_test(before: &[i64], input: i64, output: i64) {
    let mut computer = Computer::new(before.to_owned());
    computer.add_input(input);
    let result = computer.run();
    println!("{:?}", computer);
    match result {
        State::Output(out) => assert!(out == output),
        _ => assert!(false, "No output!"),
    }
}

#[test]
pub fn param_mode_test() {
    immediate_test(&[1002, 4, 3, 4, 33], &[1002, 4, 3, 4, 99]);
}

#[test]
pub fn test2() {
    let program = &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    run_test(program, 8, 1);
    run_test(program, 7, 0);
}

#[test]
pub fn test3() {
    let program = &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
    run_test(program, 8, 0);
    run_test(program, 7, 1);
}

#[test]
pub fn test4() {
    let program = &[3, 3, 1108, -1, 8, 3, 4, 3, 99];
    run_test(program, 8, 1);
    run_test(program, 7, 0);
}

#[test]
pub fn test5() {
    let program = &[3, 3, 1107, -1, 8, 3, 4, 3, 99];
    run_test(program, 8, 0);
    run_test(program, 7, 1);
}

#[test]
pub fn test6() {
    let program = &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
    run_test(program, 0, 0);
    run_test(program, 7, 1);
}

#[test]
pub fn test7() {
    let program = &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
    run_test(program, 0, 0);
    run_test(program, 7, 1);
}

#[test]
pub fn test8() {
    let program = &[
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
    ];
    run_test(program, 7, 999);
    run_test(program, 8, 1000);
    run_test(program, 9, 1001);
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|l| FromStr::from_str(l).unwrap())
        .collect::<Vec<i64>>()
}

#[aoc(day5, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let mut computer = Computer::new(input.to_owned());
    computer.add_input(1);
    match computer.run() {
        State::Output(output) => output,
        _ => 0,
    }
}

#[aoc(day5, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let mut computer = Computer::new(input.to_owned());
    computer.add_input(5);
    match computer.run() {
        State::Output(output) => output,
        _ => 0,
    }
}
