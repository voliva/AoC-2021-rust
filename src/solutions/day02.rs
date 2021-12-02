use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<(isize, isize)>;
    type Output1 = isize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .filter_map(|x| x.ok())
            .map(|line| {
                let parts = line.split(" ").collect::<Vec<&str>>();

                let direction = parts[0];
                let quantity = parts[1].parse::<isize>().unwrap();

                // .to_owned() vs clone()?
                (direction.to_owned(), quantity)
            })
            .map(|(direction, quantity)| match &direction[..] {
                "forward" => (quantity, 0),
                "down" => (0, quantity),
                "up" => (0, -quantity),
                _ => (0, 0),
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let (horizontal, depth) = input
            .into_iter()
            .fold((0, 0), |(h0, d0), (h, d)| (h0 + h, d0 + d));

        Ok(horizontal * depth)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<isize, String> {
        let (horizontal, depth, _) = input.into_iter().fold((0, 0, 0), |(h0, d0, a0), (h, a)| {
            (h0 + h, d0 + (a0 * h), a0 + a)
        });

        Ok(horizontal * depth)
    }
}
