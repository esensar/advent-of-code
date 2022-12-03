use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Item {
    name: char,
}

impl Item {
    fn new(name: char) -> Self {
        if name.is_ascii_alphabetic() {
            Self { name }
        } else {
            panic!("Invalid name");
        }
    }

    fn value(&self) -> u32 {
        if self.name.is_ascii_lowercase() {
            self.name as u32 - 'a' as u32 + 1
        } else if self.name.is_ascii_uppercase() {
            self.name as u32 - 'A' as u32 + 27
        } else {
            panic!("Invalid name");
        }
    }
}

struct Rucksack {
    first_compartment: HashSet<Item>,
    second_compartment: HashSet<Item>,
}

impl Rucksack {
    fn new(mut items: Vec<Item>) -> Self {
        let length = items.len();
        let (first, second) = items.split_at_mut(length / 2);
        let first = first.to_vec();
        let second = second.to_vec();
        Self {
            first_compartment: first.into_iter().collect(),
            second_compartment: second.into_iter().collect(),
        }
    }

    fn parse(string: &str) -> Self {
        Self::new(string.chars().map(Item::new).collect())
    }

    fn common_item(&self) -> &Item {
        self.first_compartment
            .intersection(&self.second_compartment)
            .next()
            .unwrap()
    }

    fn all_items(&self) -> HashSet<Item> {
        self.first_compartment
            .union(&self.second_compartment)
            .map(Item::clone)
            .collect()
    }
}

fn read_rucksacks<T: std::io::BufRead>(lines: Lines<T>) -> Vec<Rucksack> {
    let mut rucksacks = Vec::new();
    for line in lines {
        let line = line.unwrap();
        rucksacks.push(Rucksack::parse(&line));
    }
    rucksacks
}

fn part1() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let rucksacks = read_rucksacks(reader.lines());
    rucksacks
        .iter()
        .map(|x| x.common_item().value())
        .sum::<u32>()
}

fn part2() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let rucksacks = read_rucksacks(reader.lines());
    let groups = rucksacks.chunks(3);
    groups
        .map(|g| {
            g.iter()
                .map(|r| r.all_items())
                .reduce(|l, r| l.intersection(&r).map(Item::clone).collect())
                .unwrap()
                .iter()
                .next()
                .unwrap()
                .value()
        })
        .sum()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
