use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Plant(char);

impl From<char> for Plant {
    fn from(value: char) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Bounds(usize, usize);

impl Bounds {
    const MAX: Bounds = Bounds(usize::MAX, usize::MAX);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(usize, usize);

impl Point {
    fn neighbors(&self, bounds: &Bounds) -> HashSet<Point> {
        Direction::ALL
            .iter()
            .filter_map(|d| d.move_position(self, bounds))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    const ALL: &[Direction; 4] = &[
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
    const CORNERS: &[(Direction, Direction); 4] = &[
        (Direction::Up, Direction::Right),
        (Direction::Up, Direction::Left),
        (Direction::Down, Direction::Right),
        (Direction::Down, Direction::Left),
    ];

    fn move_position(&self, pos: &Point, bounds: &Bounds) -> Option<Point> {
        match self {
            Direction::Up => pos.1.checked_sub(1).map(|r| Point(pos.0, r)),
            Direction::Right => {
                if pos.0 + 1 < bounds.0 {
                    Some(Point(pos.0 + 1, pos.1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if pos.1 + 1 < bounds.1 {
                    Some(Point(pos.0, pos.1 + 1))
                } else {
                    None
                }
            }
            Direction::Left => pos.0.checked_sub(1).map(|l| Point(l, pos.1)),
        }
    }
}

#[derive(Debug, Clone)]
struct Region {
    plant: Plant,
    points: HashSet<Point>,
}

impl Region {
    fn find(
        plant: Plant,
        start_pos: Point,
        bounds: &Bounds,
        map: &mut [Vec<Option<Plant>>],
    ) -> Self {
        let mut points = HashSet::new();
        points.insert(start_pos.clone());
        let mut to_visit: Vec<Point> = start_pos.neighbors(bounds).into_iter().collect();
        while let Some(next) = to_visit.pop() {
            if let Some(p) = &map[next.0][next.1] {
                if *p == plant {
                    map[next.0][next.1].take();
                    points.insert(next.clone());
                    to_visit.extend(next.neighbors(bounds))
                }
            }
        }
        Self { plant, points }
    }

    fn area(&self) -> usize {
        self.points.len()
    }

    fn perimeter(&self) -> Vec<Point> {
        let modified_points: HashSet<Point> = self
            .points
            .iter()
            .map(|p| Point(p.0 + 1, p.1 + 1))
            .collect();
        modified_points
            .iter()
            .flat_map(|p| p.neighbors(&Bounds::MAX))
            .filter(|p| !modified_points.contains(p))
            .collect::<Vec<_>>()
    }

    fn edges(&self) -> HashSet<Point> {
        let perimeter = self.perimeter();
        self.points
            .iter()
            .filter(|p| {
                p.neighbors(&Bounds::MAX)
                    .iter()
                    .any(|n| perimeter.contains(n))
            })
            .cloned()
            .collect()
    }

    fn perimeter_size(&self) -> usize {
        self.perimeter().len()
    }

    fn sides(&self) -> usize {
        let modified_points: HashSet<Point> = self
            .points
            .iter()
            .map(|p| Point(p.0 + 1, p.1 + 1))
            .collect();
        modified_points
            .iter()
            .map(|p| {
                let perimeter_part: Vec<Point> = p
                    .neighbors(&Bounds::MAX)
                    .iter()
                    .filter(|n| !modified_points.contains(n))
                    .cloned()
                    .collect();

                let outer_sides = match perimeter_part.len() {
                    4 => 4, // Case of a single plant plot
                    3 => 2, // Case a part sticking out of plot
                    2 if perimeter_part[0].0 != perimeter_part[1].0
                        && perimeter_part[0].1 != perimeter_part[1].1 =>
                    {
                        1
                    }
                    _ => 0,
                };

                let inner_sides = Direction::CORNERS
                    .iter()
                    .filter(|(hd, vd)| {
                        modified_points.contains(&hd.move_position(p, &Bounds::MAX).unwrap())
                            && modified_points.contains(&vd.move_position(p, &Bounds::MAX).unwrap())
                            && !modified_points.contains(
                                &hd.move_position(p, &Bounds::MAX)
                                    .and_then(|np| vd.move_position(&np, &Bounds::MAX))
                                    .unwrap(),
                            )
                    })
                    .count();

                outer_sides + inner_sides
            })
            .sum()
    }

    fn fencing_cost(&self) -> usize {
        self.area() * self.perimeter_size()
    }

    fn bulk_fencing_cost(&self) -> usize {
        self.area() * self.sides()
    }
}

#[derive(Debug, Clone)]
struct GardenMap {
    map: Vec<Vec<Plant>>,
    bounds: Bounds,
}

impl GardenMap {
    fn parse_input(lines: &mut dyn Iterator<Item = String>) -> Self {
        let map: Vec<Vec<Plant>> = lines.map(|l| l.chars().map(Into::into).collect()).collect();
        Self {
            bounds: Bounds(map.len(), map.first().unwrap().len()),
            map,
        }
    }

    fn regions(&self) -> Vec<Region> {
        let mut opt_map = self
            .map
            .clone()
            .into_iter()
            .map(|l| l.into_iter().map(Some).collect())
            .collect::<Vec<Vec<Option<Plant>>>>();
        let mut regions = Vec::new();
        for (i, row) in self.map.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if let Some(plant) = &opt_map[i][j] {
                    regions.push(Region::find(
                        plant.clone(),
                        Point(i, j),
                        &self.bounds,
                        &mut opt_map,
                    ));
                }
            }
        }
        regions
    }
}

fn part1() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let map = GardenMap::parse_input(&mut lines);

    map.regions().iter().map(Region::fencing_cost).sum()
}

fn part2() -> usize {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|l| l.unwrap());
    let map = GardenMap::parse_input(&mut lines);

    map.regions().iter().map(Region::bulk_fencing_cost).sum()
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{GardenMap, Plant, Point, Region};

    const INPUT: &str =
        "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";

    #[test]
    fn test_region_intro() {
        let map =
            GardenMap::parse_input(&mut "AAAA\nBBCD\nBBCC\nEEEC".lines().map(|l| l.to_string()));
        let regions = map.regions();

        let region_a = regions.iter().find(|r| r.plant == Plant('A')).unwrap();
        assert_eq!(
            HashSet::from([Point(0, 0), Point(0, 1), Point(0, 2), Point(0, 3)]),
            region_a.points
        );
        assert_eq!(4, region_a.area());
        assert_eq!(10, region_a.perimeter_size());
        assert_eq!(4, region_a.sides());

        let region_b = regions.iter().find(|r| r.plant == Plant('B')).unwrap();
        assert_eq!(
            HashSet::from([Point(1, 0), Point(1, 1), Point(2, 0), Point(2, 1)]),
            region_b.points
        );
        assert_eq!(4, region_b.area());
        assert_eq!(8, region_b.perimeter_size());
        assert_eq!(4, region_b.sides());

        let region_c = regions.iter().find(|r| r.plant == Plant('C')).unwrap();
        assert_eq!(
            HashSet::from([Point(1, 2), Point(2, 2), Point(2, 3), Point(3, 3)]),
            region_c.points
        );
        assert_eq!(4, region_c.area());
        assert_eq!(10, region_c.perimeter_size());
        assert_eq!(8, region_c.sides());

        let region_d = regions.iter().find(|r| r.plant == Plant('D')).unwrap();
        assert_eq!(HashSet::from([Point(1, 3)]), region_d.points);
        assert_eq!(1, region_d.area());
        assert_eq!(4, region_d.perimeter_size());
        assert_eq!(4, region_d.sides());

        let region_e = regions.iter().find(|r| r.plant == Plant('E')).unwrap();
        assert_eq!(
            HashSet::from([Point(3, 0), Point(3, 1), Point(3, 2)]),
            region_e.points
        );
        assert_eq!(3, region_e.area());
        assert_eq!(8, region_e.perimeter_size());
        assert_eq!(4, region_e.sides());
    }

    #[test]
    fn test_region_intro_cost() {
        let map =
            GardenMap::parse_input(&mut "AAAA\nBBCD\nBBCC\nEEEC".lines().map(|l| l.to_string()));
        let regions = map.regions();

        assert_eq!(5, regions.len());

        assert_eq!(140usize, regions.iter().map(Region::fencing_cost).sum());
    }

    #[test]
    fn test_region_same_plant_intro() {
        let map = GardenMap::parse_input(
            &mut "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO"
                .lines()
                .map(|l| l.to_string()),
        );
        let regions = map.regions();

        assert_eq!(5, regions.len());

        assert_eq!(772usize, regions.iter().map(Region::fencing_cost).sum());
    }

    #[test]
    fn test_part_1_example() {
        let map = GardenMap::parse_input(&mut INPUT.lines().map(|l| l.to_string()));

        assert_eq!(
            1930usize,
            map.regions().iter().map(Region::fencing_cost).sum()
        );
    }

    #[test]
    fn test_region_intro_bulk_cost() {
        let map =
            GardenMap::parse_input(&mut "AAAA\nBBCD\nBBCC\nEEEC".lines().map(|l| l.to_string()));
        let regions = map.regions();

        assert_eq!(80usize, regions.iter().map(Region::bulk_fencing_cost).sum());
    }

    #[test]
    fn test_e_shaped_e_bulk_cost() {
        let map = GardenMap::parse_input(
            &mut "EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE"
                .lines()
                .map(|l| l.to_string()),
        );
        let regions = map.regions();

        assert_eq!(
            236usize,
            regions.iter().map(Region::bulk_fencing_cost).sum()
        );
    }

    #[test]
    fn test_abba_bulk_cost() {
        let map = GardenMap::parse_input(
            &mut "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA"
                .lines()
                .map(|l| l.to_string()),
        );
        let regions = map.regions();

        assert_eq!(
            368usize,
            regions.iter().map(Region::bulk_fencing_cost).sum()
        );
    }

    #[test]
    fn test_part_2_example() {
        let map = GardenMap::parse_input(&mut INPUT.lines().map(|l| l.to_string()));

        assert_eq!(
            1206usize,
            map.regions().iter().map(Region::bulk_fencing_cost).sum()
        );
    }
}
