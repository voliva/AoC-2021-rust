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
        let result = input
            .into_iter()
            .map(|v| (0, v))
            .reduce(
                |(t, prev), (_, new)| {
                    if new > prev {
                        (t + 1, new)
                    } else {
                        (t, new)
                    }
                },
            )
            .unwrap();

        let (total, _) = result;

        Ok(total)
    }

    fn solve_second(&self, input: &Vec<isize>) -> Result<isize, String> {
        let sums = input
            .into_iter()
            .map(|v| (0, 0, v))
            .scan((0, 0, 0), |state, (_, _, v)| {
                let (_, v2, v1) = *state;
                *state = (v2, v1, *v);

                Some(v2 + v1 + *v)
            })
            .skip(2);

        let result = sums
            .map(|v| (0, v))
            .reduce(
                |(t, prev), (_, new)| {
                    if new > prev {
                        (t + 1, new)
                    } else {
                        (t, new)
                    }
                },
            )
            .unwrap();

        let (total, _) = result;

        Ok(total)
        // Not 1653
    }
}
