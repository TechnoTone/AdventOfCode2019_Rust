use std::str::FromStr;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|l| FromStr::from_str(l).unwrap())
        .collect::<Vec<usize>>()
}

fn run(input: &[usize], noun: usize, verb: usize) -> usize {
    let mut ix: usize = 0;

    let mut mem = input.to_vec();
    mem[1] = noun;
    mem[2] = verb;

    while ix < mem.len() {
        if mem[ix] == 1 {
            let t = mem[ix + 3];
            mem[t] = mem[mem[ix + 1]] + mem[mem[ix + 2]];
        } else if mem[ix] == 2 {
            let t = mem[ix + 3];
            mem[t] = mem[mem[ix + 1]] * mem[mem[ix + 2]];
        } else if mem[ix] == 99 {
            break;
        }
        ix += 4;
    }

    return mem[0];
}

#[aoc(day2, part1)]
pub fn part1(input: &[usize]) -> usize {
    run(input, 12, 2)
}

#[aoc(day2, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut count: usize = 0;
    let mut result: usize;

    loop {
        for noun in 0..count {
            result = run(input, noun, count);
            if result == 19690720 {
                return 100 * noun + count;
            }
        }
        for verb in 0..count {
            result = run(input, count, verb);
            if result == 19690720 {
                return 100 * count + verb;
            }
        }
        count += 1;
    }
}
