use itertools::Itertools;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(Debug, Clone)]
pub struct Image {
    lit: HashSet<(isize, isize)>,
    min: (isize, isize),
    max: (isize, isize),
    // My input has # for i=0, which means the whole infinity gets lit.
    inverted: bool,
}

impl Solver for Problem {
    // Enhancement, Lit, Max_Lit (Min_Lit = 0,0)
    type Input = (Vec<bool>, Image);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();

        let enhancement = lines[0].chars().map(|c| c == '#').collect_vec();
        let mut lit = HashSet::new();
        let max = (lines[2].len() as isize - 1, (lines.len() - 2) as isize - 1);
        for i in 2..lines.len() {
            let y = (i - 2) as isize;
            let line = &lines[i];
            lit.extend(line.char_indices().filter_map(|(x, c)| {
                if c == '#' {
                    Some((x as isize, y))
                } else {
                    None
                }
            }))
        }
        (
            enhancement,
            Image {
                lit,
                max,
                min: (0, 0),
                inverted: false,
            },
        )
    }

    fn solve_first(&self, (enhancement, image): &Self::Input) -> Result<Self::Output1, String> {
        let mut last_image = image.clone();

        // print_image(&last_image);
        for _ in 0..2 {
            last_image = enhance(enhancement, &last_image);
            // print_image(&last_image);
        }

        Ok(last_image.lit.len())
    }

    fn solve_second(&self, (enhancement, image): &Self::Input) -> Result<Self::Output2, String> {
        let mut last_image = image.clone();

        for _ in 0..50 {
            last_image = enhance(enhancement, &last_image);
        }

        Ok(last_image.lit.len())
    }
}

fn _print_image(image: &Image) {
    if image.inverted {
        println!("Inverted");
    }
    for y in image.min.1..=image.max.1 {
        for x in image.min.0..=image.max.0 {
            let coord = (x, y);
            if is_lit(image, &coord) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn enhance(enhancement: &Vec<bool>, image: &Image) -> Image {
    let inverts = enhancement[0];

    let inverted = if inverts {
        !image.inverted
    } else {
        image.inverted
    };

    let mut lit = HashSet::new();
    let mut min = (isize::MAX, isize::MAX);
    let mut max = (isize::MIN, isize::MIN);

    // println!(
    //     "{} {}",
    //     image.max.0 - image.min.0,
    //     image.max.1 - image.min.1
    // );
    for y in (image.min.1 - 1)..=(image.max.1 + 1) {
        for x in (image.min.0 - 1)..=(image.max.0 + 1) {
            let coord = (x, y);
            let index = get_index(image, &coord);
            let new_is_lit = enhancement[index];
            if new_is_lit {
                min.0 = min.0.min(x);
                min.1 = min.1.min(y);
                max.0 = max.0.max(x);
                max.1 = max.1.max(y);
            }
            if !inverted {
                if new_is_lit {
                    lit.insert(coord);
                }
            } else {
                if !new_is_lit {
                    lit.insert(coord);
                }
            }
        }
    }
    // println!("{} {}", max.0 - min.0, max.1 - min.1);

    Image {
        lit,
        min,
        max,
        inverted,
    }
}

fn get_index(image: &Image, (x, y): &(isize, isize)) -> usize {
    let bit_coord = vec![
        (x - 1, y - 1),
        (*x, y - 1),
        (x + 1, y - 1),
        (x - 1, *y),
        (*x, *y),
        (x + 1, *y),
        (x - 1, y + 1),
        (*x, y + 1),
        (x + 1, y + 1),
    ];

    bit_coord
        .into_iter()
        .map(|coord| is_lit(image, &coord))
        .fold(0, |acc, lit| {
            let mut result = acc << 1;
            if lit {
                result = result | 0x01;
            }
            result
        })
}

fn is_lit(image: &Image, coord: &(isize, isize)) -> bool {
    let is_lit = image.lit.contains(coord);
    if image.inverted {
        !is_lit
    } else {
        is_lit
    }
}
