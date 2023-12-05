use std::path::PathBuf;

use clap::Parser;

use crate::utils::slurp_bytes;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day5b {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug)]
pub struct MapRange {
    pub destination_range_start: u32,
    pub source_range_start: u32,
    pub range_length: u32,
}

#[derive(Debug)]
pub struct MappedRangeExtraction {
    pub mapped_range: Range,
    pub unmapped_ranges: Vec<Range>,
}

impl MapRange {
    fn map(&self, seed: u32) -> Option<u32> {
        if seed < self.source_range_start || seed >= self.source_range_start + self.range_length {
            return None;
        }

        Some(self.destination_range_start + (seed - self.source_range_start))
    }

    fn overlaps(&self, range: &Range) -> bool {
        // If range begins after end of other range
        if range.start > self.source_range_start + self.range_length {
            return false;
        }
        // If range ends before start of other range
        if range.end < self.source_range_start {
            return false;
        }
        // if range.start > self.source_range_start && range.start < self.source_range_start + self.range_length {
        //     return true;
        // }
        // if range.end > self.source_range_start && range.end < self.source_range_start + self.range_length {
        //     return true;
        // }
        true
    }

    fn extract_mapped_range(&self, range: &Range) -> Option<MappedRangeExtraction> {
        // println!("Checking range: {:?} against {} {}", range, self.source_range_start, self.source_range_start + self.range_length - 1);
        if !self.overlaps(range) {
            // println!("Does not overlap");
            return None;
        }

        let mut unmapped_ranges = Vec::new();

        let mut contained_start = range.start;
        let mut contained_end = range.end;

        if range.start < self.source_range_start {
            unmapped_ranges.push(Range { start: range.start, end: self.source_range_start - 1 });
            contained_start = self.source_range_start;
        }
        if range.end > self.source_range_start + self.range_length {
            unmapped_ranges.push(Range { start: self.source_range_start + self.range_length, end: range.end});
            contained_end = self.source_range_start + self.range_length - 1;
        }

        // println!("contained_start: {}, contained_end: {}", contained_start, contained_end);

        let mapped_range = Range {
            start: self.map(contained_start).unwrap(),
            end: self.map(contained_end).unwrap(),
        };

        // println!("mapped_range: {:?}\nunmapped_ranges: {:?}", mapped_range, unmapped_ranges);

        Some(MappedRangeExtraction { mapped_range, unmapped_ranges})
    }
}

pub struct Solver5B<'a> {
    bytes: &'a [u8],
    read_idx: usize,
}

impl<'a> Solver5B<'a> {

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

    fn get_seed_ranges(&mut self) -> Vec<Range> {
        // Skip past "seeds:" text
        self.read_idx = 6;
        let mut ranges = Vec::new();
        loop {
            let start = self.read_next_number();
            if start.is_none() {
                break;
            }
            let start = start.unwrap();
            let length = self.read_next_number().unwrap();
            ranges.push(Range { start, end: start + length - 1 });
            if self.bytes[self.read_idx] == b'\n' {
                break;
            }
        }
        ranges
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
        let mut unmapped_ranges = self.get_seed_ranges();
        self.read_idx += 1;

        // println!("seeds({}): {:?}", seeds.len(), seeds);
        while self.read_idx < self.bytes.len() - 1 {
            // Move read idx to beginning of the next map
            self.move_to_next_line();
            self.move_to_next_line();

            let mut new_unmapped_ranges = Vec::new();
            let mut new_mapped_ranges = Vec::new();

            // Map seeds according to map
            while let Some(map_range) = self.get_next_map_range() {
                // println!("**found map_range: {:?}", map_range);
                for range in unmapped_ranges.clone().iter() {
                    if let Some(mapped_range_extraction) = map_range.extract_mapped_range(range) {
                        // println!("mapped_range_extraction for {:?}: {:?}", range, mapped_range_extraction);
                        new_mapped_ranges.push(mapped_range_extraction.mapped_range);
                        new_unmapped_ranges.extend(mapped_range_extraction.unmapped_ranges);
                    } else {
                        new_unmapped_ranges.push(range.clone());
                    }
                }

                unmapped_ranges = new_unmapped_ranges.clone();
                new_unmapped_ranges.clear();
            }

            unmapped_ranges.extend(new_mapped_ranges);
            // println!("unmapped_ranges({}): {:?}", unmapped_ranges.len(), unmapped_ranges);
            // println!("-----------------------------------");
        }

        unmapped_ranges.iter().map(|range| range.start).min().unwrap()
    }
}

impl CommandImpl for Day5b {
    fn main(&mut self) -> Result<(), DynError> {
        let bytes = slurp_bytes(self.input.as_path()).unwrap();
        let answer = Solver5B::new(&bytes[..]).solve();
        println!("5B: {answer}");

        Ok(())
    }
}
