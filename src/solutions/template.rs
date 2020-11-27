use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver<InputType, Output1Type, Output2Type> for Problem {
    fn read_input(&self, file_reader: BufReader<&File>) -> InputType {
        file_reader
            .lines()
            .filter_map(|x| x.ok())
            .map(|line| line.parse())
            .filter_map(|x| x.ok())
            .collect()
    }
    fn solve_first(&self, _input: &InputType) -> Result<Output1Type, String> {
        todo!()
    }
    fn solve_second(&self, _input: &InputType) -> Result<Output2Type, String> {
        todo!()
    }
}
