use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(Debug)]
pub struct PInput {
    sequence: Vec<usize>,
    bingo_cards: Vec<Vec<Vec<usize>>>,
}

pub struct BingoCard {
    card: Vec<Vec<usize>>,
    marks: Vec<Vec<bool>>,
    has_won: bool,
}

impl BingoCard {
    pub fn new(card: &Vec<Vec<usize>>) -> Self {
        let marks = card
            .into_iter()
            .map(|line| line.into_iter().map(|_| false).collect())
            .collect();

        BingoCard {
            card: card.clone(),
            marks,
            has_won: false,
        }
    }

    pub fn mark_num(&mut self, num: usize) -> bool {
        if self.has_won {
            return false;
        }

        for i in 0..self.card.len() {
            for j in 0..self.card[i].len() {
                self.marks[i][j] = self.marks[i][j] || (self.card[i][j] == num)
            }
        }

        self.has_won = self.has_won || self.check_win();
        self.has_won
    }

    pub fn get_value(&self) -> usize {
        let mut total = 0;

        for i in 0..self.card.len() {
            for j in 0..self.card[i].len() {
                if !self.marks[i][j] {
                    total = total + self.card[i][j];
                }
            }
        }

        total
    }

    fn check_win(&self) -> bool {
        // Check rows
        for i in 0..self.marks.len() {
            let line = &(self.marks[i]);
            if line.into_iter().all(|x| *x) {
                return true;
            }
        }

        // Check columns
        for j in 0..self.marks[0].len() {
            let mut win = true;
            for i in 0..self.marks.len() {
                if !self.marks[i][j] {
                    win = false;
                    break;
                }
            }
            if win {
                return true;
            }
        }

        false
    }
}

impl Solver for Problem {
    type Input = PInput;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines: Vec<String> = file_reader.lines().filter_map(|x| x.ok()).collect();

        let sequence: Vec<usize> = lines[0].split(",").map(|x| x.parse().unwrap()).collect();

        let mut card_lines: Vec<Vec<&String>> = Vec::new();
        let mut current = 0;
        card_lines.push(Vec::new());

        for i in 2..lines.len() {
            let line = &lines[i];
            if line == "" {
                current = current + 1;
                card_lines.push(Vec::new())
            } else {
                card_lines[current].push(line)
            }
        }

        let bingo_cards: Vec<Vec<Vec<usize>>> = card_lines
            .into_iter()
            .map(|card| {
                card.into_iter()
                    .map(|line| {
                        line.split_whitespace()
                            .map(|x| x.parse().unwrap())
                            .collect()
                    })
                    .collect()
            })
            .collect();

        PInput {
            sequence,
            bingo_cards,
        }
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut bingo_cards: Vec<_> = (&input.bingo_cards)
            .into_iter()
            .map(|card| BingoCard::new(&card))
            .collect();

        for i in 0..input.sequence.len() {
            let value = input.sequence[i];
            for c in 0..bingo_cards.len() {
                if bingo_cards[c].mark_num(value) {
                    // println!("{} {} {}", i, c, value);
                    return Ok(bingo_cards[c].get_value() * value);
                }
            }
        }

        Err("No winner".to_string())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut bingo_cards: Vec<_> = (&input.bingo_cards)
            .into_iter()
            .map(|card| BingoCard::new(&card))
            .collect();

        let mut last_win = 0;
        for i in 0..input.sequence.len() {
            let value = input.sequence[i];
            for c in 0..bingo_cards.len() {
                if bingo_cards[c].mark_num(value) {
                    last_win = bingo_cards[c].get_value() * value;
                }
            }
        }

        Ok(last_win)
    }
}
