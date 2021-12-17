use regex::Regex;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = ((isize, isize), (isize, isize));
    type Output1 = isize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let line = file_reader.lines().map(|x| x.unwrap()).next().unwrap();

        let regex = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();

        let captures = regex.captures(&line).unwrap();

        (
            (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let (x, y) = *input;
        let candidates_x = i_sqrt(2 * x.0)..=i_sqrt(2 * x.1);
        // I couldn't figure out the bounds for y_candidates :( - So I hardcoded something that should be enough.
        // let candidates_y = (y.0 + x.0) / i_sqrt(2 * x.0)..=(y.1 + x.1) / i_sqrt(2 * x.1);
        let candidates_y = 0..1000;

        let mut result = 0;

        for vx in candidates_x {
            for vy in candidates_y.clone() {
                if let Some(v) = simulate((vx, vy), (x, y)) {
                    result = result.max(v);
                }
            }
        }

        Ok(result)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let (x, y) = *input;
        let candidates_x = i_sqrt(2 * x.0)..=x.1;
        // let candidates_y = (y.0 + x.0) / i_sqrt(2 * x.0)..=(y.1 + x.1) / i_sqrt(2 * x.1);
        let candidates_y = y.0..1000;

        let mut result = 0;

        for vx in candidates_x {
            for vy in candidates_y.clone() {
                if let Some(_) = simulate((vx, vy), (x, y)) {
                    result += 1;
                }
            }
        }

        Ok(result)
    }
}

fn i_sqrt(num: isize) -> isize {
    (num as f64).sqrt() as isize
}

fn simulate(velocity: (isize, isize), (x, y): ((isize, isize), (isize, isize))) -> Option<isize> {
    let mut pos = (0, 0);
    let mut velocity = velocity.clone();
    let mut max_height = 0;

    loop {
        pos.0 += velocity.0;
        pos.1 += velocity.1;
        velocity.0 = (velocity.0 - 1).max(0);
        velocity.1 = velocity.1 - 1;

        max_height = max_height.max(pos.1);

        if x.0 <= pos.0 && pos.0 <= x.1 && y.0 <= pos.1 && pos.1 <= y.1 {
            return Some(max_height);
        }
        if pos.0 > x.1 || pos.1 < y.0 {
            return None;
        }
    }
}
