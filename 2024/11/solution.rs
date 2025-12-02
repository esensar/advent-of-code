use std::{cell::RefCell, collections::HashMap, fs, rc::Rc, str::FromStr};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Stone(usize);

impl From<usize> for Stone {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl FromStr for Stone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse::<usize>().map_err(|_| ())?.into())
    }
}

impl Stone {
    fn blink(&self, count: usize, cache: Rc<RefCell<HashMap<(usize, usize), usize>>>) -> usize {
        if count == 0 {
            return 1;
        }
        if let Some(result) = (*cache).borrow().get(&(self.0, count)) {
            return *result;
        }
        let result = if self.0 == 0 {
            Stone(1).blink(count - 1, Rc::clone(&cache))
        } else {
            let string = self.0.to_string();
            if string.len() % 2 == 0 {
                string[0..(string.len() / 2)]
                    .parse::<Stone>()
                    .unwrap()
                    .blink(count - 1, Rc::clone(&cache))
                    + string[(string.len() / 2)..string.len()]
                        .parse::<Stone>()
                        .unwrap()
                        .blink(count - 1, Rc::clone(&cache))
            } else {
                Stone(self.0 * 2024).blink(count - 1, Rc::clone(&cache))
            }
        };
        (*cache).borrow_mut().insert((self.0, count), result);
        result
    }
}

#[derive(Debug, Clone)]
struct StoneArrangement {
    stones: Vec<Stone>,
}

impl FromStr for StoneArrangement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s
            .split(" ")
            .map(|n| n.parse::<Stone>().map_err(|_| ()))
            .collect::<Result<Vec<_>, ()>>()?;
        Ok(Self { stones })
    }
}

impl StoneArrangement {
    fn blink(self, count: usize) -> usize {
        self.stones
            .into_iter()
            .map(|s| s.blink(count, Default::default()))
            .sum()
    }
}

fn part1() -> usize {
    let stones = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .parse::<StoneArrangement>()
        .unwrap();
    stones.blink(25)
}

fn part2() -> usize {
    let stones = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .parse::<StoneArrangement>()
        .unwrap();
    stones.blink(75)
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}

#[cfg(test)]
mod tests {
    use crate::StoneArrangement;

    const INPUT: &str = "125 17";

    #[test]
    fn test_part_1_explanation() {
        let stones = "0 1 10 99 999".parse::<StoneArrangement>().unwrap();
        assert_eq!(7, stones.blink(1));
    }

    #[test]
    fn test_part_1_example_6_blinks() {
        let stones = INPUT.parse::<StoneArrangement>().unwrap();
        assert_eq!(22usize, stones.blink(6));
    }

    #[test]
    fn test_part_1_example() {
        let stones = INPUT.parse::<StoneArrangement>().unwrap();
        assert_eq!(55312usize, stones.blink(25));
    }
}
