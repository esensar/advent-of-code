use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    iter::repeat,
};

struct MotionSimulator {
    parts: Vec<(isize, isize)>,
    visited_by_tail: HashSet<(isize, isize)>,
}

impl MotionSimulator {
    fn new(knots: usize) -> Self {
        let mut visited_by_tail = HashSet::new();
        visited_by_tail.insert((0, 0));
        Self {
            parts: repeat((0, 0)).take(knots).collect(),
            visited_by_tail,
        }
    }

    fn update_tail(&mut self) {
        for i in 1..self.parts.len() {
            let head = self.parts.get(i - 1).unwrap().clone();
            let tail = self.parts.get_mut(i).unwrap();
            if head.0.abs_diff(tail.0) >= 2 && head.1 == tail.1 {
                if head.0 > tail.0 {
                    *tail = (tail.0 + 1, tail.1)
                } else {
                    *tail = (tail.0 - 1, tail.1)
                }
            } else if head.1.abs_diff(tail.1) >= 2 && head.0 == tail.0 {
                if head.1 > tail.1 {
                    *tail = (tail.0, tail.1 + 1)
                } else {
                    *tail = (tail.0, tail.1 - 1)
                }
            } else if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
                if head.1 > tail.1 {
                    *tail = (tail.0, tail.1 + 1)
                } else {
                    *tail = (tail.0, tail.1 - 1)
                }
                if head.0 > tail.0 {
                    *tail = (tail.0 + 1, tail.1)
                } else {
                    *tail = (tail.0 - 1, tail.1)
                }
            }
        }
        self.visited_by_tail.insert(*self.parts.last().unwrap());
    }

    fn move_head(&mut self, movement: (isize, isize)) {
        let head = self.parts.first_mut().unwrap();
        *head = (head.0 + movement.0, head.1 + movement.1);
    }

    fn execute_command(&mut self, direction: &char, count: u32) {
        for _i in 0..count {
            match direction {
                'D' => self.move_head((0, 1)),
                'U' => self.move_head((0, -1)),
                'R' => self.move_head((1, 0)),
                'L' => self.move_head((-1, 0)),
                _ => panic!("Unexpected direction: {}", direction),
            }
            self.update_tail();
        }
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut simulator = MotionSimulator::new(2);

    for line in lines.flatten() {
        let parts: Vec<&str> = line.split(' ').collect();
        simulator.execute_command(&parts[0].chars().next().unwrap(), parts[1].parse().unwrap());
    }

    simulator.visited_by_tail.len()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut simulator = MotionSimulator::new(10);

    for line in lines.flatten() {
        let parts: Vec<&str> = line.split(' ').collect();
        simulator.execute_command(&parts[0].chars().next().unwrap(), parts[1].parse().unwrap());
    }

    simulator.visited_by_tail.len()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
