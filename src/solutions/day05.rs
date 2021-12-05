use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        Ok(Point {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" -> ");
        Ok(Line {
            start: split.next().unwrap().parse().unwrap(),
            end: split.next().unwrap().parse().unwrap(),
        })
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
        let x_grad = (self.end.x as isize) - (self.start.x as isize);
        let x_grad_u = if x_grad == 0 {
            0
        } else {
            x_grad / x_grad.abs()
        };
        let y_grad = (self.end.y as isize) - (self.start.y as isize);
        let y_grad_u = if y_grad == 0 {
            0
        } else {
            y_grad / y_grad.abs()
        };

        let mut p = Point {
            x: self.start.x,
            y: self.start.y,
        };

        if self.start.x == self.end.x {
            while p.y != ((self.end.y as isize) + y_grad_u) as usize {
                f(&p);
                p.y = ((p.y as isize) + y_grad_u) as usize;
            }
        } else if self.start.y == self.end.y {
            while p.x != ((self.end.x as isize) + x_grad_u) as usize {
                f(&p);
                p.x = ((p.x as isize) + x_grad_u) as usize;
            }
        } else {
            while (p.x != ((self.end.x as isize) + x_grad_u) as usize)
                && (p.y != ((self.end.y as isize) + y_grad_u) as usize)
            {
                f(&p);
                p.x = ((p.x as isize) + x_grad_u) as usize;
                p.y = ((p.y as isize) + y_grad_u) as usize;
            }
        }
    }
}

impl Solver for Problem {
    type Input = Vec<Line>;
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
