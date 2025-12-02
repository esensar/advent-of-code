use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::Chars,
};

#[derive(Debug)]
struct WordSearch {
    matrix: Vec<(usize, usize, char)>,
    width: usize,
    height: usize,
}

impl WordSearch {
    const ALL_DIRECTIONS: &[(isize, isize)] = &[
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    fn parse_input(lines: &mut dyn Iterator<Item = String>) -> Self {
        let matrix: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
        Self {
            height: matrix.len(),
            width: matrix.first().unwrap().len(),
            matrix: matrix
                .iter()
                .enumerate()
                .flat_map(|(i, l)| l.iter().enumerate().map(move |(j, c)| (i, j, *c)))
                .collect(),
        }
    }

    fn check_dir(
        &self,
        i: usize,
        j: usize,
        next: char,
        direction: (isize, isize),
    ) -> (usize, usize, bool) {
        let newi = i.checked_add_signed(direction.0);
        let newj = j.checked_add_signed(direction.1);
        if let Some(newi) = newi {
            if let Some(newj) = newj {
                return (
                    newi,
                    newj,
                    (0..self.height).contains(&newi)
                        && (0..self.width).contains(&newj)
                        && self.matrix.get(newi * self.width + newj).unwrap().2 == next,
                );
            }
        }
        (0, 0, false)
    }

    fn count_recur(
        &self,
        i: usize,
        j: usize,
        mut chars: Chars<'_>,
        direction: Option<(isize, isize)>,
    ) -> usize {
        let next = chars.next();
        match next {
            Some(next) => match direction {
                Some(dir) => {
                    if let (i, j, true) = self.check_dir(i, j, next, dir) {
                        self.count_recur(i, j, chars, direction)
                    } else {
                        0
                    }
                }
                None => Self::ALL_DIRECTIONS
                    .iter()
                    .map(|dir| {
                        if let (i, j, true) = self.check_dir(i, j, next, *dir) {
                            self.count_recur(i, j, chars.clone(), Some(*dir))
                        } else {
                            0
                        }
                    })
                    .sum(),
            },
            None => 1,
        }
    }

    fn count_occurences(&self, needle: &str) -> usize {
        let mut chars = needle.chars();
        let first = chars.next().unwrap();

        self.matrix
            .iter()
            .filter(|(_, _, c)| *c == first)
            .map(|(i, j, _)| self.count_recur(*i, *j, chars.clone(), None))
            .sum()
    }

    fn try_count_mas_dir(&self, i: usize, j: usize, direction: (isize, isize)) -> usize {
        let newi = i.checked_add_signed(-direction.0);
        let newj = j.checked_add_signed(-direction.1);
        if let Some(newi) = newi {
            if let Some(newj) = newj {
                if self.check_dir(i, j, 'M', (-direction.0, -direction.1)).2 {
                    return self.count_recur(newi, newj, "AS".chars(), Some(direction));
                }
            }
        }
        0
    }

    fn count_mas_x(&self) -> usize {
        self.matrix
            .iter()
            .filter(|(_, _, c)| *c == 'A')
            .map(|(i, j, _)| {
                let count = self.try_count_mas_dir(*i, *j, (1, 1))
                    + self.try_count_mas_dir(*i, *j, (-1, 1))
                    + self.try_count_mas_dir(*i, *j, (-1, -1))
                    + self.try_count_mas_dir(*i, *j, (1, -1));
                if count == 2 {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    WordSearch::parse_input(&mut lines).count_occurences("XMAS")
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    WordSearch::parse_input(&mut lines).count_mas_x()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}

#[cfg(test)]
mod tests {
    use crate::WordSearch;

    const INPUT: &str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

    #[test]
    fn test_part_1_example() {
        let search = WordSearch::parse_input(&mut INPUT.lines().map(|l| l.to_string()));
        assert_eq!(18, search.count_occurences("XMAS"));
    }

    #[test]
    fn test_part_2_example() {
        let search = WordSearch::parse_input(&mut INPUT.lines().map(|l| l.to_string()));
        assert_eq!(9, search.count_mas_x());
    }
}
