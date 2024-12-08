use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn apply(&self, l: usize, r: usize) -> usize {
        match self {
            Operator::Add => l + r,
            Operator::Mul => l * r,
            Operator::Concat => format!("{}{}", l, r).parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Equation {
    test_value: usize,
    numbers: Vec<usize>,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (test_value_str, numbers_str) = s.split_once(": ").ok_or(())?;
        Ok(Self {
            test_value: test_value_str.parse().map_err(|_| ())?,
            numbers: numbers_str
                .split(" ")
                .map(|s| s.parse::<usize>().map_err(|_| ()))
                .collect::<Result<Vec<_>, ()>>()?,
        })
    }
}

impl Equation {
    const ALL_OPERATORS: &[Operator; 3] = &[Operator::Add, Operator::Mul, Operator::Concat];

    fn possibly_true(&self, operators: &[Operator]) -> bool {
        for i in 0..operators.len().pow((self.numbers.len() - 1) as u32) {
            if self.test_value
                == self
                    .numbers
                    .clone()
                    .into_iter()
                    .enumerate()
                    .reduce(|(_, acc), (ni, n)| {
                        (
                            ni,
                            operators[(i / operators.len().pow((ni - 1) as u32)) % operators.len()]
                                .apply(acc, n),
                        )
                    })
                    .unwrap()
                    .1
            {
                return true;
            }
        }
        false
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<Equation>().unwrap())
        .filter(|e| e.possibly_true(&Equation::ALL_OPERATORS[0..2]))
        .map(|e| e.test_value)
        .sum()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<Equation>().unwrap())
        .filter(|e| e.possibly_true(Equation::ALL_OPERATORS))
        .map(|e| e.test_value)
        .sum()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}

#[cfg(test)]
mod tests {
    use crate::Equation;

    const INPUT: &str = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            3749usize,
            INPUT
                .lines()
                .map(|l| l.to_string())
                .map(|l| l.parse::<Equation>().unwrap())
                .filter(|e| e.possibly_true(&Equation::ALL_OPERATORS[0..2]))
                .map(|e| e.test_value)
                .sum()
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            11387usize,
            INPUT
                .lines()
                .map(|l| l.to_string())
                .map(|l| l.parse::<Equation>().unwrap())
                .filter(|e| e.possibly_true(Equation::ALL_OPERATORS))
                .map(|e| e.test_value)
                .sum()
        );
    }
}
