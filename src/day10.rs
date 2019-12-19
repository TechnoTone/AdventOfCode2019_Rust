use std::collections::HashSet;
use std::f32;

#[aoc_generator(day10)]
pub fn parse_map(input: &str) -> Vec<(i8, i8)> {
    let mut map: Vec<(i8, i8)> = Vec::new();
    let mut y = 0;
    for line in input.lines() {
        let chars = line.as_bytes();
        for x in 0..chars.len() {
            if chars[x] == b'#' {
                map.push((x as i8, y));
            }
        }
        y += 1;
    }
    map
}

// fn get_diff(a: (i8, i8), b: (i8, i8)) -> (i8, i8) {
//     (b.0 - a.0, b.1 - a.1)
// }

fn get_common_denominator(x: i8, y: i8) -> i8 {
    let max = if x.abs() > y.abs() { x.abs() } else { y.abs() };
    for n in (2..max + 1).rev() {
        // println!("   {} % {} = {}", x, n, x % n);
        // println!("   {} % {} = {}", y, n, y % n);
        if (x % n).abs() + (y % n).abs() == 0 {
            // print!("   common_denominator({},{}) = {:?}", x, y, n);
            return n;
        }
    }
    1
}

fn get_vector(diff: (i8, i8)) -> (i8, i8) {
    match (diff.0, diff.1) {
        (0, 0) => diff,
        (0, y) => (0, y / y.abs()),
        (x, 0) => (x / x.abs(), 0),
        (x, y) => {
            let c = get_common_denominator(x, y);
            // println!("get_common_denominator({},{}) = {}", x, y, c);
            ((x / c), (y / c))
        }
    }
}

fn dist(a: (i8, i8), b: (i8, i8)) -> i8 {
    (a.0 as i8 - b.0).abs() + (a.1 - b.1).abs()
}

fn dist_comparison(loc: (i8, i8), a: (i8, i8), b: (i8, i8)) -> std::cmp::Ordering {
    dist(loc, a).cmp(&dist(loc, b))
}

fn subtract(a: (i8, i8), b: (i8, i8)) -> (i8, i8) {
    ((b.0 - a.0), (b.1 - a.1))
}

fn add(a: (i8, i8), b: (i8, i8)) -> (i8, i8) {
    ((a.0 + b.0), (a.1 + b.1))
}

fn comparison(origin: (i8, i8), dest: (i8, i8)) -> ((i8, i8), i8) {
    let offset = subtract(origin, dest);
    (offset, (offset.0.abs() + offset.1.abs()))
}

fn get_angle(x: i8, y: i8) -> u16 {
    let d = (x as f32).atan2((-y as f32)).to_degrees();
    if d < 0.0 {
        ((360.0 + d) * 100.0) as u16
    } else {
        (d * 100.0) as u16
    }
}

fn comparison2(origin: (i8, i8), dest: (i8, i8)) -> ((i8, i8), i8, u16) {
    let offset = subtract(origin, dest);
    (
        offset,
        (offset.0.abs() + offset.1.abs()),
        get_angle(offset.0, offset.1),
    )
}

fn find_in_view(map: Vec<(i8, i8)>, loc: (i8, i8)) -> ((i8, i8), Vec<((i8, i8), i8)>) {
    let mut view = Vec::new();

    let mut map2: Vec<((i8, i8), i8)> = map.iter().map(|&x| comparison(loc, x)).collect();
    map2.sort_by(|&a, &b| a.1.cmp(&b.1));

    // println!("map: {:?}", map);
    // println!("map2: {:?}", map2);

    let mut vectors: HashSet<(i8, i8)> = HashSet::new();

    for (asteroid, dist) in map2 {
        if dist > 0 {
            let vector = get_vector(asteroid);
            if (!vectors.contains(&vector)) {
                // println!("Asteroid: {:?} = Vector: {:?}", asteroid, vector);
                vectors.insert(vector);
                view.push((asteroid, dist));
            }
        }
    }
    // println!("\nVectors: {:?}", vectors);
    // println!("\nview: {:?}", view);
    (loc, view)
}

fn find_best_asteroid(map: Vec<(i8, i8)>) -> ((i8, i8), Vec<((i8, i8), i8)>) {
    let mut in_view: Vec<((i8, i8), Vec<((i8, i8), i8)>)> = map
        .to_owned()
        .iter()
        .map(|&asteroid| find_in_view(map.to_owned(), asteroid))
        .collect();

    in_view.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    // println!("in_view:");
    // for v in &in_view {
    //     println!("({},{}): {:?}", (v.0).0, (v.0).1, v.1);
    // }

    // println!("{:?}", in_view[0]);

    in_view[0].to_owned()
}

