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
    let rows = shape[0];
    let cols = shape[1];
    let (r, c, lx, ly) = position;
    let y = *ly * rows + *r;
    let x = *lx * cols + *c;
    let max_y = rows * extend;
    let max_x = cols * extend;

    [
        (Some(x), y.checked_sub(1)),
        (x.checked_sub(1), Some(y)),
        (Some(x + 1), Some(y)),
        (Some(x), Some(y + 1)),
    ]
    .into_iter()
    .filter_map(move |v| match v {
        (Some(x), Some(y)) if x < max_x && y < max_y => Some((x, y)),
        _ => None,
    })
    .map(move |(x, y)| {
        let ly = y / rows;
        let r = y % rows;
        let lx = x / cols;
        let c = x % cols;
        (r, c, lx, ly)
    })
}
