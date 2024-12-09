use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, Clone)]
enum Block {
    Free,
    File(usize),
}

impl Block {
    fn checksum(&self, position: usize) -> usize {
        match self {
            Block::Free => 0,
            Block::File(id) => id * position,
        }
    }

    fn is_free(&self) -> bool {
        matches!(self, Block::Free)
    }

    fn is_file(&self) -> bool {
        matches!(self, Block::File(_))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct DiskFile {
    id: usize,
    start: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct DiskMap {
    files: Vec<DiskFile>,
}

impl FromStr for DiskMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut files = Vec::new();
        let mut file = true;
        let mut id = 0usize;
        let mut pos = 0;
        for c in s.chars() {
            let digit = c.to_digit(10).unwrap() as usize;
            if file {
                files.push(DiskFile {
                    id,
                    start: pos,
                    size: digit,
                });
                id += 1;
            }
            file = !file;
            pos += digit;
        }
        Ok(Self { files })
    }
}

trait DiskBlockRepr {
    fn compact(&self) -> Self;

    fn checksum(&self) -> usize;
}

impl DiskBlockRepr for Vec<Block> {
    fn compact(&self) -> Self {
        let mut clone = self.clone();
        loop {
            let first_free = clone.iter().position(Block::is_free).unwrap_or(clone.len());
            let last_file =
                clone.len() - 1 - clone.iter().rev().position(Block::is_file).unwrap_or(0);
            if first_free >= last_file {
                return clone;
            }
            clone.swap(first_free, last_file);
        }
    }

    fn checksum(&self) -> usize {
        self.iter().enumerate().map(|(i, b)| b.checksum(i)).sum()
    }
}

impl DiskMap {
    fn to_block_map(&self) -> impl DiskBlockRepr {
        let mut block_map = Vec::new();
        let mut pos = 0usize;
        for file in &self.files {
            if file.start > pos {
                (pos..file.start).for_each(|_| block_map.push(Block::Free));
                pos = file.start;
            }
            (pos..(pos + file.size)).for_each(|_| block_map.push(Block::File(file.id)));
            pos += file.size;
        }
        block_map
    }

    fn compact_defrag(&self) -> Self {
        let mut clone = self.clone();
        for file in self.files.clone().iter().rev() {
            let required_size = file.size;
            let orig_start = file.start;
            let slot = clone
                .files
                .windows(2)
                .enumerate()
                .map(|(i, files)| {
                    let l = &files[0];
                    let r = &files[1];
                    (i, l, r, r.start - (l.start + l.size))
                })
                .find(|(_, l, _, size)| *size >= required_size && (l.start + l.size) < orig_start);
            if let Some((index, l, _, _)) = slot {
                let insert_pos = index + 1;
                let new_start = l.start + l.size;
                let mut removed_file = clone
                    .files
                    .remove(clone.files.iter().position(|f| f.id == file.id).unwrap())
                    .clone();
                removed_file.start = new_start;
                clone.files.insert(insert_pos, removed_file);
            }
        }
        clone
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap())
        .next()
        .unwrap()
        .parse::<DiskMap>()
        .unwrap()
        .to_block_map()
        .compact()
        .checksum()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.unwrap())
        .next()
        .unwrap()
        .parse::<DiskMap>()
        .unwrap()
        .compact_defrag()
        .to_block_map()
        .checksum()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}

#[cfg(test)]
mod tests {
    use crate::{DiskBlockRepr, DiskMap};

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            1928usize,
            INPUT
                .parse::<DiskMap>()
                .unwrap()
                .to_block_map()
                .compact()
                .checksum()
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            2858usize,
            INPUT
                .parse::<DiskMap>()
                .unwrap()
                .compact_defrag()
                .to_block_map()
                .checksum()
        );
    }
}
