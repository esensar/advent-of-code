use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

struct Card {
    card_id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let (card_id_part, rest) = line.split_once(":").unwrap();
        let card_id = card_id_part
            .replace("Card ", "")
            .trim()
            .parse::<usize>()
            .unwrap();
        let (winning_numbers_part, numbers_part) = rest.split_once("|").unwrap();
        let winning_numbers = winning_numbers_part
            .split(" ")
            .map(str::trim)
            .map(|n| n.parse::<usize>())
            .filter(Result::is_ok)
            .map(|o| o.unwrap())
            .collect();
        let numbers = numbers_part
            .split(" ")
            .map(str::trim)
            .map(|n| n.parse::<usize>())
            .filter(Result::is_ok)
            .map(|o| o.unwrap())
            .collect();
        Self {
            card_id,
            winning_numbers,
            numbers,
        }
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }

    fn matching_number_count(&self) -> usize {
        let mut count = 0;
        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                count += 1
            }
        }
        count
    }
}

fn parse_cards<T: BufRead>(lines: Lines<T>) -> Vec<Card> {
    let mut cards = Vec::new();
    for line in lines {
        cards.push(Card::parse(&line.unwrap()));
    }
    cards
}

fn test() -> usize {
    let cards = vec![
        Card {
            card_id: 1,
            winning_numbers: vec![41, 48, 83, 86, 17],
            numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        },
        Card {
            card_id: 2,
            winning_numbers: vec![13, 32, 20, 16, 61],
            numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
        },
        Card {
            card_id: 3,
            winning_numbers: vec![1, 21, 53, 59, 44],
            numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
        },
        Card {
            card_id: 4,
            winning_numbers: vec![41, 92, 73, 84, 69],
            numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
        },
        Card {
            card_id: 5,
            winning_numbers: vec![87, 83, 26, 28, 32],
            numbers: vec![88, 30, 70, 12, 94, 22, 82, 36],
        },
        Card {
            card_id: 6,
            winning_numbers: vec![31, 18, 13, 56, 72],
            numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
        },
    ];
    let mut starting_map = HashMap::<usize, usize>::new();
    for card in &cards {
        starting_map.insert(card.card_id, 1);
    }
    cards
        .iter()
        .fold(starting_map, |mut acc: HashMap<usize, usize>, card| {
            let score = card.matching_number_count();
            for inc_card_id in (card.card_id + 1)..=(card.card_id + score) {
                if inc_card_id > cards.len() {
                    break;
                }

                *acc.entry(inc_card_id).or_insert(1) += *acc.entry(card.card_id).or_insert(1);
            }
            acc
        })
        .values()
        .sum()
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    parse_cards(reader.lines()).iter().map(Card::score).sum()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let cards = parse_cards(reader.lines());
    let mut starting_map = HashMap::<usize, usize>::new();
    for card in &cards {
        starting_map.insert(card.card_id, 1);
    }
    cards
        .iter()
        .fold(starting_map, |mut acc: HashMap<usize, usize>, card| {
            let score = card.matching_number_count();
            for inc_card_id in (card.card_id + 1)..=(card.card_id + score) {
                if inc_card_id > cards.len() {
                    break;
                }

                *acc.entry(inc_card_id).or_insert(1) += *acc.entry(card.card_id).or_insert(1);
            }
            acc
        })
        .values()
        .sum()
}

fn main() {
    println!("Test solution: {}", test());
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
