use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Rules {
    violation_map: HashMap<usize, Vec<usize>>,
}

impl Rules {
    fn parse_input(lines: &mut dyn Iterator<Item = String>) -> Self {
        let mut violation_map: HashMap<usize, Vec<usize>> = HashMap::new();

        for line in lines.take_while(|l| !l.is_empty()) {
            let (left, right) = line.split_once("|").unwrap();
            violation_map
                .entry(right.parse().unwrap())
                .or_default()
                .push(left.parse().unwrap());
        }
        Self { violation_map }
    }

    fn check(&self, update: &Update) -> bool {
        for (index, page) in update.pages.iter().enumerate() {
            if let Some(violations) = self.violation_map.get(page) {
                if update
                    .pages
                    .iter()
                    .skip(index + 1)
                    .any(|p| violations.contains(p))
                {
                    return false;
                }
            }
        }
        true
    }

    fn sort_bad_update(&self, update: &Update) -> Update {
        let mut pages = update.pages.clone();
        pages.sort_by(|a, b| {
            if let Some(violations) = self.violation_map.get(a) {
                if violations.contains(b) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else if let Some(violations) = self.violation_map.get(b) {
                if violations.contains(a) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                Ordering::Equal
            }
        });
        Update { pages }
    }
}

#[derive(Debug)]
struct Update {
    pages: Vec<usize>,
}

impl Update {
    fn parse_input(lines: &mut dyn Iterator<Item = String>) -> Vec<Self> {
        lines
            .map(|l| Self {
                pages: l.split(",").map(|n| n.parse().unwrap()).collect(),
            })
            .collect()
    }

    fn middle_page(&self) -> usize {
        self.pages[self.pages.len() / 2]
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let rules = Rules::parse_input(&mut lines);
    let updates = Update::parse_input(&mut lines);
    updates
        .iter()
        .filter(|u| rules.check(u))
        .map(Update::middle_page)
        .sum()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let rules = Rules::parse_input(&mut lines);
    let updates = Update::parse_input(&mut lines);
    updates
        .iter()
        .filter(|u| !rules.check(u))
        .map(|u| rules.sort_bad_update(u))
        .map(|u| u.middle_page())
        .sum()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}

#[cfg(test)]
mod tests {
    use crate::{Rules, Update};

    const INPUT: &str = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|1\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";

    #[test]
    fn test_part_1_example() {
        let mut lines = INPUT.lines().map(|l| l.to_string());
        let rules = Rules::parse_input(&mut lines);
        let updates = Update::parse_input(&mut lines);
        assert_eq!(
            143usize,
            updates
                .iter()
                .filter(|u| rules.check(u))
                .map(Update::middle_page)
                .sum()
        );
    }

    #[test]
    fn test_part_2_example() {
        let mut lines = INPUT.lines().map(|l| l.to_string());
        let rules = Rules::parse_input(&mut lines);
        let updates = Update::parse_input(&mut lines);
        assert_eq!(
            123usize,
            updates
                .iter()
                .filter(|u| !rules.check(u))
                .map(|u| rules.sort_bad_update(u))
                .map(|u| u.middle_page())
                .sum()
        );
    }
}
