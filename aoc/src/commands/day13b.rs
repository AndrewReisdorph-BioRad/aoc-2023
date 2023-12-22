use std::{path::PathBuf, fmt::{Display, Formatter, self}};

use clap::Parser;

use crate::utils::{AsciiReader, slurp_bytes};

use super::{CommandImpl, DynError};

struct Solver {
    reader: AsciiReader,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Ash,
    Rock
}

#[derive(Debug, PartialEq)]
enum SymmetryType {
    Approximate(usize),
    Exact(usize),
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Ash => write!(f, "."),
            Tile::Rock => write!(f, "#"),
        }
    }
}

struct Pattern {
    tiles: Vec<Vec<Tile>>,
    height: usize,
    width: usize
}

impl Pattern {
    fn new() -> Self {
        Self {
            tiles: Vec::new(),
            height: 0,
            width: 0
        }
    }

    fn add_row(&mut self, row: Vec<Tile>) {
        if self.tiles.is_empty() {
            self.width = row.len();
        } else if row.len() != self.width {
            panic!("invalid row length");
        }
        self.tiles.push(row);
        self.height += 1;
    }

    pub fn symmetry_column(&self) -> Option<SymmetryType> {
        for column in 0..(self.width-1) {
            if let Some(SymmetryType::Approximate(symmetry)) = self.check_vertical_symmetry(column) {
                return Some(SymmetryType::Approximate(symmetry));
            }
        }
        None
        // (0..(self.width-1)).find(|&x| self.check_vertical_symmetry(x).is_some())
    }

    pub fn symmetry_row(&self) -> Option<SymmetryType> {
        for row in 0..(self.height-1) {
            if let Some(SymmetryType::Approximate(symmetry)) = self.check_horizontal_symmetry(row) {
                return Some(SymmetryType::Approximate(symmetry));
            }
        }
        None
        // (0..(self.height-1)).find(|&y| self.check_horizontal_symmetry(y).is_some())
    }

    fn check_vertical_symmetry(&self, column: usize) -> Option<SymmetryType> {
        let mut left = column;
        let mut right = column + 1;
        let mut differences = 0;

        loop {
            for y in 0..self.height {
                if self.tiles[y][left] != self.tiles[y][right] {
                    if differences >= 1 {
                        return None;
                    }
                    differences += 1;
                }
            }
            if left == 0 || right == self.width - 1 {
                if differences == 0 {
                    return Some(SymmetryType::Exact(column));
                } else {
                    return Some(SymmetryType::Approximate(column));
                }
            }
            left -= 1;
            right += 1;
        }
    }

    fn check_horizontal_symmetry(&self, row: usize) -> Option<SymmetryType> {
        let mut top = row;
        let mut bottom = row + 1;
        let mut differences = 0;

        loop {
            for x in 0..self.width {
                if self.tiles[top][x] != self.tiles[bottom][x] {
                    if differences >= 1 {
                        return None;
                    }
                    differences += 1;
                }
            }
            if top == 0 || bottom == self.height - 1 {
                if differences == 0 {
                    return Some(SymmetryType::Exact(row));
                } else {
                    return Some(SymmetryType::Approximate(row));
                }
            }
            top -= 1;
            bottom += 1;
        }
    }
}

impl Display for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Solver {
    fn new(input: Vec<u8>) -> Self {
        Self {
            reader: AsciiReader::new(input),
        }
    }

    fn next_pattern(&mut self) -> Option<Pattern> {
        let mut pattern = Pattern::new();

        loop {
            let next_row = self.reader.read_until(b'\n');
            if next_row.is_none() {
                break;
            }
            let next_row = next_row.unwrap();
            if next_row.is_empty() || next_row[0] == b'\n' {
                if pattern.tiles.is_empty() {
                    self.reader.skip(1);
                    continue;
                } else {
                    break;
                }
            }

            pattern.add_row(next_row.iter().map(|&b| match b {
                b'.' => Tile::Ash,
                b'#' => Tile::Rock,
                _ => panic!("invalid tile"),
            }).collect());

            self.reader.skip(1);
        }

        if pattern.tiles.is_empty() {
            return None;
        }
        
        Some(pattern)
    }

    fn solve(&mut self) -> u64 {
        let mut sum = 0_u64;

        while let Some(pattern) = self.next_pattern() {
            if let Some(SymmetryType::Approximate(symmetry_column)) = pattern.symmetry_column() {
                sum += (symmetry_column + 1) as u64;
            } else if let Some(SymmetryType::Approximate(symmetry_row)) = pattern.symmetry_row() {
                sum += ((symmetry_row + 1) * 100) as u64;
            } else {
                panic!("no symmetry found");
            }
        }
        
        sum
    }
}

#[derive(Parser, Debug)]
pub struct Day13b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day13b {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day 13 B: {answer}");
        Ok(())
    }
}
