use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use clap::Parser;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day2 {
    #[clap(long, short)]
    input: PathBuf,
    #[clap(skip)]
    read_idx: usize,
}

#[derive(Debug)]
struct CubeSet {
    blue: u8,
    red: u8,
    green: u8,
}

impl CubeSet {
    fn new() -> Self {
        Self { blue: 0, red: 0, green: 0 }
    }

    fn get_power(&self) -> u64 {
        self.blue as u64 * self.red as u64 * self.green as u64
    }
}

#[derive(Debug)]
struct Game {
    number: u16,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn get_minimal_cube_set(&self) -> CubeSet {
        let mut min_cube_set = CubeSet::new();

        for cube_set in self.cube_sets.iter() {
            min_cube_set.blue = min_cube_set.blue.max(cube_set.blue);
            min_cube_set.red = min_cube_set.red.max(cube_set.red);
            min_cube_set.green = min_cube_set.green.max(cube_set.green);
        }

        min_cube_set
    }
}

impl Day2 {
    fn part_one(&mut self) {
        let f: File = File::open(self.input.as_path()).unwrap();
        let mut buffer = vec![];
        let mut reader = BufReader::new(f);
        reader.read_to_end(buffer.as_mut()).unwrap();

        let mut game_sum = 0;

        loop {
            // Get game number
            // Move ahead 5 characters to skip the text "Game "
            self.read_idx += 5;
            let mut game_number_slice_start = self.read_idx;

            let game_number = self.get_next_number(&buffer);

            // println!("Game number: {game_number}");

            // Move past the colon and space after the game number
            self.read_idx += 2;

            let mut cube_sets: Vec<CubeSet> = vec![];

            // Read in this games values
            'outer: loop {
                let mut cube_set = CubeSet::new();

                loop {
                    let cube_count = self.get_next_number(&buffer);
                    // Move past space after cube count
                    self.read_idx += 1;

                    match buffer[self.read_idx] {
                        b'b' => {
                            self.read_idx += 4;
                            cube_set.blue = cube_count as u8;
                        }
                        b'r' => {
                            self.read_idx += 3;
                            cube_set.red = cube_count as u8;
                        }
                        b'g' => {
                            self.read_idx += 5;
                            cube_set.green = cube_count as u8;
                        }
                        _ => {
                            panic!("Unexpected color: {}", buffer[self.read_idx] as char);
                        }
                    }

                    // Check for a comma or semicolon
                    if buffer[self.read_idx] == b',' {
                        self.read_idx += 2;
                    } else if buffer[self.read_idx] == b';' {
                        cube_sets.push(cube_set);
                        self.read_idx += 2;
                        break;
                    } else if (buffer[self.read_idx] == b'\n') {
                        cube_sets.push(cube_set);
                        self.read_idx += 1;
                        break 'outer;
                    } else {
                        panic!("Unexpected character: {}", buffer[self.read_idx] as char);
                    }
                }
            }

            // println!("Cube sets: {:?}", cube_sets);

            // Check if the game can be played with only 12 red cubes, 13 green cubes, and 14 blue cubes
            if self.game_is_possible(cube_sets.as_slice()) {
                game_sum += game_number;
            }

            // Check if we are at the end of the file
            if self.read_idx >= buffer.len() {
                break;
            }
        }

        println!("Game sum: {}", game_sum);
    }

    fn part_two(&mut self) {
        let f: File = File::open(self.input.as_path()).unwrap();
        let mut buffer = vec![];
        let mut reader = BufReader::new(f);
        reader.read_to_end(buffer.as_mut()).unwrap();

        let mut power_sum: u64 = 0;
        loop {
            let game = self.get_next_game(&buffer);
            if let Some(game) = game {
                power_sum += game.get_minimal_cube_set().get_power();
            } else {
                break;
            }
        }

        // 62031 too low

        println!("Power sum: {}", power_sum);
    }

    fn game_is_possible(&self, cube_sets: &[CubeSet]) -> bool {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        for cube_set in cube_sets {
            if cube_set.red > max_red || cube_set.green > max_green || cube_set.blue > max_blue {
                return false;
            }
        }

        true
    }

    fn get_next_number(&mut self, buffer: &[u8]) -> u16 {
        let mut number_slice_start = self.read_idx;

        while buffer[self.read_idx].is_ascii_digit() {
            self.read_idx += 1;
        }

        let number: u16 = std::str::from_utf8(&buffer[number_slice_start..self.read_idx])
            .unwrap()
            .parse()
            .unwrap();

        number
    }

    fn get_next_game(&mut self, buffer: &[u8]) -> Option<Game> {
        // Check if we are at the end of the file
        if self.read_idx >= buffer.len() {
            return None;
        }

        // Get game number
        // Move ahead 5 characters to skip the text "Game "
        self.read_idx += 5;

        let game_number = self.get_next_number(&buffer);

        // println!("Game number: {game_number}");

        // Move past the colon and space after the game number
        self.read_idx += 2;

        let mut cube_sets: Vec<CubeSet> = vec![];

        // Read in this games values
        'outer: loop {
            let mut cube_set = CubeSet::new();

            loop {
                let cube_count = self.get_next_number(&buffer);
                // Move past space after cube count
                self.read_idx += 1;

                match buffer[self.read_idx] {
                    b'b' => {
                        self.read_idx += 4;
                        cube_set.blue = cube_count as u8;
                    }
                    b'r' => {
                        self.read_idx += 3;
                        cube_set.red = cube_count as u8;
                    }
                    b'g' => {
                        self.read_idx += 5;
                        cube_set.green = cube_count as u8;
                    }
                    _ => {
                        panic!("Unexpected color: {}", buffer[self.read_idx] as char);
                    }
                }

                // Check for a comma or semicolon
                if buffer[self.read_idx] == b',' {
                    self.read_idx += 2;
                } else if buffer[self.read_idx] == b';' {
                    cube_sets.push(cube_set);
                    self.read_idx += 2;
                    break;
                } else if (buffer[self.read_idx] == b'\n') {
                    cube_sets.push(cube_set);
                    self.read_idx += 1;
                    break 'outer;
                } else {
                    panic!("Unexpected character: {}", buffer[self.read_idx] as char);
                }
            }
        }

        Some(Game { number: game_number, cube_sets })
    }
}

impl CommandImpl for Day2 {
    fn main(&mut self) -> Result<(), DynError> {
        self.part_two();
        Ok(())
    }
}
