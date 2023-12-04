use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

struct GameGrab {
    red: usize,
    blue: usize,
    green: usize,
}

impl GameGrab {
    fn parse(grab: &str) -> Self {
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;
        for part in grab.split(",") {
            if part.contains("blue") {
                blue = part.replace(" blue", "").trim().parse().unwrap();
            } else if part.contains("red") {
                red = part.replace(" red", "").trim().parse().unwrap();
            } else if part.contains("green") {
                green = part.replace(" green", "").trim().parse().unwrap();
            }
        }
        Self { red, blue, green }
    }

    fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}

struct Game {
    game_id: usize,
    grabs: Vec<GameGrab>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let (game_id_part, rest) = line.split_once(":").unwrap();
        let game_id = game_id_part.replace("Game ", "").parse::<usize>().unwrap();
        let grabs = rest.split(";").map(GameGrab::parse).collect();
        Self { game_id, grabs }
    }

    fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.grabs
            .iter()
            .map(|grab| grab.is_possible(red, green, blue))
            .fold(true, |acc, x| acc && x)
    }

    fn min_set(&self) -> GameGrab {
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;
        for grab in &self.grabs {
            min_red = grab.red.max(min_red);
            min_blue = grab.blue.max(min_blue);
            min_green = grab.green.max(min_green);
        }
        GameGrab {
            red: min_red,
            blue: min_blue,
            green: min_green,
        }
    }
}

fn parse_games<T: BufRead>(lines: Lines<T>) -> Vec<Game> {
    let mut games = Vec::new();
    for line in lines {
        games.push(Game::parse(&line.unwrap()));
    }
    games
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    parse_games(reader.lines())
        .iter()
        .filter(|g| g.is_possible(12, 13, 14))
        .map(|g| g.game_id)
        .sum()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    parse_games(reader.lines())
        .iter()
        .map(Game::min_set)
        .map(|set| set.red * set.blue * set.green)
        .sum()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
