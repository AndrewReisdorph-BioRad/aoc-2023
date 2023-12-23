use std::{path::PathBuf, fmt::{self, Display, Formatter}};

use clap::Parser;

use crate::utils::{AsciiReader, slurp_bytes};

use super::{CommandImpl, DynError};

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Tile {
    Empty,
    RollingRock,
    FixedRock
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::FixedRock => write!(f, "#"),
            Tile::RollingRock => write!(f, "O"),
        }
    }
}


#[derive(PartialEq, Hash, Eq, Clone)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
    height: usize,
    width: usize
}

impl Board {
    pub fn read(reader: &mut AsciiReader) -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        let mut width = 0;
        let mut height = 0;
        
        loop {
            let next_row = reader.read_until(b'\n');
            if next_row.is_none() {
                break;
            }
            height += 1;
            let next_row = next_row.unwrap();
            if next_row.is_empty() || next_row[0] == b'\n' {
                if tiles.is_empty() {
                    reader.skip(1);
                    continue;
                } else {
                    break;
                }
            }

            tiles.push(next_row.iter().map(|&b| match b {
                b'.' => Tile::Empty,
                b'#' => Tile::FixedRock,
                b'O' => Tile::RollingRock,
                _ => panic!("invalid tile"),
            }).collect());

            reader.skip(1);
        }

        width = tiles[0].len();

        Self {
            tiles,
            width,
            height
        }
    }

    pub fn slide(&mut self, direction: Direction) {
        match direction {
            Direction::North => {
                let mut rock_moved = true;
                while rock_moved {
                    rock_moved = false;
                    for col in 0..self.width {
                        for row in 1..self.height {
                            let current_tile = &self.tiles[row][col];
                            let tile_above = &self.tiles[row - 1][col];

                            if current_tile == &Tile::RollingRock && tile_above == &Tile::Empty {
                                rock_moved = true;
                                self.tiles[row - 1][col] = Tile::RollingRock;
                                self.tiles[row][col] = Tile::Empty;
                            }
                        }           
                    }
                }
            },
            Direction::East => {
                let mut rock_moved = true;
                while rock_moved {
                    rock_moved = false;
                    for row in 0..self.height {
                        for col in (0..self.width-1) {
                            let current_tile = &self.tiles[row][col];
                            let east_tile = &self.tiles[row][col + 1];

                            if current_tile == &Tile::RollingRock && east_tile == &Tile::Empty {
                                rock_moved = true;
                                self.tiles[row][col + 1] = Tile::RollingRock;
                                self.tiles[row][col] = Tile::Empty;
                            }
                        }           
                    }
                }
            },
            Direction::South => {
                let mut rock_moved = true;
                while rock_moved {
                    rock_moved = false;
                    for col in 0..self.width {
                        for row in (0..self.height-1).rev() {
                            let current_tile = &self.tiles[row][col];
                            let tile_below = &self.tiles[row + 1][col];

                            if current_tile == &Tile::RollingRock && tile_below == &Tile::Empty {
                                rock_moved = true;
                                self.tiles[row + 1][col] = Tile::RollingRock;
                                self.tiles[row][col] = Tile::Empty;
                            }
                        }           
                    }
                }
            },
            Direction::West => {
                let mut rock_moved = true;
                while rock_moved {
                    rock_moved = false;
                    for row in 0..self.height {
                        for col in (1..self.width).rev() {
                            let current_tile = &self.tiles[row][col];
                            let east_tile = &self.tiles[row][col - 1];

                            if current_tile == &Tile::RollingRock && east_tile == &Tile::Empty {
                                rock_moved = true;
                                self.tiles[row][col - 1] = Tile::RollingRock;
                                self.tiles[row][col] = Tile::Empty;
                            }
                        }           
                    }
                }
            },
        }
    }

    pub fn cycle(&mut self) {
        self.slide(Direction::North);
        self.slide(Direction::West);
        self.slide(Direction::South);
        self.slide(Direction::East);
    }

    pub fn calculate_load(&self) -> u64 {
        let mut load: u64 = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                if self.tiles[row][col] == Tile::RollingRock {
                    load += self.height as u64 - row as u64;
                }
            }
        }
        load
    }
}

impl Display for Board {
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
        board.slide(Direction::North);
        
        board.calculate_load()
    }
}

#[derive(Parser, Debug)]
pub struct Day14a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day14a {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day 14 A: {answer}");
        Ok(())
    }
}
