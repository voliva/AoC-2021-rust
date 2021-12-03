use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

macro_rules! printResult {
    ($part:expr, $result:expr, $start:expr ) => {
        match $result {
            Ok(res) => println!(
                "Solution to part {}: {} ({})",
                $part,
                res,
                get_elapsed($start)
            ),
            Err(val) => println!("Solution to part {} errored: {}", $part, val),
        }
    };
}

pub trait Solver {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input;
    fn solve_first(&self, input: &mut Self::Input) -> Result<Self::Output1, String>;
    fn solve_second(&self, input: &mut Self::Input) -> Result<Self::Output2, String>;

    fn solve(&self, filename: String, parts: isize) {
        let file = File::open(filename).expect("input file not found");
        let mut input = self.read_input(BufReader::new(&file));
        if parts & 0x1 > 0 {
            let start = Instant::now();
            printResult!(1, self.solve_first(&mut input), start);
        }
        if parts & 0x2 > 0 {
            let start = Instant::now();
            printResult!(2, self.solve_second(&mut input), start)
        }
    }
}

fn get_elapsed(start: Instant) -> String {
    let elapsed = start.elapsed();

    let nanos = elapsed.as_nanos();
    let decimals = format!("{}", nanos).len();
    match decimals {
        0..=3 => format!("{} ns", elapsed.as_nanos()),
        4..=6 => format!("{} μs", elapsed.as_micros()),
        7..=9 => format!("{} ms", elapsed.as_millis()),
        _ => format!("{} s", elapsed.as_secs()),
    }
}
