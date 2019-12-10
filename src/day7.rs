use std::str::FromStr;

fn p_addr(mem: &Vec<isize>, index: usize, offset: usize, p_modes: [bool; 4]) -> usize {
    // println!("p_addr: index:{}, offset:{}", index, offset);
    if p_modes[(offset)] {
        // println!("val={}", index + offset);
        return index + offset;
    } else {
        // println!("ref={}", mem[index + offset]);
        return mem[index + offset] as usize;
    }
}

fn p_read(mem: &Vec<isize>, index: usize, offset: usize, p_modes: [bool; 4]) -> isize {
    // println!("p_read: index:{}, offset:{}", index, offset);
    mem[p_addr(mem, index, offset, p_modes)]
}

fn read_p_modes(mem: usize) -> [bool; 4] {
    [false, mem % 10 > 0, mem / 10 % 10 > 0, mem / 100 % 10 > 0]
}

pub fn run(init: &[isize], inputs: &[isize]) -> (Vec<isize>, isize) {
    let mut mem = init.to_vec();
    let mut output: isize = 0;
    let mut ix: usize = 0;
    let mut input = inputs.iter();

    // println!("Run: {:?}", inputs);

    loop {
        // println!("ix:{}, mem:{:?}", ix, mem);
        let op = mem[ix] % 100;
        let p_modes = read_p_modes(mem[ix] as usize / 100);
        // println!("op:{}, p_modes:{:?}", op, p_modes);

        match op {
            1 => {
                let a = p_read(&mem, ix, 1, p_modes);
                let b = p_read(&mem, ix, 2, p_modes);
                let t = p_addr(&mem, ix, 3, p_modes);
                // println!("Sum: {} + {} => {}", a, b, t);
                mem[t] = a + b;
                ix += 4;
            }
            2 => {
                let a = p_read(&mem, ix, 1, p_modes);
                let b = p_read(&mem, ix, 2, p_modes);
                let t = p_addr(&mem, ix, 3, p_modes);
                // println!("Multiply: {} * {} => {}", a, b, t);
                mem[t] = a * b;
                ix += 4;
            }
            3 => {
                let t = p_addr(&mem, ix, 1, p_modes);
                // println!("Input: {} => {}", input, t);
                mem[t] = *input.next().unwrap();
                ix += 2;
            }
            4 => {
                let t = p_read(&mem, ix, 1, p_modes);
                // println!("Output: {}", t);
                output = t;
                ix += 2;
            }
            5 => {
                let a = p_read(&mem, ix, 1, p_modes);
                let b = p_read(&mem, ix, 2, p_modes);
                // println!("jump-if-true: {}, {}", a, b);
                if a != 0 {
                    ix = b as usize;
                } else {
                    ix += 3;
                }
            }
            6 => {
                let a = p_read(&mem, ix, 1, p_modes);
                let b = p_read(&mem, ix, 2, p_modes);
                // println!("jump-if-false: {}, {}", a, b);
                if a == 0 {
                    ix = b as usize;
                } else {
                    ix += 3;
                }
            }
            7 => {
                let a = p_read(&mem, ix, 1, p_modes);
                let b = p_read(&mem, ix, 2, p_modes);
                let t = p_addr(&mem, ix, 3, p_modes);
                // println!("less than: {} < {} => {}", a, b, t);
                if a < b {
                    mem[t] = 1;
                } else {
                    mem[t] = 0;
                }
                ix += 4;
            }
            8 => {
                let a = p_read(&mem, ix, 1, p_modes);
                let b = p_read(&mem, ix, 2, p_modes);
                let t = p_addr(&mem, ix, 3, p_modes);
                // println!("equals: {} == {} => {}", a, b, t);
                if a == b {
                    mem[t] = 1;
                } else {
                    mem[t] = 0;
                }
                ix += 4;
            }
            _ => {
                break;
            }
        }
    }

    // println!("{:?}", mem);
    return (mem, output);
}

fn get_permutations(v: Vec<u8>) -> Vec<Vec<u8>> {
    match v.as_slice() {
        [] | [_] => [v].to_vec(),
        [x, y] => [[*x, *y].to_vec(), [*y, *x].to_vec()].to_vec(),
        _ => {
            let mut result: Vec<Vec<u8>> = Vec::new();
            for i in v.to_vec() {
                let others: Vec<u8> = v.to_vec().into_iter().filter(|&x| x != i).collect();
                for perm in get_permutations(others) {
                    result.push([[i].to_vec(), perm].concat());
                }
            }
            result
        }
    }
}

fn run_amps(program: &[isize], phases: Vec<u8>) -> isize {
    let mut n: isize = 0;
    for i in phases {
        let prog = &program.to_vec()[..];
        n = run(prog, &[i as isize, n]).1;
        // println! {"output: {}", n};
    }
    n
}

fn max_signal(program: &[isize]) -> isize {
    let phases = [0, 1, 2, 3, 4].to_vec();
    let permutations = get_permutations(phases);

    let mut max_value = 0;
    for perm in permutations {
        let value = run_amps(program, perm);
        if value > max_value {
            max_value = value;
        }
    }
    max_value
}

#[test]
pub fn test1() {
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
    assert!(run_amps(program, phases) == 43210);
}

#[test]
pub fn test2b() {
    let phases = [4, 3, 2, 1, 0].to_vec();
    let program = &[
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
    ];
    assert!(max_signal(program) == 43210);
}

#[test]
pub fn test3() {
    let phases = [0, 1, 2, 3, 4].to_vec();
    let program = &[
        3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99,
        0, 0,
    ];
    assert!(run_amps(program, phases) == 54321);
}

#[test]
pub fn test4() {
    let phases = [1, 0, 4, 3, 2].to_vec();
    let program = &[
        3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33,
        31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
    ];
    assert!(run_amps(program, phases) == 65210);
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(",")
        .map(|l| FromStr::from_str(l).unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[isize]) -> isize {
    max_signal(input)
}

// #[aoc(day6, part2)]
// pub fn part2<'a>(input: &str) -> usize {
//     count_transfers(&parse_orbits_2(input), "YOU", "SAN")
// }
