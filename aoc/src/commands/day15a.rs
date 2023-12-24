use std::{path::PathBuf, collections::HashMap};

use clap::Parser;

use crate::utils::{AsciiReader, slurp_bytes};

use super::{CommandImpl, DynError};
struct Solver {
    reader: AsciiReader,
}

pub fn hash(str: &[u8]) -> u8 {
    let mut hash: u64 = 0;

    for c in str {
        hash += *c as u64;
        hash *= 17;
        hash %= 256;
    }

    hash as u8
}


impl Solver {
    fn new(input: Vec<u8>) -> Self {
        Self {
            reader: AsciiReader::new(input),
        }
    }

    fn next_step(&mut self) -> Option<&[u8]> {
        let data = self.reader.read_to(b',');
        if data?.is_empty() {
            None
        } else {
            data
        }
    }

    fn solve(&mut self) -> u64 {
        let mut sum = 0;
        loop {
            let data = self.next_step();
            if data.is_none() {
                break;
            }
            let data = data.unwrap();
            let hash = hash(data);
            sum += hash as u64;
        }

        sum
    }
}

#[derive(Parser, Debug)]
pub struct Day15a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day15a {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day 15 A: {answer}");
        Ok(())
    }
}
