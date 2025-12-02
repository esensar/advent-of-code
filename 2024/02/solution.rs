use std::{
    cmp::Reverse,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct Report {
    levels: Vec<u32>,
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            levels: s.split_whitespace().map(|p| p.parse().unwrap()).collect(),
        })
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        if self.levels.is_sorted() || self.levels.is_sorted_by_key(Reverse) {
            self.levels.windows(2).all(|window| {
                let diff = window[0].abs_diff(window[1]);
                (1..=3).contains(&diff)
            })
        } else {
            false
        }
    }
}

fn read_reports(lines: &mut dyn Iterator<Item = String>) -> Vec<Report> {
    lines.map(|l| l.parse().unwrap()).collect()
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let reports = read_reports(&mut lines);
    reports.into_iter().filter(Report::is_safe).count()
}

struct ProblemDampener;

impl ProblemDampener {
    fn build_level_removed_reports(report: &Report) -> Vec<Report> {
        report
            .levels
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let mut new_levels = report.levels.clone();
                new_levels.remove(index);
                Report { levels: new_levels }
            })
            .collect()
    }

    fn is_safe(report: &Report) -> bool {
        if report.is_safe() {
            true
        } else {
            Self::build_level_removed_reports(report)
                .iter()
                .any(|r| r.is_safe())
        }
    }
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let reports = read_reports(&mut lines);
    reports.into_iter().filter(ProblemDampener::is_safe).count()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}

#[cfg(test)]
mod tests {
    use crate::{read_reports, ProblemDampener, Report};

    #[test]
    fn test_part_1_example() {
        let reports = read_reports(
            &mut [
                "7 6 4 2 1",
                "1 2 7 8 9",
                "9 7 6 2 1",
                "1 3 2 4 5",
                "8 6 4 4 1",
                "1 3 6 7 9",
            ]
            .iter()
            .map(|l| l.to_string()),
        );
        assert_eq!(2, reports.into_iter().filter(Report::is_safe).count());
    }

    #[test]
    fn test_part_2_example() {
        let reports = read_reports(
            &mut [
                "7 6 4 2 1",
                "1 2 7 8 9",
                "9 7 6 2 1",
                "1 3 2 4 5",
                "8 6 4 4 1",
                "1 3 6 7 9",
            ]
            .iter()
            .map(|l| l.to_string()),
        );
        assert_eq!(
            4,
            reports.into_iter().filter(ProblemDampener::is_safe).count()
        );
    }
}
