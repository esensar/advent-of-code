use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn execute(&self, old_value: u64) -> u64 {
        match self {
            Operation::Add(v) => old_value + v,
            Operation::Mul(v) => old_value * v,
            Operation::Square => old_value * old_value,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: u64,
    items: VecDeque<u64>,
    operation: Operation,
    division_test: u64,
    if_true: u64,
    if_false: u64,
    inspections: u64,
}

impl Monkey {
    fn parse(lines: Vec<&str>) -> Self {
        let id: u64 = lines[0].split(' ').collect::<Vec<&str>>()[1]
            .split(':')
            .collect::<Vec<&str>>()[0]
            .parse()
            .unwrap();
        let items: VecDeque<u64> = lines[1].split(':').collect::<Vec<&str>>()[1]
            .split(',')
            .map(|i| i.trim().parse().unwrap())
            .collect();
        let operation_parts: Vec<&str> = lines[2].split(' ').rev().take(2).collect();
        let operation = match operation_parts[0] {
            "old" => Operation::Square,
            number => match operation_parts[1] {
                "+" => Operation::Add(number.parse().unwrap()),
                "*" => Operation::Mul(number.parse().unwrap()),
                _ => panic!("Unknown operation"),
            },
        };
        let division_test: u64 = lines[3].split(' ').last().unwrap().parse().unwrap();
        let if_true: u64 = lines[4].split(' ').last().unwrap().parse().unwrap();
        let if_false: u64 = lines[5].split(' ').last().unwrap().parse().unwrap();
        Self {
            id,
            items,
            operation,
            division_test,
            if_true,
            if_false,
            inspections: 0,
        }
    }
}

#[derive(Debug)]
struct MonkeyGroup {
    monkeys: Vec<Monkey>,
}

impl MonkeyGroup {
    fn parse(lines: Lines<BufReader<File>>) -> Self {
        Self {
            monkeys: lines
                .flatten()
                .collect::<Vec<String>>()
                .split(|l| l.len() < 3)
                .map(|l| Monkey::parse(l.iter().map(|x| x.as_str()).collect()))
                .collect(),
        }
    }

    fn run_rounds(&mut self, round_count: u64, worry_level_division: u64) {
        let mut items_to_add = HashMap::<u64, Vec<u64>>::new();
        let all_divisors_product = self
            .monkeys
            .iter()
            .map(|m| m.division_test)
            .fold(1, |l, r| l * r);
        for _round in 0..round_count {
            for monkey in &mut self.monkeys {
                if let Some(items) = items_to_add.remove(&monkey.id) {
                    for item in items {
                        monkey.items.push_back(item);
                    }
                }
                while let Some(item) = monkey.items.pop_front() {
                    monkey.inspections += 1;
                    let mut worry_level = monkey.operation.execute(item);
                    worry_level = worry_level / worry_level_division;
                    let monkey_id = if worry_level % monkey.division_test == 0 {
                        monkey.if_true
                    } else {
                        monkey.if_false
                    };
                    items_to_add
                        .entry(monkey_id)
                        .or_insert(Vec::new())
                        .push(worry_level % all_divisors_product);
                }
            }
        }
    }
}

fn part1() -> u64 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut monkey_group = MonkeyGroup::parse(lines);
    monkey_group.run_rounds(20, 3);
    let mut inspections: Vec<u64> = monkey_group.monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.iter().rev().take(2).fold(1, |l, r| l * r)
}

fn part2() -> u64 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut monkey_group = MonkeyGroup::parse(lines);
    monkey_group.run_rounds(10000, 1);
    let mut inspections: Vec<u64> = monkey_group.monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections.iter().rev().take(2).fold(1, |l, r| l * r)
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
