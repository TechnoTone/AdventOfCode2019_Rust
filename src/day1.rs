use std::str::FromStr;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|l| FromStr::from_str(l).unwrap())
        .collect()
}

fn fuel_for(mass: &i32) -> i32 {
    return mass / 3 - 2;
}

fn real_fuel_for(mass: &i32) -> i32 {
    let mut total: i32 = 0;
    let mut fuel: i32 = fuel_for(mass);
    while fuel > 0 {
        total += fuel;
        fuel = fuel_for(&fuel);
    }
    return total;
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    return input.iter().map(fuel_for).sum();
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    return input.iter().map(real_fuel_for).sum();
}
