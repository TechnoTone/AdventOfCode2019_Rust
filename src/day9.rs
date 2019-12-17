use crate::computer::Computer;
use crate::computer::State;
use std::str::FromStr;

#[test]
pub fn test1() {
    let program = [
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ]
    .to_vec();
    let mut computer = Computer::new(program.to_owned());
    let mut i = 0;
    loop {
        let state = computer.run();
        match state {
            State::Complete => break,
            State::Output(output) => assert!(output == program[i]),
            _ => assert!(false, "No output!"),
        }
        i += 1;
    }
}

fn run_test(before: &[i64], output: i64) {
    let mut computer = Computer::new(before.to_owned());
    computer.add_input(123);
    let result = computer.run();
    match result {
        State::Output(out) => assert!(out == output),
        _ => assert!(false, "No output!"),
    }
}

#[test]
pub fn test2() {
    let program = &[1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    run_test(program, 1219070632396864);
}

#[test]
pub fn test3() {
    let program = &[104, 1125899906842624, 99];
    run_test(program, 1125899906842624);
}

#[test]
pub fn test4() {
    let program = &[109, -1, 4, 1, 99];
    run_test(program, -1);
}

#[test]
pub fn test5() {
    let program = &[109, -1, 104, 1, 99];
    run_test(program, 1);
}

#[test]
pub fn test6() {
    let program = &[109, -1, 204, 1, 99];
    run_test(program, 109);
}

#[test]
pub fn test7() {
    let program = &[109, 1, 9, 2, 204, -6, 99];
    run_test(program, 204);
}

#[test]
pub fn test8() {
    let program = &[109, 1, 109, 9, 204, -6, 99];
    run_test(program, 204);
}

#[test]
pub fn test9() {
    let program = &[109, 1, 209, -1, 204, -106, 99];
    run_test(program, 204);
}

#[test]
pub fn test10() {
    let program = &[109, 1, 3, 3, 204, 2, 99];
    run_test(program, 123);
}

#[test]
pub fn test11() {
    let program = &[109, 1, 203, 2, 204, 2, 99];
    run_test(program, 123);
}

#[test]
pub fn test12() {
    let program = &[109, 1, 203, 200, 204, 200, 99];
    run_test(program, 123);
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|l| FromStr::from_str(l).unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let mut computer = Computer::new(input.to_vec());
    let mut output: i64 = 1;
    computer.add_input(output);
    loop {
        let state = computer.run();
        match state {
            State::Output(out) => {
                output = out;
                println!("{}", output);
            }
            State::Complete => break,
            _ => {}
        }
    }
    output
}

#[aoc(day9, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let mut computer = Computer::new(input.to_vec());
    let mut output: i64 = 2;
    computer.add_input(output);
    loop {
        let state = computer.run();
        match state {
            State::Output(out) => {
                output = out;
                println!("{}", output);
            }
            State::Complete => break,
            _ => {}
        }
    }
    output
}
