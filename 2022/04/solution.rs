use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    ops::RangeInclusive,
};

struct AssignmentPair {
    left: RangeInclusive<u32>,
    right: RangeInclusive<u32>,
}

impl AssignmentPair {
    fn larger_smaller_range(&self) -> (&RangeInclusive<u32>, &RangeInclusive<u32>) {
        let left_length = self.left.end() - self.left.start();
        let right_length = self.right.end() - self.right.start();
        if left_length > right_length {
            (&self.left, &self.right)
        } else {
            (&self.right, &self.left)
        }
    }

    fn has_contained_range(&self) -> bool {
        let (larger, smaller) = self.larger_smaller_range();
        larger.start() <= smaller.start() && larger.end() >= smaller.end()
    }

    fn has_overlap(&self) -> bool {
        self.right.contains(self.left.start())
            || self.right.contains(self.left.end())
            || self.left.contains(self.right.start())
            || self.left.contains(self.right.end())
    }

    fn parse(line: &str) -> Self {
        let mut parts = line.split(',').map(AssignmentPair::parse_range);
        AssignmentPair {
            left: parts.next().unwrap(),
            right: parts.next().unwrap(),
        }
    }

    fn parse_range(range: &str) -> RangeInclusive<u32> {
        let range_parts: Vec<u32> = range
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        RangeInclusive::new(range_parts[0], range_parts[1])
    }
}

fn read_pairs<T: std::io::BufRead>(lines: Lines<T>) -> Vec<AssignmentPair> {
    let mut assignment_pairs = Vec::new();
    for line in lines {
        let line = line.unwrap();
        assignment_pairs.push(AssignmentPair::parse(&line));
    }
    assignment_pairs
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let pairs = read_pairs(reader.lines());
    pairs.iter().filter(|x| x.has_contained_range()).count()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let pairs = read_pairs(reader.lines());
    pairs.iter().filter(|x| x.has_overlap()).count()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
