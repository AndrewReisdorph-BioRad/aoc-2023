use std::{path::PathBuf, borrow::BorrowMut};

use clap::Parser;

use crate::utils::{AsciiReader, slurp_bytes};

use super::{CommandImpl, DynError};

struct History(Vec<i64>);

impl History {
    pub fn new(values: Vec<i64>) -> Self {
        Self(values)
    }

    pub fn next_value(self) -> i64 {
        let mut rows: Vec<Vec<i64>> = vec![self.0];
        let mut last_row_idx = 0;
        
        let mut next_row_value = loop {
            let mut next_row: Vec<i64> = vec![];
            rows[last_row_idx].iter().enumerate().skip(1).for_each(|(i, v)| {
                next_row.push(v - rows[last_row_idx][i - 1]);
            });
            let homogenous = next_row.iter().all(|v| *v == next_row[0]);

            if homogenous {
                break next_row[0];
            }

            rows.push(next_row);

            last_row_idx += 1;
        };

        for row in rows.iter_mut().rev() {
            next_row_value = row.first().unwrap() - next_row_value;
        }

        next_row_value
    }
}

struct HistoryReader {
    reader: AsciiReader
}

impl HistoryReader {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self {
            reader: AsciiReader::new(buffer)
        }
    }
}

impl Iterator for HistoryReader {
    type Item = History;

    fn next(&mut self) -> Option<Self::Item> {
        let mut history = Vec::new();

        while let Some(value) = self.reader.read_next_number() {
            history.push(value);
        }

        // Move past new line
        self.reader.skip(1);

        if history.is_empty() {
            return None;
        }

        Some(History::new(history))
    }
}

struct Solver {
    reader: HistoryReader,
}

impl Solver {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self {
            reader: HistoryReader::new(buffer)
        }
    }

    pub fn solve(self) -> i64 {
        self.reader
            .map(|h| h.next_value())
            .sum()
    }
}

#[derive(Parser, Debug)]
pub struct Day9b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day9b {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day9b: {answer}");
        Ok(())
    }
}
