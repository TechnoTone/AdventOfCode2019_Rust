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
                println!("Sum: {} + {} => {}", a, b, t);
                mem[t] = a + b;
                ix += 4;
            }
            2 => {
                let a = p_read(&mem, ix, 1, p_modes);
                let b = p_read(&mem, ix, 2, p_modes);
                let t = p_addr(&mem, ix, 3, p_modes);
                println!("Multiply: {} * {} => {}", a, b, t);
                mem[t] = a * b;
                ix += 4;
            }
            3 => {
                let t = p_addr(&mem, ix, 1, p_modes);
                println!("Input: {} => {}", input, t);
                mem[t] = input;
                ix += 2;
            }
            4 => {
                let t = p_addr(&mem, ix, 1, p_modes);
                println!("Output: {} = {}", output, t);
                output = mem[t];
                ix += 2;
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
pub fn computer_tests() {
    assert!(run(&[1002, 4, 3, 4, 33], 0) == ([1002, 4, 3, 4, 99].to_vec(), 0));
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
