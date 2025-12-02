use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

struct ElfInventory {
    entries: Vec<u32>,
}

impl ElfInventory {
    fn add_entry(&mut self, entry: u32) {
        self.entries.push(entry)
    }

    fn get_calories_count(&self) -> u32 {
        self.entries.iter().sum()
    }
}

fn read_inventory<T: std::io::BufRead>(lines: Lines<T>) -> Vec<ElfInventory> {
    let mut inventories = Vec::new();
    inventories.push(ElfInventory {
        entries: Vec::new(),
    });
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            inventories.push(ElfInventory {
                entries: Vec::new(),
            });
        } else {
            inventories
                .last_mut()
                .unwrap()
                .add_entry(line.parse().unwrap());
        }
    }
    inventories
}

fn part1() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let inventories = read_inventory(reader.lines());
    inventories
        .iter()
        .map(|x| x.get_calories_count())
        .max()
        .unwrap()
}

fn part2() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut calories_counts: Vec<u32> = read_inventory(reader.lines())
        .iter()
        .map(|x| x.get_calories_count())
        .collect();
    calories_counts.sort();
    calories_counts.iter().rev().take(3).sum()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
