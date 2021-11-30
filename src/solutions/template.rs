use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<isize>;
    type Output1 = isize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .filter_map(|x| x.ok())
            .map(|line| line.parse())
            .filter_map(|x| x.ok())
            .collect()
    }

    fn solve_first(&self, input: &Vec<isize>) -> Result<Self::Output1, String> {
        todo!()
    }

    fn solve_second(&self, input: &Vec<isize>) -> Result<isize, String> {
        todo!()
    }
}
