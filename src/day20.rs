use std::collections::HashMap;
use std::collections::HashSet;
use std::f32;

#[aoc_generator(day20)]
pub fn load_map(input: &str) -> Vec<Vec<u8>> {
    let map: Vec<Vec<u8>> = input
        .lines()
        .filter(|l| l.len() > 0)
        .map(|l| l.as_bytes().to_vec())
        .collect();

    map
}

#[derive(Debug, Copy, Clone)]
enum Loci {
    Empty,
    Wall,
    Passage,
    Portal(usize),
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Pos {
    level: usize,
    x: usize,
    y: usize,
}

impl Pos {
    pub fn new(level: usize, x: usize, y: usize) -> Self {
        Self { level, x, y }
    }

    pub fn coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn same_location(&self, other: Pos) -> bool {
        (self.x == other.x) & (self.y == other.y)
    }

    pub fn shift_up(&self) -> Pos {
        if self.y > 0 {
            Pos::new(self.level, self.x, self.y - 1)
        } else {
            Pos::new(self.level, self.x, 99999)
        }
    }

    pub fn shift_left(&self) -> Pos {
        if self.x > 0 {
            Pos::new(self.level, self.x - 1, self.y)
        } else {
            Pos::new(self.level, 99999, self.y)
        }
    }

    pub fn shift_down(&self) -> Pos {
        Pos::new(self.level, self.x, self.y + 1)
    }

