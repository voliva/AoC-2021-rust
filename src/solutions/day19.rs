use itertools::Itertools;
use queues::{Buffer, IsQueue};

use super::Solver;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Problem;

/*
* => Si hi ha 12 nodes en comu, hi ha d'haver un node que tingui la mateixa distancia a 11 dels altres nodes
*/

const OVERLAP: usize = 12;

impl Solver for Problem {
    type Input = Vec<Vec<Position>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).skip(1);

        let mut result = Vec::new();
        let mut active = Vec::new();

        for l in lines {
            if l == "" {
                continue;
            }
            if l.starts_with("---") {
                result.push(active);
                active = Vec::new();
            } else {
                active.push(l.parse().unwrap());
            }
        }
        result.push(active);

        return result;
    }

    fn solve_first(&self, scanners: &Self::Input) -> Result<Self::Output1, String> {
        let distances = scanners.iter().map(|s| get_distances(s)).collect_vec();

        let matches = match_scanners(&distances);
        println!("{:?}", matches);

        Ok(0)
    }

    fn solve_second(&self, _: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}

fn match_scanners(distances: &Vec<Vec<Vec<isize>>>) -> HashMap<usize, Vec<usize>> {
    let mut to_match = Buffer::<usize>::new(distances.len());
    to_match.add(0).unwrap();
    let mut unmatched = HashSet::<usize>::from_iter(1..distances.len());
    let mut matches: HashMap<usize, Vec<usize>> = HashMap::new();

    while let Ok(i) = to_match.remove() {
        let v = matches.entry(i).or_insert(Vec::new());

        for i_match in find_matches(distances, i, &unmatched) {
            to_match.add(i_match).unwrap();
            unmatched.remove(&i_match);

            v.push(i_match);
        }
    }

    assert!(unmatched.len() == 0, "some scanners unmatched");

    matches
}

fn find_matches(
    distances: &Vec<Vec<Vec<isize>>>,
    base_i: usize,
    posible_matches: &HashSet<usize>,
) -> Vec<usize> {
    let base_match = &distances[base_i];
    posible_matches
        .iter()
        .filter(|i| {
            let posible_match = &distances[**i];

            return scanners_match(base_match, posible_match);
        })
        .map(|v| *v)
        .collect_vec()
}

fn scanners_match(distances_a: &Vec<Vec<isize>>, distances_b: &Vec<Vec<isize>>) -> bool {
    let mut overlapping_nodes = 0;
    for a in 0..distances_a.len() {
        let node_dist_a = &distances_a[a];
        for b in 0..distances_b.len() {
            let node_dist_b = &distances_b[b];
            if count_equal_elements(node_dist_a, node_dist_b) >= OVERLAP - 1 {
                overlapping_nodes += 1;
            }
            if overlapping_nodes >= OVERLAP {
                return true;
            }
        }
    }

    false
}

fn count_equal_elements(a: &Vec<isize>, b: &Vec<isize>) -> usize {
    let mut b_items: HashMap<isize, usize> = HashMap::new();
    for v in b {
        let r = b_items.entry(*v).or_insert(0);
        *r += 1;
    }

    return a
        .iter()
        .filter(|v| {
            let r = b_items.entry(**v).or_insert(0);
            if *r > 0 {
                *r -= 1;
                return true;
            }
            false
        })
        .count();
}

fn get_distances(positions: &Vec<Position>) -> Vec<Vec<isize>> {
    positions
        .iter()
        .map(|p0| {
            positions
                .iter()
                .filter(|p1| p0 != *p1)
                .map(|p1| p0.distance(p1))
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Position {
    fn distance(self: &Position, to: &Position) -> isize {
        (self.x - to.x).abs() + (self.y - to.y).abs() + (self.z - to.z).abs()
    }
}

impl FromStr for Position {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(",").collect_vec();

        Ok(Position {
            x: values[0].parse()?,
            y: values[1].parse()?,
            z: values[2].parse()?,
        })
    }
}
