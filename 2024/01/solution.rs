use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

fn read_lists(lines: &mut dyn Iterator<Item = String>) -> (Vec<u32>, Vec<u32>) {
    lines
        .map(|l| {
            let mut parts = l.split_whitespace();
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap())
        })
        .collect()
}

fn sum_diffs(left: &mut Vec<u32>, right: &mut Vec<u32>) -> u32 {
    left.sort();
    right.sort();
    zip(left, right).map(|(l, r)| l.abs_diff(*r)).sum::<u32>()
}

fn calculate_similarity_score(left: &mut [u32], right: &mut [u32]) -> usize {
    left.iter()
        .map(|l| *l as usize * right.iter().filter(|r| *r == l).count())
        .sum()
}

fn part1() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let (mut left, mut right) = read_lists(&mut lines);
    sum_diffs(&mut left, &mut right)
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let (mut left, mut right) = read_lists(&mut lines);
    calculate_similarity_score(&mut left, &mut right)
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}

#[cfg(test)]
mod tests {
    use crate::{calculate_similarity_score, read_lists, sum_diffs};

    #[test]
    fn test_part_1_example() {
        let (mut left, mut right) = read_lists(
            &mut ["3  4", "4  3", "2  5", "1  3", "3  9", "3  3"]
                .iter()
                .map(|l| l.to_string()),
        );
        assert_eq!(11, sum_diffs(&mut left, &mut right));
    }

    #[test]
    fn test_part_2_example() {
        let (mut left, mut right) = read_lists(
            &mut ["3  4", "4  3", "2  5", "1  3", "3  9", "3  3"]
                .iter()
                .map(|l| l.to_string()),
        );
        assert_eq!(31, calculate_similarity_score(&mut left, &mut right));
    }
}
