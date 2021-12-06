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
        let mut fish = input.clone();

        for _ in 0..80 {
            let mut fish_to_add = 0;

            fish.iter_mut().for_each(|v| {
                if *v == 0 {
                    fish_to_add += 1;
                    *v = 6;
                } else {
                    *v -= 1;
                }
            });

            for _ in 0..fish_to_add {
                fish.push(8);
            }
        }

        Ok(fish.len())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        const N: usize = 9;
        let mut days = vec![0; N];
        input.into_iter().fold(&mut days, |acc, fish| {
            acc[*fish] += 1;
            acc
        });

        let mut start: usize = 0;
        for _ in 0..256 {
            let fish_to_add = days[start];
            start = (start + 1) % N;

            days[(start + 6) % N] += fish_to_add;
        }

        Ok(days.into_iter().reduce(|a, b| a + b).unwrap())
    }
}
