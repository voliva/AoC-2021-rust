use regex::Regex;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = ((isize, isize), (isize, isize));
    type Output1 = isize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let line = file_reader.lines().map(|x| x.unwrap()).next().unwrap();

        let regex = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();

        let captures = regex.captures(&line).unwrap();

        (
            (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
            (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let (_, y) = *input;

        // => Y is independent from X
        // => Because acceleration is constant, it will go up, then down and
        // step on the same places when t is integer
        // => The velocity when it reaches the ground Vground === Velocity it went up V0 + 1
        //
        // It will skip when Vground < y.0
        // V0 = -Vground

        let max_v0 = -y.0 - 1;

        // let max_y_t = max_v0;

        // const getPosition = (speed, time) => (2*speed - (time - 1)) * time / 2
        let max_y = (2 * max_v0 - (max_v0 - 1)) * max_v0 / 2;
        Ok(max_y)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let (x, y) = *input;

        // Now we know that y can go from straight_into_it up to max_v0 to reach the point.
        // The X-values are also independent.
        // We can figure out the minimum speed to reach the spot, and the maximum
        // will be straight into it.

        /*
         * By solving the equation:
         * x = (2*v0+1)/2 * t - (t ** 2) / 2
         * v = v0 - t
         * With [V = 0], [X = x.0] for v0
         */
        let min_vx = i_sqrt(1 + 8 * x.0) / 2;
        let max_vy = -y.0 - 1;

        /*
         * The opposite end is shooting straight into it, which is x.1 for X and y.0 for Y
         */

        let candidates_x = min_vx..=x.1;
        let candidates_y = y.0..=max_vy;

        // 18..206, -108..1000 => 2576

        let mut result = 0;

        for vx in candidates_x {
            for vy in candidates_y.clone() {
                if let Some(_) = simulate((vx, vy), (x, y)) {
                    result += 1;
                }
            }
        }

        Ok(result)
    }
}

fn i_sqrt(num: isize) -> isize {
    (num as f64).sqrt() as isize
}

fn simulate(velocity: (isize, isize), (x, y): ((isize, isize), (isize, isize))) -> Option<isize> {
    let mut pos = (0, 0);
    let mut velocity = velocity.clone();
    let mut max_height = 0;

    loop {
        pos.0 += velocity.0;
        pos.1 += velocity.1;
        velocity.0 = (velocity.0 - 1).max(0);
        velocity.1 = velocity.1 - 1;

        max_height = max_height.max(pos.1);

        if x.0 <= pos.0 && pos.0 <= x.1 && y.0 <= pos.1 && pos.1 <= y.1 {
            return Some(max_height);
        }
        if pos.0 > x.1 || pos.1 < y.0 {
            return None;
        }
    }
}
