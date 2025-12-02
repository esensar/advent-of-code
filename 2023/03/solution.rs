use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(Debug)]
struct Gear {
    x: usize,
    y: usize,
    numbers: Vec<usize>,
}

impl Gear {
    fn ratio(&self) -> Option<usize> {
        if self.numbers.len() == 2 {
            Some(self.numbers[0] * self.numbers[1])
        } else {
            None
        }
    }
}

struct Schematic {
    matrix: Vec<Vec<char>>,
}

impl Schematic {
    fn parse<T: BufRead>(lines: Lines<T>) -> Self {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        for line in lines {
            matrix.push(line.unwrap().chars().collect());
        }
        Self { matrix }
    }

    fn part_numbers(&self) -> Vec<usize> {
        let neighbours: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut numbers = Vec::new();
        for (x, row) in self.matrix.iter().enumerate() {
            let mut y = 0;
            while y < row.len() {
                let c = row[y];
                if c.is_digit(10) {
                    let number_string = row
                        .iter()
                        .skip(y)
                        .take_while(|ch| ch.is_digit(10))
                        .collect::<String>();
                    let start = y;
                    y += number_string.len();
                    let end = y;
                    'number_seeker: for i in start..end {
                        for neighbour in &neighbours {
                            if let Some(Some(sym)) = self
                                .matrix
                                .get((x as i32 + neighbour.0).max(0) as usize)
                                .map(|r| r.get((i as i32 + neighbour.1).max(0) as usize))
                            {
                                if !sym.is_digit(10) && *sym != '.' {
                                    let number = number_string.parse::<usize>().unwrap();
                                    numbers.push(number);
                                    break 'number_seeker;
                                }
                            }
                        }
                    }
                }
                y += 1;
            }
        }
        numbers
    }

    fn gears(&self) -> Vec<Gear> {
        let neighbours: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut gears = Vec::<Gear>::new();
        for (x, row) in self.matrix.iter().enumerate() {
            let mut y = 0;
            while y < row.len() {
                let c = row[y];
                if c.is_digit(10) {
                    let number_string = row
                        .iter()
                        .skip(y)
                        .take_while(|ch| ch.is_digit(10))
                        .collect::<String>();
                    let start = y;
                    y += number_string.len();
                    let end = y;
                    for i in start..end {
                        for neighbour in &neighbours {
                            let gear_x = (x as i32 + neighbour.0).max(0) as usize;
                            let gear_y = (i as i32 + neighbour.1).max(0) as usize;
                            if let Some(Some(sym)) = self.matrix.get(gear_x).map(|r| r.get(gear_y))
                            {
                                if *sym == '*' {
                                    let mut gear =
                                        gears.iter_mut().find(|g| g.x == gear_x && g.y == gear_y);
                                    if gear.is_none() {
                                        let new_gear = Gear {
                                            x: gear_x,
                                            y: gear_y,
                                            numbers: Vec::new(),
                                        };
                                        gears.push(new_gear);
                                        gear = gears
                                            .iter_mut()
                                            .find(|g| g.x == gear_x && g.y == gear_y);
                                    }
                                    let number = number_string.parse::<usize>().unwrap();
                                    let gear_numbers = &mut gear.unwrap().numbers;
                                    if !gear_numbers.contains(&number) {
                                        gear_numbers.push(number);
                                    }
                                }
                            }
                        }
                    }
                }
                y += 1;
            }
        }
        gears
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    Schematic::parse(reader.lines()).part_numbers().iter().sum()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    Schematic::parse(reader.lines())
        .gears()
        .iter()
        .map(Gear::ratio)
        .filter(Option::is_some)
        .map(Option::unwrap)
        .sum()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
