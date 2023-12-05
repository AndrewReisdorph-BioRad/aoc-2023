use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_bytes;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day5a {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug)]
pub struct MapRange {
    pub destination_range_start: u32,
    pub source_range_start: u32,
    pub range_length: u32,
}

impl MapRange {
    fn map(&self, seed: u32) -> Option<u32> {
        if seed < self.source_range_start || seed >= self.source_range_start + self.range_length {
            return None;
        }

        Some(self.destination_range_start + (seed - self.source_range_start))
    }
}

pub struct Solver5A<'a> {
    bytes: &'a [u8],
    read_idx: usize,
}

impl<'a> Solver5A<'a> {

    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            read_idx: 0,
        }
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

    fn get_seeds(&mut self) -> Vec<u32> {
        // Skip past "seeds:" text
        self.read_idx = 6;
        let mut seeds = Vec::new();
        while let Some(seed) = self.read_next_number() {
            seeds.push(seed);
        }
        seeds
    }

    fn move_to_next_line(&mut self) {
        while self.bytes[self.read_idx] != b'\n' {
            self.read_idx += 1;
        }
        self.read_idx += 1;
    }

    fn get_next_map_range(&mut self) -> Option<MapRange> {
        let map_range = MapRange {
            destination_range_start: self.read_next_number()?,
            source_range_start: self.read_next_number().unwrap(),
            range_length: self.read_next_number().unwrap(),
        };

        self.move_to_next_line();

        Some(map_range)
    }

    fn solve(&mut self) -> u32 {
        // Get seeds
        let mut seeds: Vec<u32> = self.get_seeds();
        let mut mapped_seeds: Vec<u32> = Vec::new();
        // println!("seeds({}): {:?}", seeds.len(), seeds);
        while self.read_idx < self.bytes.len() - 1 {
            // Move read idx to beginning of the next map
            self.read_idx += 1;
            self.move_to_next_line();
            self.move_to_next_line();
    
            // Map seeds according to map
            while let Some(map_range) = self.get_next_map_range() {
                // dbg!(map_range);

                seeds.retain(|seed| {
                    let mapped = map_range.map(*seed);
                    if let Some(mapped) = mapped {
                        mapped_seeds.push(mapped);
                        return false;
                    }
                    true
                });

                // if seeds.is_empty() {
                //     break;
                // }
            }

            // Move all values back into seeds to be mapped again
            seeds.append(&mut mapped_seeds);

            // println!("-----------------------------------");
        }

        // println!("seeds({}): {:?}", seeds.len(), seeds);

        *seeds.iter().min().unwrap()
    }
}

impl CommandImpl for Day5a {
    fn main(&mut self) -> Result<(), DynError> {
        let bytes = slurp_bytes(self.input.as_path()).unwrap();
        let answer = Solver5A::new(&bytes[..]).solve();
        println!("5A: {answer}");

        // 224439347 is too low
        // 289863851
        Ok(())
    }
}
