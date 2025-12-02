use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
    ops::Range,
};

struct MapperEntry {
    source: Range<usize>,
    destination: Range<usize>,
}

impl MapperEntry {
    fn parse(line: &String) -> Self {
        let parts: Vec<&str> = line.split(" ").collect();
        let destination: usize = parts[0].trim().parse().unwrap();
        let source: usize = parts[1].trim().parse().unwrap();
        let length: usize = parts[2].trim().parse().unwrap();
        Self {
            destination: destination..(destination + length),
            source: source..(source + length),
        }
    }

    fn map(&self, value: &Range<usize>) -> (Range<usize>, Range<usize>, Range<usize>) {
        let mut before: Range<usize> = 0..0;
        let mut mid: Range<usize> = 0..0;
        let mut after: Range<usize> = 0..0;

        if self.source.contains(&value.start) {
            mid.start = self.destination.start + (value.start - self.source.start);
        } else {
            before.start = value.start;
            if self.source.contains(&(value.end - 1)) {
                before.end = self.source.start;
                mid.start = self.destination.start;
            } else {
                before.end = value.end;
            }
        }

        if self.source.contains(&(value.end - 1)) {
            mid.end = self.destination.start + (value.end - self.source.start);
        } else {
            after.end = value.end;
            if self.source.contains(&value.start) {
                after.start = self.source.end;
                mid.end = self.destination.end;
            } else {
                after.start = value.start;
            }
        }

        if value.contains(&self.source.start) && value.contains(&(self.source.end - 1)) {
            before.start = value.start;
            before.end = self.source.start;
            mid = self.destination.clone();
            after.start = self.source.end;
            after.end = value.end;
        }

        (before, mid, after)
    }
}

struct Mapper {
    source: String,
    destination: String,
    entries: Vec<MapperEntry>,
}

impl Mapper {
    fn parse(mut lines: Vec<String>) -> Self {
        let first = lines.remove(0);
        let mapping = first.replace("map:", "");
        let (source, destination) = mapping.trim().split_once("-to-").unwrap();
        let entries = lines.iter().map(MapperEntry::parse).collect();
        Self {
            source: source.to_string(),
            destination: destination.to_string(),
            entries,
        }
    }

    fn map(&self, value: &Range<usize>) -> Vec<Range<usize>> {
        let mut results = Vec::<Range<usize>>::new();

        loop {
            let mapped: Vec<(Range<usize>, Range<usize>, Range<usize>)> =
                self.entries.iter().map(|e| e.map(value)).collect();
            let mut things_to_check = Vec::<Range<usize>>::new();
            if let Some((before, mid, after)) = mapped.iter().find_map(|e| {
                let e = e.clone();
                if !e.1.is_empty() {
                    Some(e)
                } else {
                    None
                }
            }) {
                results.push(mid);
                things_to_check.push(before);
                things_to_check.push(after);
            } else {
                let mapped_thing = mapped.first().unwrap().clone();
                if mapped_thing.0 != *value {
                    things_to_check.push(mapped_thing.0);
                } else {
                    results.push(mapped_thing.0);
                }
                if mapped_thing.2 != *value {
                    things_to_check.push(mapped_thing.2);
                } else {
                    results.push(mapped_thing.2);
                }
            }

            let res = things_to_check
                .iter()
                .filter(|r| !r.is_empty())
                .map(|r| self.map(r))
                .flatten()
                .filter(|x| !x.is_empty())
                .collect::<Vec<Range<usize>>>();

            if res.clone().is_empty() || res.clone().iter().all(|x| results.contains(x)) {
                break;
            }

            res.iter().for_each(|x| results.push(x.clone()))
        }

        results.dedup();
        results
    }
}

struct Almanac {
    seeds: Vec<Range<usize>>,
    mappers: Vec<Mapper>,
}

impl Almanac {
    fn parse<T: BufRead>(mut lines: Lines<T>) -> Self {
        let seeds: Vec<Range<usize>> = (&lines.nth(0).map(Result::unwrap).unwrap())
            .to_string()
            .replace("seeds: ", "")
            .trim()
            .split(" ")
            .map(str::trim)
            .map(|x| x.parse().unwrap())
            .map(|x| x..(x + 1))
            .collect();

        let mut mappers: Vec<Mapper> = Vec::new();
        let mut curr_lines: Vec<String> = Vec::new();
        for line_result in lines {
            if let Ok(line) = line_result {
                if line.is_empty() {
                    continue;
                } else if curr_lines.is_empty() || !line.contains("map") {
                    curr_lines.push(line);
                } else {
                    mappers.push(Mapper::parse(curr_lines));
                    curr_lines = Vec::new();
                    curr_lines.push(line);
                }
            }
        }
        if !curr_lines.is_empty() {
            mappers.push(Mapper::parse(curr_lines));
        }

        Almanac { seeds, mappers }
    }

    fn parse_with_ranges<T: BufRead>(lines: Lines<T>) -> Self {
        let mut almanac = Self::parse(lines);
        let seeds = almanac.seeds;
        let new_seeds = seeds
            .iter()
            .map(|r| r.start)
            .collect::<Vec<usize>>()
            .chunks(2)
            .map(|w| w[0]..(w[0] + w[1]))
            .collect();
        almanac.seeds = new_seeds;
        almanac
    }

    fn convert(&self, into: &str) -> Vec<Range<usize>> {
        let mut from = "seed";
        let mut results = self.seeds.clone();
        while from != into {
            let mapper = self.mappers.iter().find(|m| m.source == from).unwrap();
            from = &mapper.destination;
            results = results.iter().map(|v| mapper.map(v)).flatten().collect();
            results.dedup();
        }
        results
    }
}

fn test() -> usize {
    let file = File::open("test_input.txt").unwrap();
    let reader = BufReader::new(file);
    Almanac::parse(reader.lines())
        .convert("location")
        .iter()
        .map(|x| x.start)
        .min()
        .unwrap()
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    Almanac::parse(reader.lines())
        .convert("location")
        .iter()
        .map(|x| x.start)
        .min()
        .unwrap()
}

fn test2() -> usize {
    let file = File::open("test_input.txt").unwrap();
    let reader = BufReader::new(file);
    Almanac::parse_with_ranges(reader.lines())
        .convert("location")
        .iter()
        .map(|x| x.start)
        .min()
        .unwrap()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    Almanac::parse_with_ranges(reader.lines())
        .convert("location")
        .iter()
        .map(|x| x.start)
        .min()
        .unwrap()
}

fn main() {
    println!("Test solution: {}", test());
    println!("Problem 1 solution: {}", part1());
    println!("Test 2 solution: {}", test2());
    println!("Problem 2 solution: {}", part2());
}
