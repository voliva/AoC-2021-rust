use itertools::Itertools;

use super::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = (String, HashMap<String, String>);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();

        let starting_polymer = lines[0].to_string();
        let mut polymer_rules = HashMap::new();

        for i in 2..lines.len() {
            let split = lines[i].split(" -> ").collect_vec();
            polymer_rules.insert(split[0].to_string(), split[1].to_string());
        }

        (starting_polymer, polymer_rules)
    }

    fn solve_first(&self, (polymer, rules): &Self::Input) -> Result<Self::Output1, String> {
        let mut map = polymer_to_map(polymer);

        for _ in 0..10 {
            map = polymerize(map, &rules);
        }

        let mut total_count_map = count_elements(map);
        increment_map(&mut total_count_map, &polymer[0..1], 1);
        increment_map(&mut total_count_map, &polymer[polymer.len() - 1..], 1);

        let sizes = total_count_map
            .values()
            .map(|v| v / 2)
            .sorted()
            .collect_vec();

        Ok(sizes[sizes.len() - 1] - sizes[0])
    }

    fn solve_second(&self, (polymer, rules): &Self::Input) -> Result<Self::Output2, String> {
        let mut map = polymer_to_map(polymer);

        for _ in 0..40 {
            map = polymerize(map, &rules);
        }

        let mut total_count_map = count_elements(map);
        increment_map(&mut total_count_map, &polymer[0..1], 1);
        increment_map(&mut total_count_map, &polymer[polymer.len() - 1..], 1);

        let sizes = total_count_map
            .values()
            .map(|v| v / 2)
            .sorted()
            .collect_vec();

        Ok(sizes[sizes.len() - 1] - sizes[0])
    }
}

fn polymer_to_map(polymer: &str) -> HashMap<String, usize> {
    let left = polymer.chars();
    let right = polymer.chars().skip(1);

    let pairs = left
        .zip(right)
        .map(|(l, r)| format!("{}{}", l, r))
        .collect_vec();

    let mut result: HashMap<String, usize> = HashMap::new();

    for p in pairs {
        increment_map(&mut result, &p, 1);
    }

    result
}

fn polymerize(
    polymer: HashMap<String, usize>,
    rules: &HashMap<String, String>,
) -> HashMap<String, usize> {
    let mut result = HashMap::new();

    for (key, quantity) in polymer {
        if !rules.contains_key(&key) {
            increment_map(&mut result, &key, 1);
            continue;
        }
        let interleave = rules.get(&key).unwrap();
        let mut interleaved = key.clone();
        interleaved.insert_str(1, interleave);

        increment_map(&mut result, &interleaved[0..2], quantity);
        increment_map(&mut result, &interleaved[1..], quantity);
    }

    result
}

fn count_elements(polymer: HashMap<String, usize>) -> HashMap<String, usize> {
    let mut total_count_map = HashMap::new();

    for (key, quantity) in polymer {
        let split = key.split("").collect_vec();
        increment_map(&mut total_count_map, split[1], quantity);
        increment_map(&mut total_count_map, split[2], quantity);
    }

    total_count_map
}

fn increment_map(map: &mut HashMap<String, usize>, key: &str, quantity: usize) {
    let r = map.get(key);
    let new_value = match r {
        Some(v) => v + quantity,
        None => quantity,
    };
    map.insert(key.to_string(), new_value);
}
