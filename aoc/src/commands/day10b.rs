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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Visit {
    Unvisited,
    Visited {
        north: bool,
        east: bool,
        south: bool,
        west: bool,
    },
}

impl Direction {
    fn all() -> [Self; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West
        ]
    }
}

pub struct Solver {
    reader: AsciiReader,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position(Option<usize>);

impl Position {
    pub fn new(position: usize) -> Self {
        Self(Some(position))
    }

    fn step(&mut self, direction: Direction, width: usize, max_position: usize) -> &mut Self {
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
        };

        self
    }

    fn on_perimiter(&self, width: usize, max_position: usize) -> bool {
        if let Some(position) = self.0 {
            position < width || position > max_position - width || position % width == 0 || (position + 2) % width == 0
        } else {
            false
        }
    }
}

impl Solver {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self {
            reader: AsciiReader::new(buffer),
        }
    }

    fn print_maze(&self) {
        let mut maze = String::from_utf8(self.reader.buffer.clone()).unwrap();
        maze = maze.replacen('F', "┌", self.reader.buffer.len())
            .replacen('7', "┐", self.reader.buffer.len())
            .replacen('|', "│", self.reader.buffer.len())
            .replacen('-', "─", self.reader.buffer.len())
            .replacen('J', "┘", self.reader.buffer.len())
            .replacen('L', "└", self.reader.buffer.len())
            .replacen('I', "•", self.reader.buffer.len());

        
        println!("{}", maze);
    }

    pub fn get_maze_width(&mut self) -> u32 {
        self.reader.seek(0);

        // Add one here to include the newline as part of the maze tiles
        let width = self.reader.read_until(b'\n').unwrap().len() + 1;

        self.reader.seek(0);

        width as u32
    }

    fn find_loop(&mut self, start_position: u32, width: u32, direction: Direction) -> Option<Vec<usize>> {
        // println!("find_loop: direction: {:?}", direction);
        let mut loop_length = 0;
        let max_position = self.reader.len() - 1;
        let mut position = Position(Some(start_position as usize));
        let mut loop_positions: Vec<usize> = vec![position.0.unwrap()];

        // Check if we can step in the given direction
        let mut candidate_position = position;
        candidate_position.step(direction, width as usize, max_position);
        let candidate_tile = self.reader.at(candidate_position.0?);
        match direction {
            Direction::North => match candidate_tile {
                b'|' | b'F' | b'7' => {},
                _ => {
                    return None;
                }
            },
            Direction::East => match candidate_tile {
                b'-' | b'J' | b'7' => {},
                _ => {
                    return None;
                }
            },
            Direction::South => match candidate_tile {
                b'|' | b'J' | b'L' => {},
                _ => {
                    return None;
                }
            },
            Direction::West => match candidate_tile {
                b'-' | b'F' | b'L' => {},
                _ => {
                    return None;
                }
            },
        }
        position = candidate_position;


        let mut last_direction = direction;

        loop {
            loop_length += 1;
            let tile = self.reader.at(position.0?);

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

            if let Some(position) = position.0 {
                loop_positions.push(position);
            }

            position.step(last_direction, width as usize, max_position);

            if position.0? == start_position as usize {
                break;
            }
        }

        let mut has_north = direction == Direction::North;
        let mut has_east = direction == Direction::East;
        let mut has_south = direction == Direction::South;
        let mut has_west = direction == Direction::West;

        for d in Direction::all() {
            if d == direction {
                continue;
            }
            let mut candidate_position = Position(Some(start_position as usize));
            candidate_position.step(d, width as usize, max_position);
            let in_loop = candidate_position.0.is_some() && loop_positions.contains(&candidate_position.0.unwrap());
            let tile = self.reader.at(candidate_position.0?);
            match d {
                Direction::North => {
                    if tile == b'|' || tile == b'F' || tile == b'7' {
                        has_north = true;
                        break;
                    }
                },
                Direction::East => {
                    if tile == b'-' || tile == b'J' || tile == b'7' {
                        has_east = true;
                        break;
                    }
                },
                Direction::South => {
                    if tile == b'|' || tile == b'J' || tile == b'L' {
                        has_south = true;
                        break;
                    }
                },
                Direction::West => {
                    if tile == b'-' || tile == b'F' || tile == b'L' {
                        has_west = true;
                        break;
                    }
                },
            }
        }

        match (has_north, has_east, has_south, has_west) {
            (true, true, false, false) => {
                self.reader.buffer[start_position as usize] = b'L';
            },
            (true, false, true, false) => {
                self.reader.buffer[start_position as usize] = b'|';
            },
            (true, false, false, true) => {
                self.reader.buffer[start_position as usize] = b'J';
            },
            (false, true, true, false) => {
                self.reader.buffer[start_position as usize] = b'F';
            },
            (false, true, false, true) => {
                self.reader.buffer[start_position as usize] = b'-';
            },
            (false, false, true, true) => {
                self.reader.buffer[start_position as usize] = b'7';
            },
            _ => {
                panic!("Could not determine start tile {has_north} {has_east} {has_south} {has_west}");
            }
        }

        // println!("loop_length: {}", loop_length);

        Some(loop_positions)
    }

    fn mark_loop_path(&mut self, loop_positions: &[usize], max_position: usize) {
        for position in 0..max_position {
            if loop_positions.contains(&position) {
                continue;
            }
            if self.reader.at(position) == b'\n' {
                continue;
            }
            self.reader.buffer[position] = b'.';
        }
    }

    fn count_enclosed_tiles(&mut self, loop_positions: &[usize], width: usize, max_position: usize) -> u32 {
        self.mark_loop_path(loop_positions, max_position);

        let mut count = 0;

        for position in 0..max_position {
            let tile = self.reader.at(position);
            if tile != b'.' {
                continue;
            }
            // Count how many transitions happen from this position to the end of the line
            // A tile inside the loop will have an odd number of transitions
            let mut transitions: i32 = 0;
            let mut test_position = position + 1;
            let mut last_transition_tile = None;
            loop {
                let test_tile = self.reader.at(test_position);
                match test_tile {
                    b'.' => {
                        last_transition_tile = None;
                    },
                    b'\n' => {
                        break;
                    },
                    b'J' => {
                        if last_transition_tile != Some(b'F') {
                            last_transition_tile = Some(test_tile);
                            transitions += 1;
                        }
                    },
                    b'7' => {
                        if last_transition_tile != Some(b'L') {
                            last_transition_tile = Some(test_tile);
                            transitions += 1;
                        }
                    },
                    b'F' | b'L' | b'|' | b'L' => {
                        last_transition_tile = Some(test_tile);
                        transitions += 1;
                    }
                    b'-' => {}, 
                    _ => {
                        panic!("Unexpected tile: {}", test_tile);
                    }
                }
                test_position += 1;
            }

            // If there are no transitions, skip to the next line
            if transitions == 0 {
                test_position = width * (test_position.div_ceil(width) + 1);
            } else if (transitions % 2) == 1 {
                count += 1;
            }
        }

        count
    }

    pub fn solve(&mut self) -> u32 {
        let width = self.get_maze_width();

        let start_position = self.reader.read_until(b'S').unwrap().len() as u32;
        
        for direction in Direction::all().iter() {
            if let Some(loop_positions) = self.find_loop(start_position, width, *direction) {
                return self.count_enclosed_tiles(&loop_positions, width as usize, self.reader.len() - 1);
            }
        }

        panic!("No loop found");
    }
}

#[derive(Parser, Debug)]
pub struct Day10b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day10b {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day10b: {answer}");
        // 762 is too high
        // 1777
        Ok(())
    }
}
