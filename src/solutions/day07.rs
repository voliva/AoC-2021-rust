use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .filter_map(|x| x.ok())
            .next()
            .unwrap()
            .split(",")
            .map(|line| line.parse())
            .filter_map(|x| x.ok())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        fn calc_fuel(a: usize, b: usize) -> usize {
            (a as isize - b as isize).abs() as usize
        }

        let max_position = input.iter().fold(0, |acc, x| acc.max(*x));

        let fuel_cost = (0..max_position + 1)
            .map(|p: usize| input.iter().fold(0, |acc, x| acc + calc_fuel(p, *x)));

        Ok(fuel_cost.fold(usize::MAX, |acc, x| acc.min(x)))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        fn calc_fuel(a: usize, b: usize) -> usize {
            let d = (a as isize - b as isize).abs() as usize;
            (d * (d + 1)) / 2
        }

        let max_position = input.iter().fold(0, |acc, x| acc.max(*x));

        let fuel_cost = (0..max_position + 1)
            .map(|p: usize| input.iter().fold(0, |acc, x| acc + calc_fuel(p, *x)));

        Ok(fuel_cost.fold(usize::MAX, |acc, x| acc.min(x)))
    }
}
