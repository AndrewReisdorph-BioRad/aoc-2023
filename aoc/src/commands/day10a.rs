use std::path::PathBuf;

use clap::Parser;

use crate::utils::{AsciiReader, slurp_bytes};

use super::{CommandImpl, DynError};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Solver {
    reader: AsciiReader,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position(Option<usize>);

impl Position {
    fn step(&mut self, direction: Direction, width: usize, max_position: usize) {
        if self.0.is_none() {
            panic!("Cannot step from None position");
        }

        let position = self.0.unwrap();

        self.0 = match direction {
            Direction::North => {
                if position < width {
                    None
                } else {
                    Some(position - width)
                }
            },
            Direction::East => {
                if (position + 1) % width == 0 {
                    None
                } else {
                    Some(position + 1)
                }
            },
            Direction::South => {
                if position + width > max_position {
                    None
                } else {
                    Some(position + width)
                }
            },
            Direction::West => {
                if position % width == 0 {
                    None
                } else {
                    Some(position - 1)
                }
            },
        }
    }
}

impl Solver {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self {
            reader: AsciiReader::new(buffer),
        }
    }

    pub fn get_maze_width(&mut self) -> u32 {
        self.reader.seek(0);

        // Add one here to include the newline as part of the maze tiles
        let width = self.reader.read_until(b'\n').unwrap().len() + 1;

        self.reader.seek(0);

        width as u32
    }

    fn find_loop_length(&mut self, start_position: u32, width: u32, direction: Direction) -> Option<u32> {
        // println!("find_loop_length: direction: {:?}", direction);
        let mut loop_length = 0;
        let max_position = self.reader.len() - 1;
        let mut position = Position(Some(start_position as usize));
        position.step(direction, width as usize, max_position);

        let mut last_direction = direction;

        loop {
            loop_length += 1;
            let tile = self.reader.at(position.0?);
            // println!("tile: {} direction: {:?}", tile as char, last_direction);

            last_direction = match tile {
                // Dead end
                b'\n' | b'.' => {
                    return None;
                },
                b'J' => match last_direction {
                    Direction::East => {
                        Direction::North
                    },
                    Direction::South => {
                        Direction::West
                    },
                    _ => {
                        return None;
                    }
                },
                b'F' => match last_direction {
                    Direction::North => {
                        Direction::East
                    },
                    Direction::West => {
                        Direction::South
                    },
                    _ => {
                        return None;
                    }
                },
                b'7' => match last_direction {
                    Direction::North => {
                        Direction::West
                    },
                    Direction::East => {
                        Direction::South
                    },
                    _ => {
                        return None;
                    }
                },
                b'L' => match last_direction {
                    Direction::South => {
                        Direction::East
                    },
                    Direction::West => {
                        Direction::North
                    },
                    _ => {
                        return None;
                    }
                },
                b'|' => match last_direction {
                    Direction::North | Direction::South => {
                        last_direction
                    },
                    _ => {
                        return None;
                    }
                },
                b'-' => match last_direction {
                    Direction::East | Direction::West => {
                        last_direction
                    },
                    _ => {
                        return None;
                    }
                },
                _ => panic!("Unexpected tile: {}", tile)
            };

            position.step(last_direction, width as usize, max_position);

            if position.0? == start_position as usize {
                break;
            }
        }

        // println!("loop_length: {}", loop_length);

        Some(loop_length)
    }

    pub fn solve(&mut self) -> u32 {
        let width = self.get_maze_width();

        let start_position = self.reader.read_until(b'S').unwrap().len() as u32;

        let max_loop_length = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West
        ].iter()
            .filter_map(|d| self.find_loop_length(start_position, width, *d))
            .max()
            .unwrap();

        // println!("width: {}", width);
        // println!("start_position: {}", start_position);
        println!("max_loop_length: {}", max_loop_length);

        (max_loop_length + 1) / 2
    }
}

#[derive(Parser, Debug)]
pub struct Day10a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day10a {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day10a: {answer}");
        Ok(())
    }
}
