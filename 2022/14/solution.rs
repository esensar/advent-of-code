use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

enum MapTile {
    Rock,
    Sand,
}

struct CaveMap {
    map: HashMap<(u32, u32), MapTile>,
    floor: Option<u32>,
}

impl CaveMap {
    fn parse(lines: Lines<BufReader<File>>) -> Self {
        let mut map = HashMap::<(u32, u32), MapTile>::new();
        for line in lines.flatten() {
            let trace_line = line
                .split("->")
                .map(|p| {
                    let parts = p
                        .trim()
                        .split(',')
                        .map(|x| x.parse::<u32>().unwrap())
                        .take(2)
                        .collect::<Vec<u32>>();
                    (parts[0], parts[1])
                })
                .collect::<Vec<(u32, u32)>>();
            for window in trace_line.windows(2) {
                match window {
                    [(lx, ly), (rx, ry)] => {
                        for i in *lx.min(rx)..=*lx.max(rx) {
                            for j in *ly.min(ry)..=*ly.max(ry) {
                                map.insert((i, j), MapTile::Rock);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        CaveMap { map, floor: None }
    }

    fn is_empty(&self, pos: (u32, u32)) -> bool {
        if !matches!(self.map.get(&pos), None) {
            return false;
        }
        match self.floor {
            Some(x) => x != pos.1,
            None => true,
        }
    }

    fn simulate_sand_pouring(&mut self, from: (u32, u32)) {
        'outer: loop {
            let mut sand_location = from;
            loop {
                if self.floor.is_none() {
                    if !self.map.iter().any(|((_, y), _)| *y > sand_location.1) {
                        break 'outer;
                    }
                }
                if self.is_empty((sand_location.0, sand_location.1 + 1)) {
                    sand_location = (sand_location.0, sand_location.1 + 1);
                    continue;
                }
                if self.is_empty((sand_location.0 - 1, sand_location.1 + 1)) {
                    sand_location = (sand_location.0 - 1, sand_location.1 + 1);
                    continue;
                }
                if self.is_empty((sand_location.0 + 1, sand_location.1 + 1)) {
                    sand_location = (sand_location.0 + 1, sand_location.1 + 1);
                    continue;
                }
                break;
            }
            if self.map.get(&sand_location).is_some() {
                break;
            }
            self.map.insert(sand_location, MapTile::Sand);
        }
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut map = CaveMap::parse(lines);
    map.simulate_sand_pouring((500, 0));
    map.map
        .iter()
        .filter(|(_k, v)| matches!(v, MapTile::Sand))
        .count()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut map = CaveMap::parse(lines);
    map.floor = map.map.iter().map(|((_x, y), _)| *y).max().map(|y| y + 2);
    map.simulate_sand_pouring((500, 0));
    map.map
        .iter()
        .filter(|(_k, v)| matches!(v, MapTile::Sand))
        .count()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
