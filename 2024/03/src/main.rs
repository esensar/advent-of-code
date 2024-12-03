use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use regex::Regex;

#[derive(Debug)]
struct Instruction {
    op: String,
    left: u32,
    right: u32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_string();
        let mut parts = s.split("(");
        let op = parts.next().unwrap().to_string();
        if op == "mul" {
            let mut num_parts = parts.next().unwrap().split(",");
            let left = num_parts.next().unwrap().parse::<u32>().unwrap();
            let right = num_parts
                .next()
                .unwrap()
                .replace(")", "")
                .parse::<u32>()
                .unwrap();
            Ok(Self { op, left, right })
        } else {
            Ok(Self {
                op,
                left: 0,
                right: 0,
            })
        }
    }
}

impl Instruction {
    const JUST_MUL: &[&str; 1] = &["mul"];
    const FULL: &[&str; 3] = &["mul", "do", "don't"];

    fn parse_list(list: &str) -> Vec<Instruction> {
        Regex::new(r"(mul|do|don't)\((\d{1,3},\d{1,3})?\)")
            .unwrap()
            .captures_iter(list)
            .map(|c| {
                let m = c.get(0).unwrap().as_str();
                m.parse().unwrap()
            })
            .collect()
    }

    fn get_result(&self) -> u32 {
        if self.op == "mul" {
            self.left * self.right
        } else {
            0
        }
    }
}

trait InstructionSet {
    fn execute(&self, supported_instructions: &[&str]) -> u32;
}

impl InstructionSet for Vec<Instruction> {
    fn execute(&self, supported_instructions: &[&str]) -> u32 {
        self.iter()
            .fold((true, 0), |acc, i| match i.op.as_str() {
                "do" if supported_instructions.contains(&"do") => (true, acc.1),
                "don't" if supported_instructions.contains(&"don't") => (false, acc.1),
                "mul" if supported_instructions.contains(&"mul") => {
                    if acc.0 {
                        (acc.0, acc.1 + i.get_result())
                    } else {
                        (acc.0, acc.1)
                    }
                }
                _ => acc,
            })
            .1
    }
}

fn part1() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: String = reader
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .join("");
    Instruction::parse_list(&lines).execute(Instruction::JUST_MUL)
}

fn part2() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: String = reader
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .join("");
    Instruction::parse_list(&lines).execute(Instruction::FULL)
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}

#[cfg(test)]
mod tests {
    use crate::{Instruction, InstructionSet};

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            161u32,
            Instruction::parse_list(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
            )
            .execute(Instruction::JUST_MUL)
        )
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            48u32,
            Instruction::parse_list(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            )
            .execute(Instruction::FULL)
        );
    }
}
