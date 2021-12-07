use super::Solver;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Line>;
    type Output1 = isize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let max = get_max(input);
        let mut field = ndarray::Array2::<usize>::zeros((max.x + 1, max.y + 1));

        input.into_iter().for_each(|line| {
            if line.is_straight() {
                line.run_through(|p| field[[p.x, p.y]] = field[[p.x, p.y]] + 1);
                // println!("{:?}", line);
                // line.run_through(|p| println!("{:?}", p));
            }
        });

        let result = field
            .iter()
            .fold(0, |total, v| if *v >= 2 { total + 1 } else { total });

        Ok(result)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let max = get_max(input);
        let mut field = ndarray::Array2::<usize>::zeros((max.y + 1, max.x + 1));

        input.into_iter().for_each(|line| {
            line.run_through(|p| field[[p.y, p.x]] = field[[p.y, p.x]] + 1);
        });

        let result = field
            .iter()
            .fold(0, |total, v| if *v >= 2 { total + 1 } else { total });

        Ok(result)
    }
}

fn get_max(vec: &Vec<Line>) -> Point {
    vec.into_iter().fold(Point { x: 0, y: 0 }, |mut acc, line| {
        acc.x = std::cmp::max(acc.x, std::cmp::max(line.start.x, line.end.x));
        acc.y = std::cmp::max(acc.y, std::cmp::max(line.start.y, line.end.y));
        return acc;
    })
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref LINE_RGX: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        }

        match LINE_RGX.captures(s) {
            None => Err(format!("Error parsing line '{}'", s)),
            Some(captures) => Ok(Line {
                start: Point {
                    x: captures[1].parse().unwrap(),
                    y: captures[2].parse().unwrap(),
                },
                end: Point {
                    x: captures[3].parse().unwrap(),
                    y: captures[4].parse().unwrap(),
                },
            }),
        }
    }
}

impl Line {
    fn is_straight(&self) -> bool {
        (self.start.x == self.end.x) || (self.start.y == self.end.y)
    }
    fn run_through<F>(&self, mut f: F)
    where
        F: FnMut(&Point) -> (),
    {
        let x_grad = Gradient::new(self.start.x, self.end.x);
        let y_grad = Gradient::new(self.start.y, self.end.y);

        let mut p = Point {
            x: self.start.x,
            y: self.start.y,
        };

        if self.start.x == self.end.x {
            while p.y != y_grad.apply(self.end.y) {
                f(&p);
                p.y = y_grad.apply(p.y);
            }
        } else if self.start.y == self.end.y {
            while p.x != x_grad.apply(self.end.x) {
                f(&p);
                p.x = x_grad.apply(p.x);
            }
        } else {
            while (p.x != x_grad.apply(self.end.x)) && (p.y != y_grad.apply(self.end.y)) {
                f(&p);
                p.x = x_grad.apply(p.x);
                p.y = y_grad.apply(p.y);
            }
        }
    }
}

struct Gradient(isize);

impl Gradient {
    pub fn new(start: usize, end: usize) -> Self {
        let diff = (end as isize) - (start as isize);
        Gradient(if diff == 0 { 0 } else { diff / diff.abs() })
    }

    pub fn apply(&self, value: usize) -> usize {
        ((value as isize) + self.0) as usize
    }
}
