use std::path::PathBuf;

use clap::Parser;

use crate::utils::{AsciiReader, slurp_bytes};

use super::{day16a::{Map, Direction}, CommandImpl, DynError};


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
        let mut map = Map::new(&mut self.reader);

        let mut greatest = 0;

        for y in 0..map.height() {
            for (x, direction) in [(0, Direction::East), (map.width() - 1, Direction::West)] {
                map.cast_ray(x, y, direction);
                let visited = map.count_visited();
                greatest = greatest.max(visited);
                map.reset();
            }
        }

        for x in 0..map.width() {
            for (y, direction) in [(0, Direction::South), (map.height() - 1, Direction::North)] {
                map.cast_ray(x, y, direction);
                let visited = map.count_visited();
                greatest = greatest.max(visited);
                map.reset();
            }
        }

        greatest as u64
    }
}

#[derive(Parser, Debug)]
pub struct Day16b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day16b {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day 16 B: {answer}");
        Ok(())
    }
}
