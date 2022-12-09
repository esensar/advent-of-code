use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

struct Forest {
    trees: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Forest {
    fn parse(lines: Lines<BufReader<File>>) -> Self {
        let mut trees = Vec::<Vec<u32>>::new();
        for line in lines.flatten() {
            trees.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
        }
        let width = trees[0].len();
        let height = trees.len();
        Self {
            trees,
            width,
            height,
        }
    }

    fn is_tree_visible(&self, x: usize, y: usize) -> bool {
        let tree = self.trees[x][y];
        let invisibe_top = self.trees.iter().take(x).map(|r| r[y]).any(|t| t >= tree);
        if !invisibe_top {
            return true;
        }
        let invisibe_bottom = self
            .trees
            .iter()
            .skip(x + 1)
            .map(|r| r[y])
            .any(|t| t >= tree);
        if !invisibe_bottom {
            return true;
        }
        let invisible_left = self.trees[x].iter().take(y).any(|t| *t >= tree);
        if !invisible_left {
            return true;
        }
        let invisible_right = self.trees[x].iter().skip(y + 1).any(|t| *t >= tree);
        if !invisible_right {
            return true;
        }
        return false;
    }

    fn count_visible_trees(&self) -> usize {
        let outer_count = self.width * 2 + (self.height - 2) * 2;
        let mut inner_visible_count = 0;
        for i in 1..(self.height - 1) {
            for j in 1..(self.width - 1) {
                if self.is_tree_visible(i, j) {
                    inner_visible_count += 1;
                }
            }
        }
        outer_count + inner_visible_count
    }

    fn get_scenic_score(&self, x: usize, y: usize) -> u32 {
        let tree = self.trees[x][y];

        let counter_func = |(count, done), t| {
            if done {
                (count, done)
            } else {
                if t < tree {
                    (count + 1, false)
                } else {
                    (count + 1, true)
                }
            }
        };

        let visible_above = self
            .trees
            .iter()
            .take(x)
            .map(|r| r[y])
            .rev()
            .fold((0, false), counter_func)
            .0;
        let visible_below = self
            .trees
            .iter()
            .skip(x + 1)
            .map(|r| r[y])
            .fold((0, false), counter_func)
            .0;
        let visible_left = self.trees[x]
            .iter()
            .take(y)
            .rev()
            .map(u32::clone)
            .fold((0, false), counter_func)
            .0;
        let visible_right = self.trees[x]
            .iter()
            .skip(y + 1)
            .map(u32::clone)
            .fold((0, false), counter_func)
            .0;
        visible_left * visible_right * visible_below * visible_above
    }

    fn get_highest_scenic_score(&self) -> u32 {
        self.trees
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(j, t)| (i.clone(), j.clone(), t))
            })
            .map(|(i, j, _t)| self.get_scenic_score(i, j))
            .max()
            .unwrap()
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    Forest::parse(lines).count_visible_trees()
}

fn part2() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    Forest::parse(lines).get_highest_scenic_score()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
