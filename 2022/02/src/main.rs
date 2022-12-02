use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn wins_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn loses_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    fn parse(string: &str) -> Hand {
        match string.chars().next().unwrap() {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissors,
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissors,
            _ => unimplemented!(),
        }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }

    fn parse(string: &str) -> Outcome {
        match string.chars().next().unwrap() {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unimplemented!(),
        }
    }
}

struct Game {
    left: Hand,
    right: Hand,
}

impl Game {
    fn outcome(&self) -> Outcome {
        if self.left == self.right {
            return Outcome::Draw;
        }

        if self.left.wins_against() == self.right {
            return Outcome::Loss;
        }

        Outcome::Win
    }

    fn game_score(&self) -> u32 {
        self.outcome().score() + self.right.score()
    }

    fn parse(game_line: &str) -> Game {
        let mut parts = game_line.split(' ');
        Game {
            left: Hand::parse(parts.next().unwrap()),
            right: Hand::parse(parts.next().unwrap()),
        }
    }
}

fn parse_hand_outcome_pair(outcome_line: &str) -> (Hand, Outcome) {
    let mut parts = outcome_line.split(' ');
    (
        Hand::parse(parts.next().unwrap()),
        Outcome::parse(parts.next().unwrap()),
    )
}

fn read_games<T: std::io::BufRead>(lines: Lines<T>) -> Vec<Game> {
    let mut games = Vec::new();
    for line in lines {
        let line = line.unwrap();
        games.push(Game::parse(&line));
    }
    games
}

fn read_game_outcomes<T: std::io::BufRead>(lines: Lines<T>) -> Vec<(Hand, Outcome)> {
    let mut game_outcomes = Vec::new();
    for line in lines {
        let line = line.unwrap();
        game_outcomes.push(parse_hand_outcome_pair(&line));
    }
    game_outcomes
}

fn hand_outcome_to_second_hand(hand: &Hand, outcome: Outcome) -> Hand {
    match outcome {
        Outcome::Loss => hand.wins_against(),
        Outcome::Draw => *hand,
        Outcome::Win => hand.loses_against(),
    }
}

fn part1() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let games = read_games(reader.lines());
    games.iter().map(|x| x.game_score()).sum::<u32>()
}

fn part2() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let games = read_game_outcomes(reader.lines());
    games
        .into_iter()
        .map(|(hand, outcome)| Game {
            left: hand,
            right: hand_outcome_to_second_hand(&hand, outcome),
        })
        .map(|x| x.game_score())
        .sum::<u32>()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
