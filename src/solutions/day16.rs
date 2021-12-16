use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(Debug)]
enum PacketPayload {
    Literal(usize),
    Operation(Vec<Packet>),
}
#[derive(Debug)]
pub struct Packet {
    version: u8,
    id: u8,
    payload: PacketPayload,
}

impl Solver for Problem {
    type Input = Packet;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let line = file_reader.lines().next().unwrap().unwrap();
        let values = line
            .split("")
            .filter(|x| x.len() == 1)
            .map(|x| u8::from_str_radix(x, 16).unwrap())
            .collect();

        let mut reader = BinaryReader::new(values);

        parse_packet(&mut reader)
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(sum_version(input))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        Ok(calculate_value(input))
    }
}

fn sum_version(packet: &Packet) -> usize {
    let mut total = packet.version as usize;

    if let PacketPayload::Operation(subpackets) = &packet.payload {
        for subpacket in subpackets {
            total += sum_version(subpacket);
        }
    }

    return total;
}

fn calculate_value(packet: &Packet) -> usize {
    match &packet.payload {
        PacketPayload::Literal(v) => *v,
        PacketPayload::Operation(subpackets) => {
            let subvalues = subpackets.iter().map(|p| calculate_value(p));
            match packet.id {
                0 => subvalues.sum(),
                1 => subvalues.fold(1, |acc, v| acc * v),
                2 => subvalues.min().unwrap(),
                3 => subvalues.max().unwrap(),
                5 => {
                    let v = subvalues.collect_vec();
                    if v[0] > v[1] {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    let v = subvalues.collect_vec();
                    if v[0] < v[1] {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    let v = subvalues.collect_vec();
                    if v[0] == v[1] {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn parse_packet(reader: &mut BinaryReader) -> Packet {
    let version = reader.next_u8(3);
    let id = reader.next_u8(3);

    let payload = match id {
        4 => PacketPayload::Literal(parse_literal(reader)),
        _ => PacketPayload::Operation(parse_subpackets(reader)),
    };

    Packet {
        version,
        id,
        payload,
    }
}

fn parse_literal(reader: &mut BinaryReader) -> usize {
    let mut result = 0;

    loop {
        let has_more = reader.next_bit() == 1;
        let v = reader.next_u8(4);
        result = (result << 4) | (v as usize);

        if !has_more {
            break;
        }
    }

    result
}

fn parse_subpackets(reader: &mut BinaryReader) -> Vec<Packet> {
    let mut result = Vec::new();

    match reader.next_bit() {
        0 => {
            let len = reader.next_usize(15);
            let initial = reader.remaining();
            while reader.remaining() > (initial - len) {
                result.push(parse_packet(reader));
            }
        }
        1 => {
            let n = reader.next_usize(11);
            for _ in 0..n {
                result.push(parse_packet(reader));
            }
        }
        _ => unreachable!(),
    }

    result
}

struct BinaryReader {
    data: Vec<u8>,
    pointer: usize,
    bit: u8,
}
// Assumes values of u4.
impl BinaryReader {
    fn new(data: Vec<u8>) -> Self {
        BinaryReader {
            data,
            pointer: 0,
            bit: 0,
        }
    }
    fn remaining(&self) -> usize {
        (self.data.len() - self.pointer) * 4 - (self.bit as usize)
    }
    fn next_bit(&mut self) -> u8 {
        let v = self.data[self.pointer];
        let ret = (v >> (3 - self.bit)) & 0x01;
        if self.bit == 3 {
            self.bit = 0;
            self.pointer += 1;
        } else {
            self.bit += 1;
        }

        ret
    }
    fn next_u8(&mut self, n: u8) -> u8 {
        let mut v = 0;
        for _ in 0..n {
            v = (v << 1) | self.next_bit()
        }
        v
    }
    fn next_usize(&mut self, n: usize) -> usize {
        let mut v = 0;
        for _ in 0..n {
            v = (v << 1) | (self.next_bit() as usize)
        }
        v
    }
}
