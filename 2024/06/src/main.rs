use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_position(&self, pos: (isize, isize)) -> (isize, isize) {
        match self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0 + 1, pos.1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
        }
    }

    fn is_inverse(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Right, Direction::Left)
                | (Direction::Left, Direction::Right)
        )
    }
}

#[derive(Debug, Clone)]
enum MapSlot {
    Guard(Direction),
    Obstruction,
    Empty,
    Visited(Direction),
}

impl TryFrom<char> for MapSlot {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => MapSlot::Empty,
            '#' => MapSlot::Obstruction,
            '<' => MapSlot::Guard(Direction::Left),
            '>' => MapSlot::Guard(Direction::Right),
            '^' => MapSlot::Guard(Direction::Up),
            'v' => MapSlot::Guard(Direction::Down),
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone)]
struct GuardMap {
    map: Vec<Vec<MapSlot>>,
    width: usize,
    height: usize,
}

impl GuardMap {
    fn parse_input(lines: &mut dyn Iterator<Item = String>) -> Self {
        let map: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
        Self {
            height: map.len(),
            width: map.first().unwrap().len(),
            map: map
                .iter()
                .flat_map(|r| r.iter().map(|&c| c.try_into().unwrap()))
                .map(|c| vec![c])
                .collect(),
        }
    }

    fn simulate_guard(&mut self) -> bool {
        let position = self
            .map
            .iter()
            .position(|s| matches!(s.last().unwrap(), MapSlot::Guard(_)));
        if position.is_none() {
            return true;
        }
        let position = position.unwrap();
        let guard = &self.map[position].last().unwrap().clone();
        if let MapSlot::Guard(direction) = guard {
            self.map[position].push(MapSlot::Visited(direction.clone()));
            let y: isize = (position / self.width).try_into().unwrap();
            let x: isize = (position % self.width).try_into().unwrap();
            let (x, y) = direction.move_position((x, y));
            if (0..self.height).contains(&(y as usize)) && (0..self.width).contains(&(x as usize)) {
                let new_position = x as usize + (y as usize) * self.width;
                if self.map[new_position]
                    .iter()
                    .any(|s| matches!(s, MapSlot::Visited(prev_dir) if prev_dir == direction))
                {
                    return false;
                }
                match &self.map[new_position].last().unwrap() {
                    MapSlot::Obstruction => {
                        self.map[position].push(MapSlot::Guard(direction.turn_right()))
                    }
                    _ => self.map[new_position].push(guard.clone()),
                }
                return self.simulate_guard();
            }
        } else {
            panic!("");
        }
        true
    }

    fn find_potential_loop_placement_positions(
        &self,
        potential_positions: impl Iterator<Item = (usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for (i, j) in potential_positions {
            if !matches!(
                self.map[i * self.width + j].last().unwrap(),
                MapSlot::Guard(_)
            ) {
                let mut new_map = self.map.clone();
                new_map[i * self.width + j].push(MapSlot::Obstruction);
                let mut new_self = Self {
                    height: self.height,
                    width: self.width,
                    map: new_map,
                };
                if !new_self.simulate_guard() {
                    positions.push((i, j));
                }
            }
        }
        positions
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let mut map = GuardMap::parse_input(&mut lines);
    map.simulate_guard();

    map.map
        .iter()
        .filter(|s| matches!(s.last().unwrap(), MapSlot::Visited(_)))
        .count()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let map = GuardMap::parse_input(&mut lines);
    let mut test_map = map.clone();
    test_map.simulate_guard();

    map.find_potential_loop_placement_positions(
        test_map
            .map
            .iter()
            .enumerate()
            .filter(|(_, s)| matches!(s.last().unwrap(), MapSlot::Visited(_)))
            .map(|(i, _)| (i / map.width, i % map.width)),
    )
    .len()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}

#[cfg(test)]
mod tests {
    use crate::{GuardMap, MapSlot};

    const INPUT: &str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...\n";

    #[test]
    fn test_part_1_example() {
        let mut map = GuardMap::parse_input(&mut INPUT.lines().map(|l| l.to_string()));
        map.simulate_guard();

        assert_eq!(
            41,
            map.map
                .iter()
                .filter(|s| matches!(s.last().unwrap(), MapSlot::Visited(_)))
                .count(),
        );
    }

    #[test]
    fn test_part_2_example() {
        let map = GuardMap::parse_input(&mut INPUT.lines().map(|l| l.to_string()));
        let mut test_map = map.clone();
        test_map.simulate_guard();

        assert_eq!(
            6,
            map.find_potential_loop_placement_positions(
                test_map
                    .map
                    .iter()
                    .enumerate()
                    .filter(|(_, s)| matches!(s.last().unwrap(), MapSlot::Visited(_)))
                    .map(|(i, _)| (i / map.width, i % map.width)),
            )
            .len()
        );
    }
}
