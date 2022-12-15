use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Lines},
};

type Position = (i64, i64);

trait ManhattanDistance {
    fn distance(&self, other: &Self) -> u64;
}

impl ManhattanDistance for Position {
    fn distance(&self, other: &Self) -> u64 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

struct BeaconMap {
    entries: HashMap<Position, Position>,
}

impl BeaconMap {
    fn parse(lines: Lines<BufReader<File>>) -> Self {
        let mut entries = HashMap::new();
        for line in lines.flatten() {
            let parts = line.split(':').collect::<Vec<&str>>();
            let sensor_parts = parts[0].split(',').collect::<Vec<&str>>();
            let beacon_parts = parts[1].split(',').collect::<Vec<&str>>();
            entries.insert(
                (
                    sensor_parts[0].split('=').last().unwrap().parse().unwrap(),
                    sensor_parts[1].split('=').last().unwrap().parse().unwrap(),
                ),
                (
                    beacon_parts[0].split('=').last().unwrap().parse().unwrap(),
                    beacon_parts[1].split('=').last().unwrap().parse().unwrap(),
                ),
            );
        }
        BeaconMap { entries }
    }

    fn is_covered(&self, pos: Position) -> bool {
        self.entries
            .iter()
            .any(|(s, b)| s.distance(&pos) <= s.distance(b))
    }

    fn get_edge_set(&self, min: Position, max: Position) -> HashSet<Position> {
        let mut set = HashSet::new();
        for (s, b) in self.entries.iter() {
            // Go out 1 more spot, to look for the first unreachable spot
            let distance = (s.distance(b) + 1) as i64;
            for x in (s.0 - distance)..=(s.0 + distance) {
                if x < min.0 || x > max.0 {
                    continue;
                }
                let offset = (x - (s.0 - distance)).min(s.0 + distance - x);
                let y1 = s.1 + offset;
                let y2 = s.1 - offset;
                if y1 < min.1 || y1 > max.1 || y2 < min.1 || y2 > max.1 {
                    continue;
                }
                set.insert((x, y1));
                set.insert((x, y2));
            }
        }
        set
    }

    fn covered_in_row(&self, row: i64) -> u64 {
        let min_x = self
            .entries
            .iter()
            .map(|(s, b)| s.0 - s.distance(b) as i64)
            .min()
            .unwrap();
        let max_x = self
            .entries
            .iter()
            .map(|(s, b)| s.0 + s.distance(b) as i64)
            .max()
            .unwrap();

        let mut count = 0 as u64;
        for x in min_x..=max_x {
            if self.is_covered((x, row))
                && !self.entries.values().any(|(bx, by)| *bx == x && *by == row)
            {
                count += 1;
            }
        }
        count
    }
}

fn part1() -> u64 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let map = BeaconMap::parse(lines);
    map.covered_in_row(2000000)
}

fn part2() -> i64 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let map = BeaconMap::parse(lines);
    let edge_set = map.get_edge_set((0, 0), (4000000, 4000000));
    for (x, y) in edge_set.iter() {
        if !map.is_covered((*x, *y)) {
            return *x * 4000000 + *y;
        }
    }
    panic!("No solution found!");
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
