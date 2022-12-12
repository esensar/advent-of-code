use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(Clone)]
enum Node {
    Start,
    End,
    Square(i32),
}

impl Node {
    fn get_elevation(&self) -> i32 {
        match self {
            Node::Start => 'a' as i32 - 'a' as i32,
            Node::End => 'z' as i32 - 'a' as i32,
            Node::Square(elevation) => *elevation,
        }
    }
}

struct HeightMap {
    map: Vec<Node>,
    height: usize,
    width: usize,
    cache: HashMap<usize, usize>,
}

impl HeightMap {
    fn parse(lines: Lines<BufReader<File>>) -> Self {
        let mut height = 0;
        let mut width = 0;
        let mut map = Vec::<Node>::new();
        for line in lines.flatten() {
            width = line.len();
            height += 1;
            line.chars()
                .map(|c| match c {
                    'S' => Node::Start,
                    'E' => Node::End,
                    char => Node::Square(char as i32 - 'a' as i32),
                })
                .for_each(|n| map.push(n));
        }

        Self {
            map,
            height,
            width,
            cache: HashMap::new(),
        }
    }

    fn get_valid_neighbours(&self, pos: usize) -> Vec<usize> {
        let x = pos % self.width;
        let y = pos / self.width;
        let node = &self.map[pos];
        let mut neighbours = Vec::<usize>::new();
        if x > 0 {
            neighbours.push(y * self.width + x - 1);
        }
        if x < self.width - 1 {
            neighbours.push(y * self.width + x + 1);
        }
        if y > 0 {
            neighbours.push((y - 1) * self.width + x);
        }
        if y < self.height - 1 {
            neighbours.push((y + 1) * self.width + x);
        }
        neighbours
            .into_iter()
            .filter(|n| node.get_elevation() >= self.map[*n].get_elevation() - 1)
            .collect::<Vec<usize>>()
    }

    fn recur_on_paths(
        &mut self,
        current_pos: usize,
        visited: Vec<usize>,
        target_pos: usize,
    ) -> Option<usize> {
        if let Some(cached_val) = self.cache.get(&current_pos) {
            if cached_val <= &visited.len() {
                return None;
            }
        }
        self.cache.insert(current_pos, visited.len());
        if current_pos == target_pos {
            return Some(visited.len() - 1);
        }

        let mut paths = self
            .get_valid_neighbours(current_pos)
            .iter()
            .filter(|n| !visited.contains(n))
            .map(|n| {
                let mut new_visited = visited.clone();
                new_visited.push(*n);
                self.recur_on_paths(*n, new_visited, target_pos)
            })
            .flatten()
            .collect::<Vec<usize>>();
        paths.sort();
        paths.first().copied()
    }

    fn get_shortest_path_length(&mut self) -> usize {
        let start = self
            .map
            .iter()
            .enumerate()
            .find(|(i, val)| matches!(val, Node::Start))
            .unwrap();
        let end = self
            .map
            .iter()
            .enumerate()
            .find(|(i, val)| matches!(val, Node::End))
            .unwrap();
        let visited = vec![start.0];
        self.recur_on_paths(start.0, visited, end.0).unwrap()
    }

    fn get_shortest_possible_path_length(&mut self) -> usize {
        let map_clone = self.map.clone();
        let end = map_clone
            .iter()
            .enumerate()
            .find(|(i, val)| matches!(val, Node::End))
            .unwrap();
        let mut min = self.get_shortest_path_length();
        for (start, n) in map_clone
            .iter()
            .enumerate()
            .filter(|(i, n)| n.get_elevation() == 0)
        {
            let visited = vec![start];
            if let Some(new_val) = self.recur_on_paths(start, visited, end.0) {
                if new_val < min {
                    min = new_val;
                }
            }
        }
        min
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut map = HeightMap::parse(lines);
    map.get_shortest_path_length()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut map = HeightMap::parse(lines);
    map.get_shortest_possible_path_length()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
