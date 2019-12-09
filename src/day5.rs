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

pub fn run(init: &[isize], input: isize) -> (Vec<isize>, isize) {
    let mut mem = init.to_vec();
    let mut output: isize = 0;
    let mut ix: usize = 0;

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
                mem[t] = input;
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

#[test]
pub fn test1() {
    assert!(run(&[1002, 4, 3, 4, 33], 0) == ([1002, 4, 3, 4, 99].to_vec(), 0));
}

#[test]
pub fn test2() {
    assert!(run(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8).1 == 1);
    assert!(run(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 7).1 == 0);
}

#[test]
pub fn test3() {
    assert!(run(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8).1 == 0);
    assert!(run(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 7).1 == 1);
}

#[test]
pub fn test4() {
    assert!(run(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], 8).1 == 1);
    assert!(run(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], 7).1 == 0);
}

#[test]
pub fn test5() {
    assert!(run(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], 8).1 == 0);
    assert!(run(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], 7).1 == 1);
}

#[test]
pub fn test6() {
    assert!(
        run(
            &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            0
        )
        .1 == 0
    );
    assert!(
        run(
            &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            7
        )
        .1 == 1
    );
}

#[test]
pub fn test7() {
    assert!(run(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 0).1 == 0);
    assert!(run(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 7).1 == 1);
}

#[test]
pub fn test8() {
    assert!(
        run(
            &[
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99
            ],
            7
        )
        .1 == 999
    );
    assert!(
        run(
            &[
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99
            ],
            8
        )
        .1 == 1000
    );
    assert!(
        run(
            &[
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99
            ],
            9
        )
        .1 == 1001
    );
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(",")
        .map(|l| FromStr::from_str(l).unwrap())
        .collect::<Vec<isize>>()
}

#[aoc(day5, part1)]
pub fn part1(input: &[isize]) -> isize {
    let output = run(input, 1);
    output.1
}

#[aoc(day5, part2)]
pub fn part2(input: &[isize]) -> isize {
    let output = run(input, 5);
    output.1
}
