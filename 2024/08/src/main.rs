use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};
#[derive(Debug, Clone)]
struct AntennaMap {
    frequencies: HashMap<char, Vec<(usize, usize)>>,
    width: usize,
    height: usize,
}

impl AntennaMap {
    fn parse_input(lines: &mut dyn Iterator<Item = String>) -> Self {
        let map: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
        let mut frequencies: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
        let height = map.len();
        let width = map.first().unwrap().len();
        for i in 0..height {
            for j in 0..width {
                if map[i][j] != '.' {
                    frequencies.entry(map[i][j]).or_default().push((i, j));
                }
            }
        }
        Self {
            height,
            width,
            frequencies,
        }
    }

    fn generate_antinode_map(&self, with_resonant_harmonics: bool) -> HashSet<(usize, usize)> {
        let mut antinode_map = HashSet::new();
        let in_bounds = |(x, y): (isize, isize)| {
            x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
        };
        let mut add_if_in_bounds = |(x, y): (isize, isize)| {
            if in_bounds((x, y)) {
                antinode_map.insert((y as usize, x as usize));
            }
        };
        for (_, positions) in self.frequencies.iter() {
            for i in 0..(positions.len() - 1) {
                for j in (i + 1)..positions.len() {
                    let (y1, x1) = positions[i];
                    let (y2, x2) = positions[j];
                    let xdiff = x1 as isize - x2 as isize;
                    let ydiff = y1 as isize - y2 as isize;
                    let mut anx1 = x1 as isize + xdiff;
                    let mut any1 = y1 as isize + ydiff;
                    let mut anx2 = x2 as isize - xdiff;
                    let mut any2 = y2 as isize - ydiff;

                    if with_resonant_harmonics {
                        add_if_in_bounds((x1 as isize, y1 as isize));
                        while in_bounds((anx1, any1)) {
                            add_if_in_bounds((anx1, any1));
                            anx1 += xdiff;
                            any1 += ydiff;
                        }
                        add_if_in_bounds((x2 as isize, y2 as isize));
                        while in_bounds((anx2, any2)) {
                            add_if_in_bounds((anx2, any2));
                            anx2 -= xdiff;
                            any2 -= ydiff;
                        }
                    } else {
                        add_if_in_bounds((anx1, any1));
                        add_if_in_bounds((anx2, any2));
                    }
                }
            }
        }
        antinode_map
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let map = AntennaMap::parse_input(&mut lines);

    map.generate_antinode_map(false).len()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let map = AntennaMap::parse_input(&mut lines);

    map.generate_antinode_map(true).len()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}

#[cfg(test)]
mod tests {
    use crate::AntennaMap;

    const INPUT: &str = "............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............";

    #[test]
    fn test_part_1_example() {
        let map = AntennaMap::parse_input(&mut INPUT.lines().map(|l| l.to_string()));

        assert_eq!(14, map.generate_antinode_map(false).len());
    }

    #[test]
    fn test_part_2_example() {
        let map = AntennaMap::parse_input(&mut INPUT.lines().map(|l| l.to_string()));

        assert_eq!(34, map.generate_antinode_map(true).len());
    }
}
