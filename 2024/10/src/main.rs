use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bounds(usize, usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl Point {
    fn neighbors(&self, bounds: &Bounds) -> HashSet<Point> {
        Direction::ALL
            .iter()
            .filter_map(|d| d.move_position(self, bounds))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const ALL: &[Direction; 4] = &[
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    fn move_position(&self, pos: &Point, bounds: &Bounds) -> Option<Point> {
        match self {
            Direction::Up => pos.1.checked_sub(1).map(|r| Point(pos.0, r)),
            Direction::Right => {
                if pos.0 + 1 < bounds.0 {
                    Some(Point(pos.0 + 1, pos.1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if pos.1 + 1 < bounds.1 {
                    Some(Point(pos.0, pos.1 + 1))
                } else {
                    None
                }
            }
            Direction::Left => pos.0.checked_sub(1).map(|l| Point(l, pos.1)),
        }
    }
}

#[derive(Debug, Clone)]
struct TrailMap {
    map: Vec<Vec<usize>>,
    bounds: Bounds,
}

impl TrailMap {
    fn parse_input(lines: &mut dyn Iterator<Item = String>) -> Self {
        let map: Vec<Vec<usize>> = lines
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        Self {
            bounds: Bounds(map.len(), map.first().unwrap().len()),
            map,
        }
    }

    fn trailheads(&self) -> Vec<Point> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(i, r)| {
                r.iter()
                    .enumerate()
                    .filter(|(_, h)| **h == 0)
                    .map(move |(j, _)| Point(i, j))
            })
            .collect()
    }

    fn is_trailhead(&self, pos: &Point) -> bool {
        pos.0 < self.bounds.0 && pos.1 < self.bounds.1 && self.map[pos.0][pos.1] == 0
    }

    fn is_trailend(&self, pos: &Point) -> bool {
        pos.0 < self.bounds.0 && pos.1 < self.bounds.1 && self.map[pos.0][pos.1] == 9
    }

    fn trailhead_score_recur(&self, current: Point) -> HashSet<Point> {
        if self.is_trailend(&current) {
            return HashSet::from([current]);
        }
        current
            .neighbors(&self.bounds)
            .iter()
            .filter(|n| self.map[n.0][n.1] == self.map[current.0][current.1] + 1)
            .map(|n| self.trailhead_score_recur(n.clone()))
            .reduce(|l, r| l.union(&r).cloned().collect())
            .unwrap_or(HashSet::default())
    }

    fn trailhead_rating_recur(&self, current: Point) -> usize {
        if self.is_trailend(&current) {
            return 1;
        }
        current
            .neighbors(&self.bounds)
            .iter()
            .filter(|n| self.map[n.0][n.1] == self.map[current.0][current.1] + 1)
            .map(|n| self.trailhead_rating_recur(n.clone()))
            .sum()
    }

    fn trailhead_score(&self, trailhead: Point) -> usize {
        if !self.is_trailhead(&trailhead) {
            return 0;
        }
        let reachable_ends = self.trailhead_score_recur(trailhead);
        reachable_ends.len()
    }

    fn trailhead_rating(&self, trailhead: Point) -> usize {
        if !self.is_trailhead(&trailhead) {
            return 0;
        }
        self.trailhead_rating_recur(trailhead)
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let map = TrailMap::parse_input(&mut lines);

    map.trailheads()
        .into_iter()
        .map(|t| map.trailhead_score(t))
        .sum()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let map = TrailMap::parse_input(&mut lines);

    map.trailheads()
        .into_iter()
        .map(|t| map.trailhead_rating(t))
        .sum()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}

#[cfg(test)]
mod tests {
    use crate::TrailMap;

    const INPUT: &str =
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";

    #[test]
    fn test_part_1_example() {
        let map = TrailMap::parse_input(&mut INPUT.lines().map(|l| l.to_string()));

        assert_eq!(
            36usize,
            map.trailheads()
                .into_iter()
                .map(|t| map.trailhead_score(t))
                .sum()
        );
    }

    #[test]
    fn test_part_2_example() {
        let map = TrailMap::parse_input(&mut INPUT.lines().map(|l| l.to_string()));

        assert_eq!(
            81usize,
            map.trailheads()
                .into_iter()
                .map(|t| map.trailhead_rating(t))
                .sum()
        );
    }
}
