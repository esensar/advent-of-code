use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

#[derive(Clone)]
struct Valve {
    id: String,
    flow_rate: u32,
    links: Vec<String>,
}

struct ValveMap {
    valves: Vec<Valve>,
    steps_cache: HashMap<String, HashMap<String, u32>>,
}

impl ValveMap {
    fn parse(lines: Lines<BufReader<File>>) -> Self {
        let mut valves = Vec::new();
        for line in lines.flatten() {
            let parts = line.split(';').collect::<Vec<&str>>();
            let valve_info = parts[0].split(" has flow rate=").collect::<Vec<&str>>();
            let id = valve_info[0].split(' ').last().unwrap().to_string();
            let flow_rate = valve_info[1].parse::<u32>().unwrap();
            let links = parts[1]
                .replace("tunnel leads to valve", "tunnels lead to valves")
                .split("tunnels lead to valves")
                .last()
                .unwrap()
                .split(", ")
                .map(|x| x.trim().to_string())
                .collect::<Vec<String>>();
            valves.push(Valve {
                id,
                flow_rate,
                links,
            })
        }
        ValveMap {
            valves,
            steps_cache: HashMap::new(),
        }
    }

    fn steps_from_to(&mut self, from: &str, to: &str, mut visited: Vec<String>) -> u32 {
        if let Some(cache) = self.steps_cache.get(from) {
            if let Some(steps) = cache.get(to) {
                return *steps;
            }
        }

        let valves_clone = self.valves.clone();
        let current = valves_clone
            .iter()
            .find(|v| v.id == from.to_string())
            .unwrap();
        if current.links.contains(&to.to_string()) {
            return 1;
        }
        visited.push(from.to_string());
        let result = 1 + current
            .links
            .iter()
            .filter(|l| !visited.contains(l))
            .map(|l| self.steps_from_to(l, to, visited.clone()))
            .min()
            .unwrap_or(1000);
        if result < 1000 {
            self.steps_cache
                .entry(from.to_string())
                .or_insert(HashMap::new())
                .insert(to.to_string(), result);
        }
        result
    }

    fn update_weights_map_cache(
        &mut self,
        weight_map_cache: &mut HashMap<String, (u32, u32)>,
        valve_map: Vec<Valve>,
        current_valve: &str,
        steps_left: u32,
    ) {
        let useful_valves = valve_map.iter().filter(|v| v.flow_rate > 0);

        useful_valves.for_each(|v| {
            let steps_needed = self.steps_from_to(current_valve, &v.id, Vec::new());
            if steps_needed + 1 <= steps_left {
                weight_map_cache.insert(
                    v.id.clone(),
                    (
                        (steps_left - steps_needed - 1) * v.flow_rate,
                        steps_needed + 1,
                    ),
                );
            }
        });
    }

    fn find_max_pressure_release_recur(
        &mut self,
        steps: u32,
        pressure_released: u32,
        current_valve: String,
        current_map: Vec<Valve>,
        results: &mut Vec<u32>,
    ) {
        if steps <= 0 {
            results.push(pressure_released);
            return;
        }
        let mut weight_map_cache = HashMap::new();
        self.update_weights_map_cache(
            &mut weight_map_cache,
            current_map.clone(),
            &current_valve,
            steps,
        );
        let mut iterated = false;
        for (next_valve, (next_pressure_val, steps_taken)) in
            weight_map_cache.iter().filter(|(_k, v)| v.0 > 0)
        {
            iterated = true;
            let mut new_map = current_map.clone();
            new_map
                .iter_mut()
                .find(|v| v.id == next_valve.to_string())
                .unwrap()
                .flow_rate = 0;
            self.find_max_pressure_release_recur(
                steps - steps_taken,
                pressure_released + next_pressure_val,
                next_valve.to_string(),
                new_map,
                results,
            );
        }
        if !iterated {
            results.push(pressure_released);
            return;
        }
    }

    fn find_max_pressure_release(&mut self, steps: u32) -> u32 {
        let mut results = Vec::new();
        self.find_max_pressure_release_recur(
            steps,
            0,
            "AA".to_string(),
            self.valves.clone(),
            &mut results,
        );
        *results.iter().max().unwrap()
    }

    fn find_max_pressure_release_two_recur(
        &mut self,
        steps: (u32, u32),
        done: (bool, bool),
        pressure_released: u32,
        current_valve: (String, String),
        current_map: Vec<Valve>,
        results: &mut Vec<u32>,
    ) {
        if steps.0 <= 0 && steps.1 <= 0 {
            results.push(pressure_released);
            return;
        }
        if current_map.iter().filter(|v| v.flow_rate > 0).count() == 0 {
            results.push(pressure_released);
            return;
        }
        if done.0 && done.1 {
            results.push(pressure_released);
            return;
        }

        if done.0 {
            let mut weight_map_cache = HashMap::new();
            self.update_weights_map_cache(
                &mut weight_map_cache,
                current_map.clone(),
                &current_valve.1,
                steps.1,
            );
            let mut iterated = false;
            for (next_valve, (next_pressure_val, steps_taken)) in
                weight_map_cache.iter().filter(|(_k, v)| v.0 > 0)
            {
                iterated = true;

                let mut new_map = current_map.clone();
                new_map
                    .iter_mut()
                    .find(|v| v.id == next_valve.to_string())
                    .unwrap()
                    .flow_rate = 0;
                self.find_max_pressure_release_two_recur(
                    (steps.0, steps.1 - steps_taken),
                    done,
                    pressure_released + next_pressure_val,
                    (current_valve.0.clone(), next_valve.to_string()),
                    new_map,
                    results,
                );
            }
            if !iterated {
                results.push(pressure_released);
                return;
            }
        } else {
            let mut weight_map_cache = HashMap::new();
            self.update_weights_map_cache(
                &mut weight_map_cache,
                current_map.clone(),
                &current_valve.0,
                steps.0,
            );
            let mut iterated = false;
            for (next_valve, (next_pressure_val, steps_taken)) in
                weight_map_cache.iter().filter(|(_k, v)| v.0 > 0)
            {
                iterated = true;

                let mut new_map = current_map.clone();
                new_map
                    .iter_mut()
                    .find(|v| v.id == next_valve.to_string())
                    .unwrap()
                    .flow_rate = 0;
                self.find_max_pressure_release_two_recur(
                    (steps.0 - steps_taken, steps.1),
                    done,
                    pressure_released + next_pressure_val,
                    (next_valve.to_string(), current_valve.1.clone()),
                    new_map,
                    results,
                );
            }
            if !iterated {
                self.find_max_pressure_release_two_recur(
                    steps,
                    (true, done.1),
                    pressure_released,
                    current_valve,
                    current_map,
                    results,
                );
            }
        }
    }

    fn find_max_pressure_release_two(&mut self, steps: u32) -> u32 {
        let mut results = Vec::new();
        self.find_max_pressure_release_two_recur(
            (steps, steps),
            (false, false),
            0,
            ("AA".to_string(), "AA".to_string()),
            self.valves.clone(),
            &mut results,
        );
        *results.iter().max().unwrap()
    }
}

fn part1() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut map = ValveMap::parse(lines);
    map.find_max_pressure_release(30)
}

fn part2() -> u32 {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut map = ValveMap::parse(lines);
    map.find_max_pressure_release_two(26)
}

fn main() {
    println!("Problem 1 solution: {}", part1());
    println!("Problem 2 solution: {}", part2());
}
