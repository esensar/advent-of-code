use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_calibration_values(lines: &mut dyn Iterator<Item = String>) -> Vec<u32> {
    lines
        .map(|l| {
            let digits: Vec<char> = l.chars().filter(|c| c.is_digit(10)).collect();
            let first = digits.first().unwrap();
            let last = digits.last().unwrap();
            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .collect()
}

fn part1() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    read_calibration_values(&mut lines).iter().sum()
}

fn part2() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap()).map(|l| {
        l.replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
    });
    read_calibration_values(&mut lines).iter().sum()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
