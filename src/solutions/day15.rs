use ndarray::Array2;
use pathfinding::dijkstra;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Array2<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines: Vec<Vec<usize>> = file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                line.bytes()
                    .map(|c| (c - b'0') as usize)
                    .collect::<Vec<usize>>()
            })
            .collect();

        ndarray::Array2::from_shape_fn((lines.len(), lines[0].len()), |(r, c)| lines[r][c])
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let shape = input.shape();
        let end = (shape[0] - 1, shape[1] - 1, 0, 0);

        let path = dijkstra(
            &(0, 0, 0, 0),
            |actual| {
                adjacent(shape, actual, 1).map(|p| {
                    let risk = input[[p.0, p.1]];
                    (p, risk)
                })
            },
            |p| *p == end,
        );

        match path {
            Some((_, risk)) => Ok(risk),
            None => Err("path not found".to_string()),
        }
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let shape = input.shape();
        let end = (shape[0] - 1, shape[1] - 1, 4, 4);

        let path = dijkstra(
            &(0, 0, 0, 0),
            |actual| {
                adjacent(shape, actual, 5).map(|p| {
                    let risk = (((input[[p.0, p.1]] + p.2 + p.3) - 1) % 9) + 1;
                    (p, risk)
                })
            },
            |p| *p == end,
        );

        match path {
            Some((_, risk)) => Ok(risk),
            None => Err("path not found".to_string()),
        }
    }
}

fn adjacent(
    shape: &[usize],
    position: &(usize, usize, usize, usize),
    extend: usize,
) -> impl Iterator<Item = (usize, usize, usize, usize)> {
    let (r, c, lx, ly) = position.to_owned();

    let rs = match r {
        0 => {
            if ly == 0 {
                vec![(r + 1, ly)]
            } else {
                vec![(r + 1, ly), (shape[0] - 1, ly - 1)]
            }
        }
        v if v == shape[0] - 1 => {
            if ly == extend - 1 {
                vec![(v - 1, ly)]
            } else {
                vec![(v - 1, ly), (0, ly + 1)]
            }
        }
        v => vec![(v - 1, ly), (v + 1, ly)],
    };
    let cs = match c {
        0 => {
            if lx == 0 {
                vec![(c + 1, lx)]
            } else {
                vec![(c + 1, lx), (shape[1] - 1, lx - 1)]
            }
        }
        v if v == shape[1] - 1 => {
            if lx == extend - 1 {
                vec![(v - 1, lx)]
            } else {
                vec![(v - 1, lx), (0, lx + 1)]
            }
        }
        v => vec![(v - 1, lx), (v + 1, lx)],
    };

    rs.into_iter()
        .map(move |(r, ly)| (r, c, lx, ly))
        .chain(cs.into_iter().map(move |(c, lx)| (r, c, lx, ly)))
}