    pub fn shift_right(&self) -> Pos {
        Pos::new(self.level, self.x + 1, self.y)
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Portal {
    id: usize,
    positions: Vec<Pos>,
}

impl Portal {
    fn new<'a>(id: usize, x: usize, y: usize) -> Portal {
        Self {
            id,
            positions: [Pos::new(0, x, y)].to_vec(),
        }
    }
    fn add_position(&self, x: usize, y: usize) -> Portal {
        let mut positions = self.positions.to_owned();
        positions.push(Pos::new(0, x, y));

        Self {
            id: self.id,
            positions,
        }
    }
    fn name(&self) -> String {
        let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        format!(
            "{}{}",
            chars[(self.id / 100) - 65],
            chars[(self.id % 100) - 65]
        )
    }
    fn to_string(&self) -> String {
        format!(
            "{}: {:?}",
            self.name(),
            self.positions
                .iter()
                .map(|&p| (p.x, p.y))
                .collect::<Vec<(usize, usize)>>()
        )
    }
}

#[derive(Debug)]
pub struct Maze {
    map: Vec<Vec<Loci>>,
    portals: HashMap<usize, Portal>,
}

impl Maze {
    fn new(map: Vec<Vec<Loci>>) -> Self {
        let portals = get_portals(map.to_owned());
        Self {
            map: map.to_owned(),
            // portals,
            portals: portals,
        }
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn is_passage(&self, pos: Pos) -> bool {
        match self.get(pos) {
            Loci::Passage => true,
            _ => false,
        }
    }

    fn get(&self, pos: Pos) -> &Loci {
        &self.map[pos.y][pos.x]
    }

    fn get_xy(&self, x: usize, y: usize) -> &Loci {
        &self.map[y][x]
    }

    fn parse(map: Vec<Vec<u8>>) -> Self {
        let mut parsed_map: Vec<Vec<Loci>> = Vec::new();
        let width = map[0].len();
        let height = map.len();
        //find top left corner of hole
        let mut hole: Pos = Pos::new(0, 0, 0);
        let mut y = 2;
        let mut x = 2;
        loop {
            if map[y][x] == b' ' {
                hole = Pos::new(0, x, y);
                // println!("hole: {:?}", hole);
                break;
            }
            x += 1;
            if x == width - 3 {
                x = 2;
                y += 1;
                if y == height - 3 {
                    break;
                }
            }
        }
        //top portals
        {
            let mut parsed_row: Vec<Loci> = Vec::new();
            for x in 1..width - 1 {
                parsed_row.push(get_empty_or_portal(map[0][x], map[1][x]));
            }
            parsed_map.push(parsed_row);
        }
        //side portals and maze
        for y in 2..height - 2 {
            let mut parsed_row: Vec<Loci> = Vec::new();
            // left portals
            parsed_row.push(get_empty_or_portal(map[y][0], map[y][1]));
            for x in 2..width - 2 {
                if (x >= hole.x) & (x < width - hole.x) & (y == hole.y) {
                    //hole top edge
                    parsed_row.push(get_empty_or_portal(map[y][x], map[y + 1][x]));
                } else if (x == hole.x) & (y >= hole.y) & (y < height - hole.y) {
                    //hole left edge
                    parsed_row.push(get_empty_or_portal(map[y][x], map[y][x + 1]));
                } else if (x == width - hole.x - 1) & (y >= hole.y) & (y < height - hole.y) {
                    //hole right edge
                    parsed_row.push(get_empty_or_portal(map[y][x - 1], map[y][x]));
                } else if (x >= hole.x) & (x < width - hole.x) & (y == height - hole.y - 1) {
                    //hole bottom edge
                    parsed_row.push(get_empty_or_portal(map[y - 1][x], map[y][x]));
                } else {
                    //MAZE
                    parsed_row.push(get_loci(map[y][x]));
                }
            }
            // right portals
            parsed_row.push(get_empty_or_portal(map[y][width - 2], map[y][width - 1]));
            parsed_map.push(parsed_row);
        }
        //bottom portals
        {
            let mut parsed_row: Vec<Loci> = Vec::new();
            for i in 1..width - 1 {
                parsed_row.push(get_empty_or_portal(map[height - 2][i], map[height - 1][i]));
            }
            parsed_map.push(parsed_row);
        }
        Maze::new(parsed_map)
    }

    fn portal_jump(&self, portal_id: &usize, pos: Pos, level: usize) -> Pos {
        let vp = &self.portals.get(portal_id).unwrap().positions;
        let mut np = *vp.iter().filter(|&p| !p.same_location(pos)).next().unwrap();
        np.level = level;
        np
    }

    fn is_edge_portal(&self, pos: Pos) -> bool {
        (pos.x == 0) | (pos.x == self.width() - 1) | (pos.y == 0) | (pos.y == self.height() - 1)
    }

    fn get_best_route(&self) -> Vec<Pos> {
        // println!("get_best_route");
        // println!("portals: {:?}", self.portals);
        let start = self.portals.get(&6565).unwrap().positions[0];

        let mut best_route: Vec<Pos> = Vec::new();
        let mut route: Vec<(Pos, Vec<Pos>)> = Vec::new();

        let mut pos = Pos::new(0, start.x, start.y);

        loop {
            // println!("loop: {:?}", pos);

            match self.get(pos) {
                Loci::Portal(p) => {
                    if *p == 6565 {
                        // println!("start: {:?}", pos);
                    } else if *p == 9090 {
                        // println!("new best route: {:?}", route.len());
                        // println!("new best route: {:?}", route);
                        best_route = route.iter().map(|r| r.0).collect();
                    } else {
                        // println!("portal: {} at {:?}", p, pos);
                        pos = self.portal_jump(p, pos, 0);
                        // println!("jump to {:?}", pos);
                    }
                }
                _ => {}
            }
            let mut valid_directions = self.get_valid_directions_from(pos, route.to_owned());
            // println!("valid_directions: {:?}", valid_directions);

            if valid_directions.len() == 0 {
                // println!("--DEAD-END--");
            } else {
                if (valid_directions.len() > 1) {
                    let (_, remaining) = valid_directions.split_at(1);
                    // println!("split at {:?}, {:?}", pos, valid_directions);
                    // println!("remaining {:?}", remaining);
                    route.push((pos, remaining.to_vec()));
                    pos = valid_directions[0];
                } else {
                    route.push((pos, Vec::new()));
                    pos = valid_directions[0];
                }
            }

            if (valid_directions.len() == 0)
                | (route.len() >= best_route.len()) & (best_route.len() > 0)
            {
                // println!("back_track: {:?}", route);
                loop {
                    let back_step = route.pop();
                    // println!("back_step: {:?}", back_step);
                    match back_step {
                        None => {
                            let result: Vec<Pos> = best_route
                                .iter()
                                .filter(|&p| self.is_passage(*p))
                                .map(|&p| p)
                                .collect();
                            return result;
                        }
                        Some(bs) => {
                            if (bs.1).len() > 0 {
                                pos = (bs.1)[0];
                                let (_, remaining) = (bs.1).split_at(1);
                                route.push((bs.0, remaining.to_vec()));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_best_route_2(&self) -> Vec<Pos> {
        // println!("get_best_route_2");
        let start = self.portals.get(&6565).unwrap().positions[0];
        let end = self.portals.get(&9090).unwrap().positions[0];

        let mut best_route: Vec<Pos> = Vec::new();
        let mut route: Vec<(Pos, Vec<Pos>)> = Vec::new();

        let mut pos = Pos::new(0, start.x, start.y);

        loop {
            println!("loop: {:?}: {:?}", route.len(), pos);
            // if (route.len() >= 80) {
            //     return route.iter().map(|r| r.0).collect();
            // }

            match self.get(pos) {
                Loci::Portal(p) => {
                    // println!("Portal: {:?} at {:?}", portal_name(*p), pos);
                    if *p == 6565 {
                        if (pos.level == 0) {
                            // println!("start: {:?}", pos);
                        }
                    } else if *p == 9090 {
                        if (pos.level == 0) {
                            println!("new best route: {:?}", route.len());
                            println!("new best route: {:?}", route);
                            best_route = route.iter().map(|r| r.0).collect();
                        }
                    } else {
                        if self.is_edge_portal(pos) {
                            if (pos.level == 0) {
                                // println!("Portal closed on level 0");
                            } else {
                                let old_pos = pos;
                                pos = self.portal_jump(p, pos, pos.level - 1);
                                // println!(
                                //     "portal out: {:?} from {:?} to {:?}",
                                //     portal_name(*p),
                                //     old_pos,
                                //     pos
                                // );
                            }
                        } else {
                            let visited: HashSet<(usize, usize)> =
                                route.iter().map(|a| a.0.coords()).collect();
                            let new_pos = self.portal_jump(p, pos, pos.level + 1);
                            if visited.contains(&new_pos.coords()) {
                                // println!("PORTAL RECURSION!!!  {:?} at {:?}", portal_name(*p), pos);
                            } else {
                                // println!(
                                //     "portal in: {:?} from {:?} to {:?}",
                                //     portal_name(*p),
                                //     pos,
                                //     new_pos
                                // );
                                pos = new_pos;
                            }
                        }
                    }
                }
                _ => {}
            }
            let mut valid_directions = self.get_valid_directions_from(pos, route.to_owned());
            // println!("valid_directions: {:?}", valid_directions);

            if valid_directions.len() == 0 {
                // println!("--DEAD-END--");
            } else {
                if (valid_directions.len() > 1) {
                    let (_, remaining) = valid_directions.split_at(1);
                    // println!("split at {:?}, {:?}", pos, valid_directions);
                    // println!("remaining {:?}", remaining);
                    route.push((pos, remaining.to_vec()));
                    pos = valid_directions[0];
                } else {
                    route.push((pos, Vec::new()));
                    pos = valid_directions[0];
                }
            }

            if (valid_directions.len() == 0)
                | (route.len() >= best_route.len()) & (best_route.len() > 0)
            {
                // println!("back_track: {:?}", route);
                loop {
                    let back_step = route.pop();
                    // println!("back_step: {:?}", back_step);
                    match back_step {
                        None => {
                            let result: Vec<Pos> = best_route
                                .iter()
                                .filter(|&p| self.is_passage(*p))
                                .map(|&p| p)
                                .collect();
                            return result;
                        }
                        Some(bs) => {
                            if (bs.1).len() > 0 {
                                pos = (bs.1)[0];
                                let (_, remaining) = (bs.1).split_at(1);
                                route.push((bs.0, remaining.to_vec()));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn get_valid_directions_from(&self, pos: Pos, route: Vec<(Pos, Vec<Pos>)>) -> Vec<Pos> {
        let visited: HashSet<Pos> = route.iter().map(|a| a.0).collect();

        let mut v: Vec<Pos> = Vec::new();
        v.push(pos.shift_up());
        v.push(pos.shift_down());
        v.push(pos.shift_left());
        v.push(pos.shift_right());

        v.iter()
            .filter(|&p| !visited.contains(p))
            .filter(|&p| self.is_valid_path(*p))
            .map(|&p| p)
            .collect()
    }

    fn is_valid_path(&self, pos: Pos) -> bool {
        if (pos.x >= self.width()) | (pos.y >= self.height()) {
            return false;
        }
        match self.get_xy(pos.x, pos.y) {
            Loci::Empty | Loci::Wall => false,
            _ => true,
        }
    }

    fn print(&self) {
        for row in &self.map {
            println!();
            for cell in row {
                print!(
                    "{}",
                    match cell {
                        Loci::Passage => ".",
                        Loci::Wall => "#",
                        Loci::Portal(_) => "O",
                        _ => " ",
                    }
                );
            }
        }
    }

    fn print_portals(&self) {
        let mut ids: Vec<usize> = self.portals.values().map(|p| p.id).collect();
        ids.sort();
        for id in ids {
            println!("{}", self.portals.get(&id).unwrap().to_string());
        }
    }
}

fn get_portals<'a>(map: Vec<Vec<Loci>>) -> HashMap<usize, Portal> {
    let mut portals: HashMap<usize, Portal> = HashMap::new();
    let width = map[0].len();
    let height = map.len();
    for y in 0..height {
        // print!("\n{}:", y);
        for x in 0..width {
            // print!(" {:?}", map[y][x]);
            match map[y][x] {
                Loci::Portal(id) => {
                    if (portals.contains_key(&id)) {
                        portals.insert(id, portals.get(&id).unwrap().add_position(x, y));
                    } else {
                        portals.insert(id, Portal::new(id, x, y));
                    }
                }
                _ => {}
            }
        }
        // println!();
    }
    portals
}

fn get_loci(a: u8) -> Loci {
    match a {
        b'#' => Loci::Wall,
        b'.' => Loci::Passage,
        _ => Loci::Empty,
    }
}

fn get_empty_or_portal(a: u8, b: u8) -> Loci {
    match (a, b) {
        (b' ', _) => Loci::Empty,
        (_, b' ') => Loci::Empty,
        _ => Loci::Portal(a as usize * 100 + b as usize),
    }
}

#[test]
pub fn test1() {
    let map = load_map(
        "
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ",
    );

    let maze = &Maze::parse(map);

    assert_eq!(maze.height(), 17);

    let portals = &maze.portals;

    assert_eq!(portals.len(), 5);
    assert_eq!(
        portals.values().map(|v| v.positions.len()).sum::<usize>(),
        8
    );

    let mut route = maze.get_best_route();

    assert_eq!(route.len() - 1, 23);
}

#[test]
pub fn test2() {
    let map = load_map(
        "
                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ",
    );

    let maze = &Maze::parse(map);

    let mut route = maze.get_best_route();

    assert_eq!(route.len() - 1, 58);
}

#[test]
pub fn test3() {
    let map = load_map(
        "
         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ",
    );

    let maze = &Maze::parse(map);
    let mut route = maze.get_best_route_2();

    assert_eq!(route.len() - 1, 26);
}

#[test]
pub fn test4() {
    let map = load_map(
        "
             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ",
    );

    let maze = &Maze::parse(map);
    maze.print_portals();

    let mut route = maze.get_best_route_2();

    // maze.print();
    println!("{:?}", route);

    assert_ne!(route.len(), 0);
    assert_eq!(route.len() - 1, 396);
}

#[aoc(day20, part1)]
pub fn part1<'a>(map: &Vec<Vec<u8>>) -> usize {
    let maze = &Maze::parse(map.to_owned());
    let mut route = maze.get_best_route();
    route.len() - 1
}

#[aoc(day20, part2)]
pub fn part2<'a>(map: &Vec<Vec<u8>>) -> usize {
    let maze = &Maze::parse(map.to_owned());
    let mut route = maze.get_best_route_2();
    route.len() - 1
}
