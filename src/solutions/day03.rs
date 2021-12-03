use super::Solver;
use itertools::partition;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

pub struct Problem;

impl Solver for Problem {
    type Input = (usize, Vec<usize>);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let mut lines = file_reader.lines().peekable();

        let length = lines.peek().unwrap().as_ref().unwrap().len();

        (
            length,
            lines
                .filter_map(|x| x.ok())
                .map(|line| usize::from_str_radix(line.as_str(), 2))
                .filter_map(|x| x.ok())
                .collect(),
        )
    }

    fn solve_first(&self, (bits, input): &Self::Input) -> Result<Self::Output1, String> {
        let mut vec = vec![0; *bits];

        for n in input {
            for i in 0..*bits {
                if n & (0x01 << i) > 0 {
                    vec[i] = vec[i] + 1
                }
            }
        }

        let n = input.len();
        let mut epsilon = 0;
        let mut gamma = 0;
        for i in 0..*bits {
            let mask = 0x01 << i;
            if vec[i] > n / 2 {
                gamma = gamma | mask
            } else {
                epsilon = epsilon | mask
            }
        }

        Ok(epsilon * gamma)
    }

    fn solve_second(&self, (bits, input): &Self::Input) -> Result<Self::Output2, String> {
        let mut clone = input.clone();

        let main_split = partition(&mut clone.iter_mut(), |x| x & (0x01 << (bits - 1)) >= 1);
        let next_bits = (bits - 2).try_into().unwrap();

        let (oxygen, co2) = if main_split > input.len() / 2 {
            (
                calculate_oxygen(&mut clone[..main_split], next_bits),
                calculate_co2(&mut clone[main_split..], next_bits),
            )
        } else {
            (
                calculate_oxygen(&mut clone[main_split..], next_bits),
                calculate_co2(&mut clone[..main_split], next_bits),
            )
        };

        Ok(oxygen * co2)
    }
}

fn calculate_oxygen(input: &mut [usize], bit: isize) -> usize {
    if input.len() == 1 {
        return input[0];
    }

    let split = partition(&mut input.iter_mut(), |x| x & (0x01 << bit) >= 1);

    let left = split;
    let right = input.len() - split;
    if left >= right {
        calculate_oxygen(&mut input[..split], bit - 1)
    } else {
        calculate_oxygen(&mut input[split..], bit - 1)
    }
}
fn calculate_co2(input: &mut [usize], bit: isize) -> usize {
    if input.len() == 1 {
        return input[0];
    }

    let split = partition(&mut input.iter_mut(), |x| x & (0x01 << bit) >= 1);

    let left = split;
    let right = input.len() - split;
    if left < right {
        calculate_co2(&mut input[..split], bit - 1)
    } else {
        calculate_co2(&mut input[split..], bit - 1)
    }
}
