use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_bytes;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day6a {
    #[clap(long, short)]
    input: PathBuf,
}

pub struct Solver {
    bytes: Vec<u8>,
    read_idx: usize,
}

#[derive(Debug)]
pub struct Race {
    pub time: u32,
    pub distance: u32,
}

impl Race {
    pub fn ways_to_win(&self) -> u32 {
        let mut ways_to_win = 0;

        for hold_time in 1..=self.time {
            let distance_covered = hold_time * (self.time - hold_time);
            if distance_covered > self.distance {
                ways_to_win += 1;
            } else if ways_to_win > 1 {
                break;
            }
        }

        ways_to_win
    }
}

impl Solver {
    fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            read_idx: 0,
        }
    }

    pub fn read_races(&mut self) -> Vec<Race> {
        let mut times: Vec<u32> = Vec::new();
        let mut distances: Vec<u32> = Vec::new();

        // Skip past "Time:" text
        self.read_idx = 5;
        while let Some(time) = self.read_next_number() {
            times.push(time);
        }

        // Skip past "Distance:" text
        self.read_idx += 10;
        while let Some(distance) = self.read_next_number() {
            distances.push(distance);
        }

        times
            .iter()
            .zip(distances.iter())
            .map(|f| Race { time: *f.0, distance: *f.1 })
            .collect()
    }

    fn read_next_number(&mut self) -> Option<u32> {
        if self.read_idx >= self.bytes.len() || self.bytes[self.read_idx] == b'\n' {
            return None;
        }

        while self.bytes[self.read_idx] == b' ' {
            self.read_idx += 1;
        }

        let mut num = 0;
        while self.bytes[self.read_idx].is_ascii_digit() {
            num *= 10;
            num += (self.bytes[self.read_idx] - b'0') as u32;
            self.read_idx += 1;
        }

        while self.bytes[self.read_idx] == b' ' {
            self.read_idx += 1;
        }
        // *read_idx += 1;
        Some(num)
    }

    pub fn solve(&mut self) -> u32 {
        self.read_races()
            .iter()
            .map(|r| r.ways_to_win())
            .product::<u32>()
    }
}

impl CommandImpl for Day6a {
    fn main(&mut self) -> Result<(), DynError> {
        let bytes = slurp_bytes(self.input.as_path()).unwrap();
        let answer = Solver::new(bytes).solve();
        println!("6A: {answer}");

        Ok(())
    }
}
