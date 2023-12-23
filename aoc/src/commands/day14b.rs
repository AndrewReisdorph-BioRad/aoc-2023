use std::{path::PathBuf, collections::HashMap};

use clap::Parser;

use crate::{utils::{AsciiReader, slurp_bytes}, commands::day14a::{Board, Direction}};

use super::{CommandImpl, DynError};
struct Solver {
    reader: AsciiReader,
}

impl Solver {
    fn new(input: Vec<u8>) -> Self {
        Self {
            reader: AsciiReader::new(input),
        }
    }

    fn solve(&mut self) -> u64 {
        let mut board = Board::read(&mut self.reader);

        let mut seen: HashMap<Board, usize> = HashMap::new();
        let mut counter = 0;
        let num_cycles = 1_000_000_000;

        let (cycle_interval_start, cycle_interval) = loop {
            if seen.contains_key(&board) {
                break (seen[&board], counter - seen[&board])
            }
            seen.insert(board.clone(), counter);

            board.cycle();

            counter += 1;
        };

        let idx = (num_cycles - cycle_interval_start) % cycle_interval + cycle_interval_start;

        for (b, i) in seen {
            if i == idx {
                return b.calculate_load();
            }
        }
        
        panic!("No board found at index {}", idx);
    }
}

#[derive(Parser, Debug)]
pub struct Day14b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day14b {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day 14 B: {answer}");
        Ok(())
    }
}
