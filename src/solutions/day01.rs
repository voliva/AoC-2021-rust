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
        for i in 0..input.len() {
            for j in i..input.len() {
                if input[i] + input[j] == 2020 {
                    return Ok(input[i] * input[j]);
                }
            }
        }
        Err(String::from("Didn't find any"))
    }
    fn solve_second(&self, input: &Vec<isize>) -> Result<isize, String> {
        for i in 0..input.len() {
            for j in i..input.len() {
                for k in j..input.len() {
                    if input[i] + input[j] + input[k] == 2020 {
                        return Ok(input[i] * input[j] * input[k]);
                    }
                }
            }
        }
        Err(String::from("Didn't find any"))
    }
}
