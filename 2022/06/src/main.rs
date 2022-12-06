use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

fn position_of_distinct(count: usize) -> usize {
    let contents = read_to_string("input.txt").unwrap();
    let mut last = contents.chars().take(count - 1).collect::<VecDeque<char>>();
    let mut pos = count - 1;
    for char in contents.chars().skip(count - 1) {
        last.push_back(char);
        pos += 1;
        if last.len() > count {
            last.pop_front();
        }
        if last.clone().into_iter().collect::<HashSet<char>>().len() == count {
            break;
        }
    }
    pos
}

fn part1() -> usize {
    position_of_distinct(4)
}

fn part2() -> usize {
    position_of_distinct(14)
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
