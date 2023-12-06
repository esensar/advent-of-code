use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct RecordsMap {
    races: Vec<(usize, usize)>,
}

impl RecordsMap {
    fn parse(lines: Vec<String>) -> Self {
        let times_line = lines[0].clone();
        let times = times_line
            .split(" ")
            .filter(|s| !s.is_empty())
            .skip(1)
            .map(|x| x.trim().parse::<usize>().unwrap());
        let distances_line = lines[1].clone();
        let distances = distances_line
            .split(" ")
            .filter(|s| !s.is_empty())
            .skip(1)
            .map(|x| x.trim().parse::<usize>().unwrap());
        Self {
            races: times.zip(distances).collect(),
        }
    }

    fn parse_bad_kerning(lines: Vec<String>) -> Self {
        let times_line = lines[0].clone();
        let times = times_line
            .replace(" ", "")
            .replace("Time:", "")
            .trim()
            .parse::<usize>()
            .unwrap();
        let distances_line = lines[1].clone();
        let distances = distances_line
            .replace(" ", "")
            .replace("Distance:", "")
            .trim()
            .parse::<usize>()
            .unwrap();
        Self {
            races: vec![(times, distances)],
        }
    }

    fn ways_to_beat(time: usize, record: usize) -> usize {
        let result = (1..time).filter(|t| t * (time - t) > record).count();
        result
    }

    fn total_ways_to_beat(&self) -> usize {
        self.races
            .iter()
            .map(|r| Self::ways_to_beat(r.0, r.1))
            .fold(1, |acc, w| acc * w)
    }
}

fn test1() -> usize {
    RecordsMap::parse(vec![
        "Time: 7 15 30".to_string(),
        "Distance: 9 40 200".to_string(),
    ])
    .total_ways_to_beat()
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    RecordsMap::parse(reader.lines().map(|x| x.unwrap()).collect()).total_ways_to_beat()
}

fn test2() -> usize {
    RecordsMap::parse_bad_kerning(vec![
        "Time: 7 15 30".to_string(),
        "Distance: 9 40 200".to_string(),
    ])
    .total_ways_to_beat()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    RecordsMap::parse_bad_kerning(reader.lines().map(|x| x.unwrap()).collect()).total_ways_to_beat()
}

fn main() {
    println!("Test solution: {}", test1());
    println!("Problem 1 solution: {}", part1());
    println!("Test solution: {}", test2());
    println!("Problem 2 solution: {}", part2());
}
