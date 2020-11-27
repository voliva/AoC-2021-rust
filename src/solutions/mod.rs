mod day01;
mod solver;

pub use solver::Solver;

pub fn solve(day: u8, parts: u8) {
    let filename = if day < 10 {
        format!("input0{}", day)
    } else {
        format!("input{}", day)
    };
    match day {
        1 => day01::Problem.solve(filename, parts),
        _ => panic!("day not implemented"),
    }
}
