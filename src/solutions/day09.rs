use itertools::Itertools;
use ndarray::Array2;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Array2<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines: Vec<Vec<u8>> = file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.bytes().map(|c| c - b'0').collect::<Vec<u8>>())
            .collect();

        ndarray::Array2::from_shape_fn((lines.len(), lines[0].len()), |(r, c)| lines[r][c])
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let result = get_minima(input)
            .into_iter()
            .map(|(_, v)| 1 + (v as usize))
            .sum();

        Ok(result)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let result = get_minima(input)
            .into_iter()
            .map(|(p, _)| {
                let r = get_basin(input, p);
                return r;
            })
            .sorted()
            .rev()
            .take(3)
            .fold(1, |acc, v| acc * v);

        Ok(result)
    }
}

fn get_basin(input: &Array2<u8>, minima: (usize, usize)) -> usize {
    let mut checked = HashSet::new();

    expand_basin(input, &mut checked, minima)
}

fn expand_basin(
    input: &Array2<u8>,
    checked: &mut HashSet<(usize, usize)>,
    pos: (usize, usize),
) -> usize {
    let self_value = input[[pos.0, pos.1]];
    if checked.contains(&pos) || (self_value == 9) {
        return 0;
    }

    let adj: Vec<(usize, usize)> = adjacent(input.shape(), pos)
        .into_iter()
        .filter(|p| !checked.contains(p))
        .collect();

    let is_minimum = adj.iter().all(|p| input[[p.0, p.1]] >= self_value);

    if !is_minimum {
        return 0;
    }

    checked.insert(pos);
    let adj_size: usize = adj
        .into_iter()
        .map(|p| expand_basin(input, checked, p))
        .sum();

    adj_size + 1
}

fn get_minima(input: &Array2<u8>) -> Vec<((usize, usize), u8)> {
    let shape = input.shape();
    input
        .indexed_iter()
        .filter(|(position, v)| {
            let a = adjacent(shape, *position);
            a.iter().all(|(r, c)| input.get((*r, *c)).unwrap() > v)
        })
        .map(|(p, v)| (p, *v))
        .collect()
}

fn adjacent(shape: &[usize], (r, c): (usize, usize)) -> Vec<(usize, usize)> {
    let rs = match r {
        0 => vec![r + 1],
        v if v == shape[0] - 1 => vec![v - 1],
        v => vec![v - 1, v + 1],
    };
    let cs = match c {
        0 => vec![c + 1],
        v if v == shape[1] - 1 => vec![v - 1],
        v => vec![v - 1, v + 1],
    };

    rs.iter()
        .map(|r| (*r, c))
        .chain(cs.iter().map(|c| (r, *c)))
        .collect()
}
