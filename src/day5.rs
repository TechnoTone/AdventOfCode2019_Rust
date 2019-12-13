use crate::computer::Computer;
use std::str::FromStr;

fn run_test(before: &[i32], input: i32, output: i32) {
    let mut computer = Computer::new(before.to_vec());
    computer.add_input(input);
    computer.run();
    println!("{:?}", computer);
    assert!(computer.output == output);
}

#[test]
pub fn simple_multiplication() {
    run_test(&[1002, 4, 3, 4, 33], 0, 0);
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
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|l| FromStr::from_str(l).unwrap())
        .collect::<Vec<i32>>()
}

#[aoc(day5, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let mut computer = Computer::new(input.to_vec());
    computer.add_input(1);
    computer.run();
    computer.output
}

#[aoc(day5, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let mut computer = Computer::new(input.to_vec());
    computer.add_input(5);
    computer.run();
    computer.output
}
