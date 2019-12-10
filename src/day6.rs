use std::collections::HashMap;

fn parse_orbit(line: &str) -> (&str, &str) {
    let mut s = line.split(')');
    (s.next().unwrap(), s.next().unwrap())
}

pub fn parse_orbits_1(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut system: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let orbit = parse_orbit(line);
        system.entry(orbit.0).or_default().push(orbit.1);
    }
    system
}

fn count_total_orbits(orbits: HashMap<&str, Vec<&str>>) -> usize {
    let mut level = 0;
    let mut count = 0;
    let mut planets = Vec::new();
    planets.push("COM");
    while planets.len() > 0 {
        let mut next: Vec<&str> = Vec::new();
        for p in planets {
            next.extend_from_slice(orbits.get(p).unwrap_or(&Vec::new()));
        }
        planets = next;
        level += 1;
        count += level * planets.len();
    }
    count
}

#[test]
pub fn test1() {
    let input = parse_orbits_1(
        "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L",
    );

    assert_eq!(count_total_orbits(input), 42);
}

fn parse_orbits_2(input: &str) -> HashMap<&str, &str> {
    let mut orbits: HashMap<&str, &str> = HashMap::new();
    for line in input.lines() {
        let orbit = parse_orbit(line);
        orbits.insert(orbit.1, orbit.0);
    }
    orbits
}

fn orbits_of<'a>(orbits: &HashMap<&str, &'a str>, object: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();
    let mut obj = &object;
    while *obj != "COM" {
        obj = orbits.get(obj).unwrap();
        result.push(obj);
    }
    result
}

fn count_transfers(orbits: &HashMap<&str, &str>, start: &str, dest: &str) -> usize {
    let start_orbits = orbits_of(orbits, start);
    let dest_orbits = orbits_of(orbits, dest);

    let mut count = 0;

    for s in start_orbits {
        count += 1;
        if dest_orbits.contains(&s) {
            return count + dest_orbits.iter().position(|&d| d == s).unwrap() - 1;
        }
    }

    count
}

#[test]
pub fn test2() {
    let input = parse_orbits_2(
        "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN",
    );
    assert_eq!(count_transfers(&input, "YOU", "SAN"), 4);
}

#[aoc(day6, part1)]
pub fn part1<'a>(input: &str) -> usize {
    count_total_orbits(parse_orbits_1(input))
}

#[aoc(day6, part2)]
pub fn part2<'a>(input: &str) -> usize {
    count_transfers(&parse_orbits_2(input), "YOU", "SAN")
}
