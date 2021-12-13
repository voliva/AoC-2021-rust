use itertools::Itertools;
use ndarray::Array2;
use regex::Regex;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = (Vec<Coordinate>, Vec<Fold>);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let mut coordinates = Vec::new();
        let mut folds = Vec::new();

        let lines = file_reader.lines().map(|x| x.unwrap());

        for line in lines {
            if line == "" {
                continue;
            }

            if line.starts_with("fold") {
                folds.push(line.parse().unwrap());
            } else {
                coordinates.push(line.parse().unwrap());
            }
        }

        (coordinates, folds)
    }

    fn solve_first(&self, (coordinates, folds): &Self::Input) -> Result<Self::Output1, String> {
        let first_fold = &folds[0];
        let mut coordinate_set = HashSet::new();

        for c in coordinates {
            let folded = apply_fold(c, first_fold);
            coordinate_set.insert(folded);
        }

        Ok(coordinate_set.len())
    }

    fn solve_second(&self, (coordinates, folds): &Self::Input) -> Result<Self::Output2, String> {
        let mut coordinate_set = HashSet::new();

        for c in coordinates {
            let folded = apply_folds(c, &folds);
            coordinate_set.insert(folded);
        }

        let max = coordinate_set
            .iter()
            .fold((0, 0), |(max_x, max_y), c| (max_x.max(c.x), max_y.max(c.y)));
        let mut result = Array2::<usize>::zeros((max.1 + 1, max.0 + 1));

        for c in &coordinate_set {
            result[[c.y, c.x]] = 1;
        }

        for y in 0..max.1 + 1 {
            for x in 0..(max.0 + 1) * 2 {
                if result[[y, x / 2]] == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!("");
        }

        Ok(0)
    }
}

fn apply_folds(c: &Coordinate, folds: &Vec<Fold>) -> Coordinate {
    let mut result = c.clone();

    for f in folds {
        result = apply_fold(&result, f);
    }

    result
}

fn apply_fold(c: &Coordinate, f: &Fold) -> Coordinate {
    let mut result = c.clone();

    match f {
        Fold::X(v) if *v < c.x => {
            result.x = v - (result.x - v);
        }
        Fold::Y(v) if *v < c.y => {
            result.y = v - (result.y - v);
        }
        _ => {}
    }

    return result;
}

#[derive(Debug)]
pub enum Fold {
    X(usize),
    Y(usize),
}
impl FromStr for Fold {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref LINE_RGX: Regex = Regex::new(r"fold along (x|y)=(\d+)").unwrap();
        }

        let captures = LINE_RGX.captures(s).unwrap();

        let result = match &captures[1] {
            "x" => Fold::X(captures[2].parse().unwrap()),
            "y" => Fold::Y(captures[2].parse().unwrap()),
            _ => unreachable!(),
        };
        Ok(result)
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, PartialOrd, Ord)]
pub struct Coordinate {
    x: usize,
    y: usize,
}
impl FromStr for Coordinate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(",").collect_vec();

        Ok(Coordinate {
            x: split[0].parse().unwrap(),
            y: split[1].parse().unwrap(),
        })
    }
}