fn get_targets(map: Vec<(i8, i8)>, loc: (i8, i8)) -> Vec<(i8, i8)> {
    let mut map2: Vec<((i8, i8), i8, u16)> = map.iter().map(|&a| comparison2(loc, a)).collect();
    map2.retain(|a| a.0 != (0, 0));
    map2.sort_by(|&a, &b| a.2.cmp(&b.2).then(a.1.cmp(&b.1)));

    let mut n = 0;
    let mut targets = Vec::new();

    while map2.len() > 0 {
        let mut target = map2.remove(0);
        loop {
            targets.push(add(target.0, loc));
            n += 1;
            // println!("Target {}: {},{}", n, (target.0).0 + 8, (target.0).1 + 3);
            let angle = target.2;
            let mut next_target = map2.iter().find(|&(a, b, c)| c > &angle);
            match next_target {
                None => break,
                Some(t) => target = *t,
            }
            map2.retain(|a| a.0 != target.0);
        }
    }

    targets
}

#[test]
pub fn test1() {
    let map = parse_map(
        ".#..#
.....
#####
....#
...##",
    );

    let best = find_best_asteroid(map);

    assert!(best.0 == (3, 4));
    assert!(best.1.len() == 8);
}

#[test]
pub fn test2() {
    let map = parse_map(
        "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
    );

    // let res = find_in_view(map.to_owned(), (5, 8));
    // println!("find_in_view(5,8) = {:?}", (res.0, res.1.len()));
    // println!("find_in_view(5,8) = {:?}", res);
    // assert!(false);

    let best = find_best_asteroid(map);

    // println!("{:?}", (best.0, best.1.len()));
    // println!("{:?}", best);

    assert!(best.0 == (5, 8));
    assert!(best.1.len() == 33);
}

#[test]
pub fn test3() {
    let map = parse_map(
        "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.",
    );

    let best = find_best_asteroid(map);

    assert!(best.0 == (1, 2));
    assert!(best.1.len() == 35);
}

#[test]
pub fn test4() {
    let map = parse_map(
        ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..",
    );

    let best = find_best_asteroid(map);

    assert!(best.0 == (6, 3));
    assert!(best.1.len() == 41);
}

#[test]
pub fn test5() {
    let map = parse_map(
        ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
    );

    let best = find_best_asteroid(map);

    assert!(best.0 == (11, 13));
    assert!(best.1.len() == 210);
}

#[test]
pub fn test6() {
    let map = parse_map(
        ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##",
    );

    let targets = get_targets(map, (8, 3));
    println!("{:?}", targets);

    assert!(targets.len() == 36);
    assert!(
        targets
            == [
                (8, 1),
                (9, 0),
                (9, 1),
                (10, 0),
                (9, 2),
                (11, 1),
                (12, 1),
                (11, 2),
                (15, 1),
                (12, 2),
                (13, 2),
                (14, 2),
                (15, 2),
                (12, 3),
                (16, 4),
                (15, 4),
                (10, 4),
                (4, 4),
                (2, 4),
                (2, 3),
                (0, 2),
                (1, 2),
                (0, 1),
                (1, 1),
                (5, 2),
                (1, 0),
                (5, 1),
                (6, 1),
                (6, 0),
                (7, 0),
                (8, 0),
                (10, 1),
                (14, 0),
                (16, 1),
                (13, 3),
                (14, 3)
            ]
            .to_vec()
    );
}

#[test]
pub fn test7() {
    let map = parse_map(
        ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
    );

    let targets = get_targets(map, (11, 13));
    println!("{:?}", targets.len());
    println!("{:?}", targets);

    assert!(targets.len() == 299);
    assert!(targets[0] == (11, 12));
    assert!(targets[1] == (12, 1));
    assert!(targets[2] == (12, 2));
    assert!(targets[9] == (12, 8));
    assert!(targets[19] == (16, 0));
    assert!(targets[49] == (16, 9));
    assert!(targets[99] == (10, 16));
    assert!(targets[198] == (9, 6));
    assert!(targets[199] == (8, 2));
    assert!(targets[200] == (10, 9));
    assert!(targets[298] == (11, 1));
}

#[aoc(day10, part1)]
pub fn part1<'a>(input: &Vec<(i8, i8)>) -> usize {
    let best = find_best_asteroid(input.to_owned());
    best.1.len()
}

#[aoc(day10, part2)]
pub fn part2<'a>(input: &Vec<(i8, i8)>) -> usize {
    let best = find_best_asteroid(input.to_owned());
    let targets = get_targets(input.to_owned(), best.0);
    (targets[199].0 as usize) * 100 + (targets[199].1 as usize)
}
