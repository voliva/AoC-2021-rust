use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;

macro_rules! printResult {
    ($part:expr, $result:expr ) => {
        match $result {
            Ok(res) => println!("Solution to part {}: {}", $part, res),
            Err(val) => println!("Solution to part {} errored: {}", $part, val),
        }
    };
}

pub trait Solver {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input;
    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String>;
    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String>;

    fn solve(&self, filename: String, parts: u8) {
        let file = File::open(filename).expect("input file not found");
        let input = self.read_input(BufReader::new(&file));
        if parts & 0x1 > 0 {
            printResult!(1, self.solve_first(&input))
        }
        if parts & 0x2 > 0 {
            printResult!(2, self.solve_second(&input))
        }
    }
}
