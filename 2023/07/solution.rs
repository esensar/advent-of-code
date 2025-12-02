use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

const TYPE_POWER_STEP: usize = 1_000_000;

#[derive(Clone, Copy, Debug)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    fn determine(hand: &String) -> Self {
        let mut groups = HashMap::<char, usize>::new();
        for char in hand.chars() {
            *groups.entry(char).or_insert(0) += 1;
        }
        let mut sorted: Vec<(&char, &usize)> = groups.iter().collect();
        sorted.sort_by_key(|(_, count)| *count);
        sorted.reverse();
        let top_count = if *sorted[0].0 == 'X' {
            sorted[0].1 + *sorted.get(1).map(|x| x.1).unwrap_or(&0)
        } else {
            sorted[0].1 + groups.get(&'X').unwrap_or(&0)
        };
        match top_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if *sorted[1].1 == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if *sorted[1].1 == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            _ => HandType::HighCard,
        }
    }

    fn power(&self) -> usize {
        (*self as usize) * TYPE_POWER_STEP
    }
}

#[derive(Debug)]
struct Hand {
    cards: String,
    hand_type: HandType,
    power: usize,
}

impl Hand {
    fn new(cards: String) -> Self {
        let quindec_cards = cards
            .replace("A", "E")
            .replace("K", "D")
            .replace("Q", "C")
            .replace("J", "B")
            .replace("T", "A")
            .replace("X", "1");
        let cards_power = usize::from_str_radix(&quindec_cards, 15).unwrap();
        let hand_type = HandType::determine(&cards);
        Self {
            cards,
            hand_type,
            power: hand_type.power() + cards_power,
        }
    }
}

struct Hands {
    bids: Vec<(Hand, usize)>,
}

impl Hands {
    fn parse(lines: Vec<String>) -> Self {
        let mut bids = Vec::new();
        for line in lines {
            let (cards, bid) = line.split_once(" ").unwrap();
            bids.push((Hand::new(cards.to_string()), bid.parse().unwrap()));
        }
        Self { bids }
    }
}

fn test1() -> usize {
    let mut hands = Hands::parse(vec![
        "32T3K 765".to_string(),
        "T55J5 684".to_string(),
        "KK677 28".to_string(),
        "KTJJT 220".to_string(),
        "QQQJA 483".to_string(),
    ]);
    hands.bids.sort_by_key(|(h, _)| h.power);
    hands
        .bids
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, (_, bid))| acc + (rank + 1) * bid)
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut hands = Hands::parse(reader.lines().map(|l| l.unwrap()).collect());
    hands.bids.sort_by_key(|(h, _)| h.power);
    hands
        .bids
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, (_, bid))| acc + (rank + 1) * bid)
}

fn test2() -> usize {
    let mut hands = Hands::parse(vec![
        "32T3K 765".to_string(),
        "T55X5 684".to_string(),
        "KK677 28".to_string(),
        "KTXXT 220".to_string(),
        "QQQXA 483".to_string(),
    ]);
    hands.bids.sort_by_key(|(h, _)| h.power);
    hands
        .bids
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, (_, bid))| acc + (rank + 1) * bid)
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut hands = Hands::parse(
        reader
            .lines()
            .map(|l| l.unwrap().replace("J", "X"))
            .collect(),
    );
    hands.bids.sort_by_key(|(h, _)| h.power);
    hands
        .bids
        .iter()
        .enumerate()
        .fold(0, |acc, (rank, (_, bid))| acc + (rank + 1) * bid)
}

fn main() {
    println!("Test 1 solution: {}", test1());
    println!("Problem 1 solution: {}", part1());
    println!("Test 2 solution: {}", test2());
    println!("Problem 2 solution: {}", part2());
}
