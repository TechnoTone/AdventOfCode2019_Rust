use std::str::FromStr;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|l| FromStr::from_str(l).unwrap())
        .collect::<Vec<usize>>()
}

#[aoc(day2, part1)]
pub fn run(input_: &[usize]) -> usize {
    let mut ix: usize = 0;

    let mut input = input_.to_vec();
    // input[1] = 12 as usize;
    // input[2] = 2 as usize;

    while ix < input.len() {
        if input[ix] == 1 {
            let t = input[ix + 3];
            input[t] = input[input[ix + 1]] + input[input[ix + 2]];
        } else if input[ix] == 2 {
            let t = input[ix + 3];
            input[t] = input[input[ix + 1]] * input[input[ix + 2]];
        } else if input[ix] == 99 {
            break;
        }
        ix += 4;
    }

    return input[0];
}

// #[aoc(day2, part2)]
// pub fn part2(input: &[i32]) -> i32 {
//     return input.iter().map(real_fuel_for).sum();
// }
