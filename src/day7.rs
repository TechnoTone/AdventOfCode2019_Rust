use crate::computer::Computer;
use crate::computer::State;
use std::str::FromStr;

fn get_permutations(v: Vec<i64>) -> Vec<Vec<i64>> {
    match v.as_slice() {
        [] | [_] => [v].to_vec(),
        [x, y] => [[*x, *y].to_vec(), [*y, *x].to_vec()].to_vec(),
        _ => {
            let mut result: Vec<Vec<i64>> = Vec::new();
            for i in v.to_owned() {
                let others: Vec<i64> = v.to_owned().into_iter().filter(|&x| x != i).collect();
                for perm in get_permutations(others) {
                    result.push([[i].to_vec(), perm].concat());
                }
            }
            result
        }
    }
}

fn run_amps_once(program: &[i64], phases: Vec<i64>) -> i64 {
    let mut n: i64 = 0;
    for i in phases {
        let mut computer = Computer::new(program.to_owned());
        computer.add_input(i);
        computer.add_input(n);
        match computer.run() {
            State::Output(output) => n = output,
            _ => {}
        }
    }
    n
}

fn max_signal_single_run(program: &[i64]) -> i64 {
    let phases = [0, 1, 2, 3, 4].to_vec();
    let permutations = get_permutations(phases);

    let mut max_value = 0;
    for perm in permutations {
        let value = run_amps_once(program, perm);
        if value > max_value {
            max_value = value;
        }
    }
    max_value
}

fn run_amps_feedback_loop(program: &[i64], phases: Vec<i64>) -> i64 {
    let amp = || Computer::new(program.to_owned());
    let mut amps = Vec::new();

    for i in 0..5 {
        amps.push(Computer::new(program.to_owned()));
        amps[i].add_input(phases[i]);
        amps[i].run();
    }

    let mut i: usize = 0;
    let mut v: i64 = 0;
    let mut complete = false;

    loop {
        for i in 0..5 {
            // println!("Amp: {}: {}", i, v);
            amps[i].add_input(v);
            match amps[i].run() {
                State::Output(output) => v = output,
                State::Complete => complete = true,
                _ => {}
            }
        }

        // println!("Loop result: {}", v);

        if complete {
            return v;
        }
    }
}

fn max_signal_feedback_loop(program: &[i64]) -> i64 {
    let phases = [5, 6, 7, 8, 9].to_vec();
    let permutations = get_permutations(phases);

    let mut max_value = 0;
    for perm in permutations {
        let value = run_amps_feedback_loop(program, perm.to_owned());
        if value > max_value {
            max_value = value;
        }
    }
    max_value
}

#[test]
pub fn permutations() {
    let phases = [0, 1, 2, 3, 4].to_vec();
    let permutations = get_permutations(phases);
    assert_eq!(permutations.len(), 120);
}

#[test]
pub fn test2() {
    let phases = [4, 3, 2, 1, 0].to_vec();
    let program = &[
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    assert!(run_amps_once(program, phases) == 43210);
    assert!(max_signal_single_run(program) == 43210);
}

#[test]
pub fn test3() {
    let phases = [0, 1, 2, 3, 4].to_vec();
    let program = &[
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];
    assert!(run_amps_once(program, phases) == 54321);
    assert!(max_signal_single_run(program) == 54321);
}

#[test]
pub fn test4() {
    let phases = [1, 0, 4, 3, 2].to_vec();
    let program = &[
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    assert!(run_amps_once(program, phases) == 65210);
    assert!(max_signal_single_run(program) == 65210);
}

#[test]
pub fn test5() {
    let phases = [9, 8, 7, 6, 5].to_vec();
    let program = &[
        3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
        1005, 28, 6, 99, 0, 0, 5,
    ];
    assert!(run_amps_feedback_loop(program, phases) == 139629729);
    assert!(max_signal_feedback_loop(program) == 139629729);
}

#[test]
pub fn test6() {
    let phases = [9, 7, 8, 5, 6].to_vec();
    let program = &[
        3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5,
        54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53,
        1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
    ];
    assert!(run_amps_feedback_loop(program, phases) == 18216);
    assert!(max_signal_feedback_loop(program) == 18216);
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(",")
        .map(|l| FromStr::from_str(l).unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[i64]) -> i64 {
    max_signal_single_run(input)
}

#[aoc(day7, part2)]
pub fn part2(input: &[i64]) -> i64 {
    max_signal_feedback_loop(input)
}
