use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(PartialEq, Eq, Clone, Debug)]
enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Packet::List(self_list) => match other {
                Packet::List(other_list) => {
                    for i in 0..self_list.len().max(other_list.len()) {
                        if matches!(self_list.get(i), None) {
                            return Some(Ordering::Less);
                        }
                        if matches!(other_list.get(i), None) {
                            return Some(Ordering::Greater);
                        }
                        let self_item = &self_list[i];
                        let other_item = &other_list[i];
                        if self_item < other_item {
                            return Some(Ordering::Less);
                        }
                        if self_item > other_item {
                            return Some(Ordering::Greater);
                        }
                    }
                    Some(Ordering::Equal)
                }
                Packet::Integer(other_int) => {
                    self.partial_cmp(&Packet::List(vec![Packet::Integer(*other_int)]))
                }
            },
            Packet::Integer(self_int) => match other {
                Packet::List(_) => {
                    Packet::List(vec![Packet::Integer(*self_int)]).partial_cmp(other)
                }
                Packet::Integer(other_int) => self_int.partial_cmp(other_int),
            },
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Packet {
    fn parse(line: &str) -> Self {
        if line.starts_with("[") {
            Packet::List(Self::parse_list(&line[1..]).0)
        } else {
            Packet::Integer(line.split(',').next().unwrap().parse().unwrap())
        }
    }

    fn parse_list(line: &str) -> (Vec<Self>, usize) {
        let result = line.chars().enumerate().fold(
            (Vec::<Self>::new(), Vec::<char>::new(), 0, 0),
            |(parts, last, done, skip), (i, next)| {
                if done != 0 {
                    return (parts, last, done, skip);
                }
                if skip > 0 {
                    return (parts, last, done, skip - 1);
                }
                if next.is_digit(10) {
                    let mut new = last.clone();
                    new.push(next);
                    return (parts, new, done, skip);
                }
                if next == ',' || next == ']' {
                    let done = if next == ']' { i + 1 } else { done };
                    if last.is_empty() {
                        return (parts, last, done, skip);
                    } else {
                        let packet = last.iter().collect::<String>().parse::<u32>().unwrap();
                        let mut new = parts.clone();
                        new.push(Packet::Integer(packet));
                        return (new, Vec::<char>::new(), done, skip);
                    }
                }
                if next == '[' {
                    let mut new = parts.clone();
                    let list = Self::parse_list(&line[(i + 1)..]);
                    new.push(Packet::List(list.0));
                    return (new, Vec::<char>::new(), done, skip + list.1);
                }
                panic!("What happened?");
            },
        );
        (result.0, result.2)
    }

    fn divider(val: u32) -> Self {
        Packet::List(vec![Packet::List(vec![Packet::Integer(val)])])
    }
}

fn parse_packet_pairs(lines: Lines<BufReader<File>>) -> Vec<(Packet, Packet)> {
    let lines = lines.flatten().collect::<Vec<String>>();
    lines
        .chunks(3)
        .map(|chunk| (Packet::parse(&chunk[0]), Packet::parse(&chunk[1])))
        .collect()
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let packet_pairs = parse_packet_pairs(lines);
    packet_pairs
        .iter()
        .enumerate()
        .filter(|(_i, (l, r))| l < r)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut packets = lines
        .flatten()
        .filter(|l| l.len() > 1)
        .map(|l| Packet::parse(&l))
        .collect::<Vec<Packet>>();
    packets.push(Packet::divider(2));
    packets.push(Packet::divider(6));
    packets.sort();

    (packets
        .iter()
        .enumerate()
        .find(|(_i, v)| **v == Packet::divider(2))
        .unwrap()
        .0
        + 1)
        * (packets
            .iter()
            .enumerate()
            .find(|(_i, v)| **v == Packet::divider(6))
            .unwrap()
            .0
            + 1)
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
