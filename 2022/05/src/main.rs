use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Command {
    boxes: usize,
    from: usize,
    to: usize,
}

impl Command {
    fn parse(line: &str) -> Self {
        let parts: Vec<&str> = line.split(' ').collect();
        Self {
            boxes: parts[1].parse::<usize>().unwrap(),
            from: parts[3].parse::<usize>().unwrap() - 1,
            to: parts[5].parse::<usize>().unwrap() - 1,
        }
    }

    fn execute(&self, stacks: &mut [Vec<char>]) {
        for _ in 0..self.boxes {
            let b = stacks.get_mut(self.from).unwrap().pop().unwrap();
            stacks.get_mut(self.to).unwrap().push(b);
        }
    }

    fn execute_9001(&self, stacks: &mut [Vec<char>]) {
        let from = stacks.get_mut(self.from).unwrap();
        let mut last_boxes = from.split_off(from.len() - self.boxes);
        stacks.get_mut(self.to).unwrap().append(&mut last_boxes);
    }
}

fn parse_input() -> (Vec<Command>, Vec<Vec<char>>) {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut stack_lines = Vec::<String>::new();
    let mut commands = Vec::<Command>::new();

    for line in lines.flatten() {
        if line.starts_with("move") {
            commands.push(Command::parse(&line));
        } else if !line.is_empty() {
            stack_lines.push(line);
        }
    }

    let stack_count = stack_lines.last().unwrap().len() / 4 + 1;

    let mut stacks = Vec::<Vec<char>>::with_capacity(stack_count);

    for _ in 0..stack_count {
        stacks.push(Vec::<char>::new());
    }

    stack_lines.pop();

    for line in stack_lines {
        let line_chars = line.chars().collect::<Vec<char>>();
        let boxes = line_chars
            .chunks(4)
            .map(|c| c.iter().take(3).collect::<String>().trim().to_owned());

        for (i, b) in boxes.enumerate() {
            if !b.is_empty() {
                stacks[i].insert(0, b.chars().collect::<Vec<char>>()[1]);
            }
        }
    }

    (commands, stacks)
}

fn part1() -> String {
    let (commands, mut stacks) = parse_input();

    for command in commands {
        command.execute(&mut stacks);
    }

    stacks
        .iter()
        .map(|s| s.last().unwrap_or(&' '))
        .collect::<String>()
}

fn part2() -> String {
    let (commands, mut stacks) = parse_input();

    for command in commands {
        command.execute_9001(&mut stacks);
    }

    stacks
        .iter()
        .map(|s| s.last().unwrap_or(&' '))
        .collect::<String>()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
