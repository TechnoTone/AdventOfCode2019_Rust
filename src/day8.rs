use std::str::from_utf8;
use std::str::FromStr;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input.as_bytes().iter().map(|&n| n - 48).collect()
}

fn count(layer: &[u8], x: u8) -> usize {
    layer.iter().filter(|&n| n == &x).count()
}

#[test]
pub fn test1() {
    let input = "123456789012";
    let converted = input_generator(input);
    assert!(converted == &[1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
}

#[aoc(day8, part1)]
pub fn part1(input: &[u8]) -> usize {
    let layer = input.chunks(150).min_by_key(|n| count(n, 0)).unwrap();
    count(layer, 1) * count(layer, 2)
}

#[aoc(day8, part2)]
pub fn part2(input: &[u8]) -> String {
    let layers: Vec<&[u8]> = input.chunks(150).collect();

    let mut output = String::new();

    for y in 0..6 {
        output += "\n";
        for x in 0..25 {
            let i = x + y * 25;
            println!("{}", i);
            if layers
                .iter()
                .map(|l| l[i as usize])
                .filter(|&n| n < 2)
                .next()
                .unwrap()
                == 0
            {
                output += " ";
            } else {
                output += "*";
            }
        }
    }

    output
}
