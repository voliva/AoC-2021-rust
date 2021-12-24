use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Instruction>;
    type Output1 = isize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .filter(|l| l.len() > 0)
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut registers: [isize; 4] = [0; 4];

        // let r = [9, 9, 9, 1, 1, 9, 9, 3, 9, 4, 9, 6, 8, 4];
        //                 0   1  2  3   4  5  6  7  8 9  10 11  12 13
        //                 12,10,13,-11,13,-1,10,11,0,10,-5,-16,-7,-11
        let r = [6, 2, 9, 1, 1, 9, 4, 1, 7, 1, 6, 1, 1, 1];
        // let r = [1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        // let r = [1, 1, 9, 1, 1, 9, 9, 3, 9, 1, 6, 6, 1, -3];
        let mut input_value = r.iter();
        let mut c = -1;
        for i in input {
            match i.clone() {
                Instruction::Inp(reg) => {
                    println!(
                        "Input {} {} {} {}",
                        c,
                        registers[3],
                        str_rep(registers[3]),
                        registers[3] % 26
                    );
                    c += 1;
                    registers[r_to_i(reg)] = *input_value.next().unwrap();
                }
                Instruction::Add(reg, val) => registers[r_to_i(reg)] += get_val(val, &registers),
                Instruction::Mul(reg, val) => registers[r_to_i(reg)] *= get_val(val, &registers),
                Instruction::Div(reg, val) => registers[r_to_i(reg)] /= get_val(val, &registers),
                Instruction::Mod(reg, val) => {
                    let val = get_val(val, &registers);
                    if registers[r_to_i(reg)] < 0 || val <= 0 {
                        panic!("Invalid modulo {}, {}", registers[r_to_i(reg)], val);
                    }
                    registers[r_to_i(reg)] %= val;
                }
                Instruction::Eql(reg, val) => {
                    registers[r_to_i(reg)] = if registers[r_to_i(reg)] == get_val(val, &registers) {
                        1
                    } else {
                        0
                    }
                }
            }
        }
        println!(
            "Input {} {} {} {}",
            c,
            registers[3],
            str_rep(registers[3]),
            registers[3] % 26
        );

        Ok(registers[3])
    }

    fn solve_second(&self, _: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}

fn str_rep(val: isize) -> String {
    if val == 0 {
        return "".to_string();
    }
    let base = u32::from('a');
    format!(
        "{}{}",
        str_rep(val / 26),
        char::from_u32(base + (val as u32) % 26).unwrap()
    )
}

fn r_to_i(reg: char) -> usize {
    reg.to_digit(36).unwrap() as usize - 'w'.to_digit(36).unwrap() as usize
}

fn get_val(val: Value, regs: &[isize; 4]) -> isize {
    match val {
        Value::Reg(reg) => regs[r_to_i(reg)],
        Value::Imm(val) => val,
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Reg(char),
    Imm(isize),
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regs = ["w", "x", "y", "z"];
        Ok(if regs.contains(&s) {
            Value::Reg(s.chars().next().unwrap())
        } else {
            Value::Imm(s.parse().unwrap())
        })
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Inp(char),
    Add(char, Value),
    Mul(char, Value),
    Div(char, Value),
    Mod(char, Value),
    Eql(char, Value),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect_vec();
        let r = match split[0] {
            "inp" => Instruction::Inp(split[1].parse().unwrap()),
            "add" => Instruction::Add(split[1].parse().unwrap(), split[2].parse().unwrap()),
            "mul" => Instruction::Mul(split[1].parse().unwrap(), split[2].parse().unwrap()),
            "div" => Instruction::Div(split[1].parse().unwrap(), split[2].parse().unwrap()),
            "mod" => Instruction::Mod(split[1].parse().unwrap(), split[2].parse().unwrap()),
            "eql" => Instruction::Eql(split[1].parse().unwrap(), split[2].parse().unwrap()),
            _ => unreachable!(),
        };
        Ok(r)
    }
}
