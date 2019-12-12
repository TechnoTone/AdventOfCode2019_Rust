use crate::computer::Computer;
use std::str::FromStr;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|l| FromStr::from_str(l).unwrap())
        .collect::<Vec<i32>>()
}

fn run_test(before: &[i32], after: &[i32]) {
    let mut computer = Computer::new(before.to_vec());
    computer.run();
    assert!(computer.memory == after);
}

#[test]
pub fn tests() {
    run_test(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
    run_test(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
    run_test(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
    run_test(
        &[1, 1, 1, 4, 99, 5, 6, 0, 99],
        &[30, 1, 1, 4, 2, 5, 6, 0, 99],
    );
    run_test(
        &[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
        &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
    );
}

#[aoc(day2, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let mut program = input.to_vec();
    program[1] = 12;
    program[2] = 2;
    let mut computer = Computer::new(program);
    computer.run();
    (computer.memory)[0]
}

#[aoc(day2, part2)]
pub fn part2(input: &[i32]) -> usize {
    let mut count: usize = 0;
    let mut result: i32;

    loop {
        for noun in 0..count {
            let mut program = input.to_vec();
            program[1] = noun as i32;
            program[2] = count as i32;
            let mut computer = Computer::new(program);
            computer.run();
            if (computer.memory)[0] == 19690720 {
                return 100 * noun + count;
            }
        }
        for verb in 0..count {
            let mut program = input.to_vec();
            program[1] = count as i32;
            program[2] = verb as i32;
            let mut computer = Computer::new(program);
            computer.run();
            if (computer.memory)[0] == 19690720 {
                return 100 * count + verb;
            }
        }
        count += 1;
    }
}
