use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_bytes;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day4 {
    #[clap(long, short)]
    input: PathBuf,
}

pub struct CardScorer<'a> {
    bytes: &'a [u8],
    read_idx: usize,
}

impl<'a> CardScorer<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            read_idx: 0,
        }
    }

    fn read_next_number(&mut self) -> Option<u32> {
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
}

impl<'a> Iterator for CardScorer<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.read_idx >= self.bytes.len() - 1 {
            return None;
        }

        // Find beginning of numbers
        // Skip past "Card " text
        self.read_idx += 4;
        while self.bytes[self.read_idx] != b':' {
            self.read_idx += 1;
        }
        // Skip white space after colon
        self.read_idx += 2;

        let mut map: [u8; 100] = [0; 100];

        while let Some(value) = self.read_next_number() {
            map[value as usize] = 1;
            if self.bytes[self.read_idx] == b'|' {
                break;
            }
        }
        // Skip past pipe and whitespace
        self.read_idx += 2;

        let mut points: u32 = 0;
        while let Some(value) = self.read_next_number() {
            points += map[value as usize] as u32;

            if self.bytes[self.read_idx] == b'\n' {
                break;
            }
        }

        Some(points)
    }
}

impl Day4 {
    pub fn part_one(bytes: &[u8]) {
        let mut read_idx: usize = 0;
        let mut sum = 0;
        while let Some(sore) = Self::get_next_card_score(bytes, &mut read_idx) {
            sum += sore;
        }
        println!("Part 1: {}", sum);
        // 20117
    }

    pub fn part_two(bytes: &[u8]) {
        let mut read_idx: usize = 0;
        let mut card_count = 0;
        let mut card_copy_map = [1; 10000];
        for (idx, score) in CardScorer::new(bytes).enumerate() {
            for map_idx in (idx+1)..(1+idx+score as usize) {
                if map_idx > card_copy_map.len() {
                    break;
                }
                card_copy_map[map_idx] += card_copy_map[idx];
            }
            card_count += card_copy_map[idx];
        }
        //13768818
        println!("Part 2: {}", card_count);
    }

    fn read_next_number(bytes: &[u8], read_idx: &mut usize) -> Option<u32> {
        while bytes[*read_idx] == b' ' {
            *read_idx += 1;
        }

        let mut num = 0;
        while bytes[*read_idx].is_ascii_digit() {
            num *= 10;
            num += (bytes[*read_idx] - b'0') as u32;
            *read_idx += 1;
        }

        while bytes[*read_idx] == b' ' {
            *read_idx += 1;
        }
        // *read_idx += 1;
        Some(num)
    }

    fn get_next_card_score(bytes: &[u8], read_idx: &mut usize) -> Option<u32> {
        if *read_idx >= bytes.len() - 1 {
            return None;
        }

        // Find beginning of numbers
        // Skip past "Card " text
        *read_idx += 4;
        while bytes[*read_idx] != b':' {
            *read_idx += 1;
        }
        // Skip white space after colon
        *read_idx += 2;

        let mut map: [u8; 100] = [0; 100];

        while let Some(value) = Self::read_next_number(bytes, read_idx) {
            map[value as usize] = 1;
            if bytes[*read_idx] == b'|' {
                break;
            }
        }
        // Skip past pipe and whitespace
        *read_idx += 2;

        let mut points: u32 = 0;
        while let Some(value) = Self::read_next_number(bytes, read_idx) {
            if map[value as usize] == 1 {
                if points > 0 {
                    points *= 2;
                } else {
                    points = 1;
                }
            }

            if bytes[*read_idx] == b'\n' {
                break;
            }
        }

        Some(points)
    }


}

impl CommandImpl for Day4 {
    fn main(&mut self) -> Result<(), DynError> {
        let bytes = slurp_bytes(self.input.as_path()).unwrap();
        Self::part_two(&bytes);
        Ok(())
    }
}
