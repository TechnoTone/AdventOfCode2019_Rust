fn to_digit_array(n: u32) -> [u32; 6] {
    [
        n / 100000,
        n % 100000 / 10000,
        n % 10000 / 1000,
        n % 1000 / 100,
        n % 100 / 10,
        n % 10,
    ]
}

fn is_valid1(n: u32) -> bool {
    let digits = to_digit_array(n);
    is_sorted(&digits) && has_duplicate1(&digits)
}

fn is_valid2(n: u32) -> bool {
    let digits = to_digit_array(n);
    is_sorted(&digits) && has_duplicate2(&digits)
}

fn is_sorted(data: &[u32]) -> bool {
    data.windows(2).all(|w| w[0] <= w[1])
}

fn has_duplicate1(data: &[u32]) -> bool {
    data.windows(2).any(|w| w[0] == w[1])
}

fn has_duplicate2(data: &[u32]) -> bool {
    let mut last: u32 = 0;
    let mut count: u32 = 0;
    for &n in data {
        if n == last {
            count += 1;
        } else {
            if count == 2 {
                // println!("return true");
                return true;
            }
            last = n;
            count = 1;
        }
        // println!("n={}, count={}", n, count);
    }
    // println!("count={}", count);
    // println!("return {}", count == 2);
    count == 2
}

#[aoc_generator(day4)]
pub fn get_input(_s: &str) -> Vec<u32> {
    (246515..739105).collect()
}

#[test]
pub fn test1() {
    assert!(is_valid1(111111));
    assert!(!is_valid1(223450));
    assert!(!is_valid1(123789));
}

#[test]
pub fn test2() {
    assert!(!is_valid2(123444));
    assert!(is_valid2(111122));
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<u32>) -> usize {
    input.iter().filter(|&n| is_valid1(*n)).count()
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<u32>) -> usize {
    input.iter().filter(|&n| is_valid2(*n)).count()
}
