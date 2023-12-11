use std::path::PathBuf;

use clap::Parser;

use crate::utils::{slurp_bytes, AsciiReader};

use super::{CommandImpl, DynError};

struct Solver {
    input: Vec<u8>
}

impl Solver {
    fn new(input: Vec<u8>) -> Self {
        Self {
            input
        }
    }

    fn find_galaxies(&mut self, width: usize) -> Vec<(u32, u32)> {
        let galaxies: Vec<(u32, u32)> = (0..self.input.len()).filter_map(|read_idx| {
            match self.input[read_idx] {
                b'#' => {
                    Some(
                        ((read_idx % (width + 1)) as u32,
                        (read_idx / (width + 1)) as u32)
                    )
                },
                _ => {
                    None
                }
            }
        }).collect();

        galaxies
    }

    fn get_galaxy_distance(&mut self, galaxy_a: (u32, u32), galaxy_b: (u32, u32), empty_rows: &[u32], empty_cols: &[u32]) -> u64 {
        let mut column_distance: u64 = (galaxy_a.0).abs_diff(galaxy_b.0) as u64;
        // Count empty columns between the galaxies
        for empty_col in empty_cols {
            if *empty_col > galaxy_a.0.min(galaxy_b.0) && *empty_col < galaxy_b.0.max(galaxy_a.0) {
                column_distance += 999999;
            }
        }

        let mut row_distance: u64 = (galaxy_a.1).abs_diff(galaxy_b.1) as u64;
        // Count rows columns between the galaxies
        for empty_row in empty_rows {
            if *empty_row > galaxy_a.1.min(galaxy_b.1) && *empty_row < galaxy_b.1.max(galaxy_a.1) {
                row_distance += 999999;
            }
        }

        column_distance + row_distance
    }

    fn get_dimensions(&mut self) -> (usize, usize) {
        let mut width = 0;
        while self.input[width] != b'\n' {
            width += 1;
        }

        let height = self.input.len() / (width + 1);

        (width, height)
    }

    fn find_empty_rows_and_cols(&mut self, width: usize, height: usize, galaxies: &Vec<(u32, u32)>) -> (Vec<u32>, Vec<u32>) {
        let mut empty_row_mask: Vec<u8> = vec![1; height];
        let mut empty_col_mask: Vec<u8> = vec![1; width];

        for galaxy in galaxies {
            empty_row_mask[galaxy.0 as usize] = 0;
            empty_col_mask[galaxy.1 as usize] = 0;
        }

        let empty_rows = empty_row_mask.iter().enumerate().filter_map(|(idx, val)| {
            if *val == 1 {
                Some(idx as u32)
            } else {
                None
            }
        }).collect();
        let empty_cols = empty_col_mask.iter().enumerate().filter_map(|(idx, val)| {
            if *val == 1 {
                Some(idx as u32)
            } else {
                None
            }
        }).collect();

        (empty_rows, empty_cols)
    }

    fn solve(&mut self) -> u64 {
        let (height, width) = self.get_dimensions();
        let galaxies = self.find_galaxies(width);
        let (empty_cols, empty_rows) = self.find_empty_rows_and_cols(width, height, &galaxies);

        let mut sum = 0;
        for galaxy_a_idx in 0..galaxies.len() {
            for galaxy_b_idx in (galaxy_a_idx + 1)..galaxies.len() {
                let distance = self.get_galaxy_distance(galaxies[galaxy_a_idx], galaxies[galaxy_b_idx], &empty_rows[..], &empty_cols[..]);
                sum += distance;
            }
        }

        sum
    }
}

#[derive(Parser, Debug)]
pub struct Day11b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day11b {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day11b: {answer}");
        Ok(())
    }
}
