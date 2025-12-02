use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    str::FromStr,
};

struct Cpu {
    register_history: Vec<i32>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            register_history: vec![1],
        }
    }

    fn execute_command(&mut self, command: &Command) {
        let last_value = self.register_history.last().unwrap().clone();
        match command {
            Command::Addx(value) => {
                self.register_history.push(last_value);
                self.register_history.push(last_value + value);
            }
            Command::Noop => self.register_history.push(last_value),
        }
    }
}

enum Command {
    Addx(i32),
    Noop,
}

struct Crt {
    width: usize,
    height: usize,
    image: Vec<bool>,
}

impl Crt {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            image: Vec::new(),
        }
    }

    fn run_cycles(&mut self, cpu: &Cpu) {
        self.image.clear();
        for i in 0..(self.width * self.height) {
            if cpu.register_history[i].abs_diff((i % self.width).try_into().unwrap()) <= 1 {
                self.image.push(true);
            } else {
                self.image.push(false);
            }
        }
    }
}

impl ToString for Crt {
    fn to_string(&self) -> String {
        String::from_str("\n\n").unwrap()
            + &self
                .image
                .chunks(self.width)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|active| if *active { '#' } else { '.' })
                        .collect::<String>()
                })
                .reduce(|l, r| l + "\n" + &r)
                .unwrap()
    }
}

fn get_signal_strength(cpu: &Cpu) -> i32 {
    cpu.register_history[19] * 20
        + cpu.register_history[59] * 60
        + cpu.register_history[99] * 100
        + cpu.register_history[139] * 140
        + cpu.register_history[179] * 180
        + cpu.register_history[219] * 220
}

fn parse_commands(lines: Lines<BufReader<File>>) -> Vec<Command> {
    let mut commands = Vec::new();
    for line in lines.flatten() {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[0] {
            "noop" => commands.push(Command::Noop),
            "addx" => commands.push(Command::Addx(parts[1].parse().unwrap())),
            _ => panic!("Unknown command: {}", parts[0]),
        }
    }
    commands
}

fn part1() -> i32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut cpu = Cpu::new();
    let commands = parse_commands(lines);

    commands.iter().for_each(|c| cpu.execute_command(c));
    get_signal_strength(&cpu)
}

fn part2() -> String {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut cpu = Cpu::new();
    let commands = parse_commands(lines);

    commands.iter().for_each(|c| cpu.execute_command(c));

    let mut crt = Crt::new(40, 6);
    crt.run_cycles(&cpu);

    crt.to_string()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
