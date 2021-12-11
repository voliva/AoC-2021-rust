use ndarray::Array2;

use super::Solver;
use itertools::iproduct;
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
        let mut field = input.clone();
        let mut total_flashes: usize = 0;

        let shape = input.shape(); // Can't use field bc borrow check?

        for _ in 0..100 {
            for (r, c) in iproduct!(0..shape[0], 0..shape[1]) {
                increase_cell(&mut field, shape, (r, c))
            }

            field.iter_mut().for_each(|v| {
                if *v == 10 {
                    *v = 0;
                    total_flashes += 1;
                }
            });

            // println!("{}", i);
            // print_field(&field);
            // println!("");
        }

        Ok(total_flashes)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut field = input.clone();
        let mut total_flashes: usize = 0;

        let shape = input.shape();

        for i in 1.. {
            for (r, c) in iproduct!(0..shape[0], 0..shape[1]) {
                increase_cell(&mut field, shape, (r, c))
            }

            field.iter_mut().for_each(|v| {
                if *v == 10 {
                    *v = 0;
                    total_flashes += 1;
                }
            });

            // println!("{}", i);
            // print_field(&field);
            // println!("");

            if field.iter().all(|v| *v == 0) {
                return Ok(i);
            }
        }

        unreachable!()
    }
}

fn increase_cell(field: &mut Array2<u8>, shape: &[usize], (r, c): (usize, usize)) {
    if field[[r, c]] == 10 {
        return;
    }
    field[[r, c]] += 1;

    if field[[r, c]] == 10 {
        for pos in adjacent(shape, (r, c)) {
            increase_cell(field, shape, pos);
        }
    }
}

fn adjacent(shape: &[usize], (r, c): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let rs = match r {
        0 => vec![r, r + 1],
        v if v == shape[0] - 1 => vec![v - 1, v],
        v => vec![v - 1, v, v + 1],
    };
    let cs = match c {
        0 => vec![c, c + 1],
        v if v == shape[1] - 1 => vec![v - 1, c],
        v => vec![v - 1, c, v + 1],
    };

    itertools::iproduct!(rs.into_iter(), cs.into_iter()).filter(move |p| !(p.0 == r && p.1 == c))
}

// fn print_field(arr: &Array2<u8>) {
//     for r in arr.rows() {
//         for c in r.iter() {
//             if *c == 0 {
//                 print!("0");
//             } else {
//                 print!(".");
//             }
//         }
//         println!("");
//     }
// }
