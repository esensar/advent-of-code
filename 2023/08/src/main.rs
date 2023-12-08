use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

struct DesertMap {
    instructions: String,
    map: HashMap<String, (String, String)>,
}

impl DesertMap {
    fn parse(lines: Vec<String>) -> Self {
        let instructions = lines[0].to_string();
        let mut map = HashMap::new();
        for line in lines.iter().skip(2) {
            let (source, dest_str) = line.split_once(" = ").unwrap();
            let cleaned_up = dest_str.to_string().replace("(", "").replace(")", "");
            let (left, right) = cleaned_up.split_once(", ").unwrap();
            map.insert(source.to_string(), (left.to_string(), right.to_string()));
        }
        Self { instructions, map }
    }

    fn walk(&self, from: &str, to: &str) -> usize {
        let mut current = from.to_string();
        let mut instructions_cycle = self.instructions.chars().cycle();
        let mut steps = 0;
        while current != to {
            steps += 1;
            match instructions_cycle.next().unwrap() {
                'L' => {
                    current = self.map[&current].0.to_string();
                }
                'R' => {
                    current = self.map[&current].1.to_string();
                }
                _ => panic!("ISSUE"),
            }
        }
        steps
    }

    fn walk_parallel(&self, from_ending: &str, to_ending: &str) -> u128 {
        let start_nodes: HashSet<String> = self
            .map
            .keys()
            .filter(|k| k.ends_with(from_ending))
            .map(|k| k.to_string())
            .collect();
        let end_nodes: HashSet<String> = self
            .map
            .keys()
            .filter(|k| k.ends_with(to_ending))
            .map(|k| k.to_string())
            .collect();
        let steps: Vec<u128> = start_nodes
            .iter()
            .map(|node| {
                let mut current = node.to_string();
                let mut steps = 0;
                let mut instructions_cycle = self.instructions.chars().cycle();
                while !end_nodes.contains(&current) {
                    steps += 1;
                    match instructions_cycle.next().unwrap() {
                        'L' => {
                            current = self.map[&current].0.to_string();
                        }
                        'R' => {
                            current = self.map[&current].1.to_string();
                        }
                        _ => panic!("ISSUE"),
                    }
                }
                steps
            })
            .collect();
        steps.iter().fold(steps[0], |l, r| Self::lcm(l, *r))
    }

    fn lcm(a: u128, b: u128) -> u128 {
        a * b / Self::gcd(a, b)
    }

    fn gcd(mut a: u128, mut b: u128) -> u128 {
        let mut r: u128;
        while a % b > 0 {
            r = a % b;
            a = b;
            b = r;
        }
        b
    }
}

fn test1() -> usize {
    let instructions = "LLR".to_string();
    let mut map = HashMap::new();
    map.insert("AAA".to_string(), ("BBB".to_string(), "BBB".to_string()));
    map.insert("BBB".to_string(), ("AAA".to_string(), "ZZZ".to_string()));
    map.insert("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string()));
    DesertMap { instructions, map }.walk("AAA", "ZZZ")
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    DesertMap::parse(reader.lines().map(|l| l.unwrap()).collect()).walk("AAA", "ZZZ")
}

fn test2() -> u128 {
    let instructions = "LR".to_string();
    let mut map = HashMap::new();
    map.insert("11A".to_string(), ("11B".to_string(), "XXX".to_string()));
    map.insert("11B".to_string(), ("XXX".to_string(), "11Z".to_string()));
    map.insert("11Z".to_string(), ("11B".to_string(), "XXX".to_string()));
    map.insert("22A".to_string(), ("22B".to_string(), "XXX".to_string()));
    map.insert("22B".to_string(), ("22C".to_string(), "22C".to_string()));
    map.insert("22C".to_string(), ("22Z".to_string(), "22Z".to_string()));
    map.insert("22Z".to_string(), ("22B".to_string(), "22B".to_string()));
    map.insert("XXX".to_string(), ("XXX".to_string(), "XXX".to_string()));
    DesertMap { instructions, map }.walk_parallel("A", "Z")
}

fn part2() -> u128 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    DesertMap::parse(reader.lines().map(|l| l.unwrap()).collect()).walk_parallel("A", "Z")
}

fn main() {
    println!("Test 1 solution: {}", test1());
    println!("Problem 1 solution: {}", part1());
    println!("Test 2 solution: {}", test2());
    println!("Problem 2 solution: {}", part2());
}
