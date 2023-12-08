use std::path::PathBuf;

use clap::Parser;

use crate::utils::{AsciiReader, slurp_bytes};

use super::{CommandImpl, DynError};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right
}

struct InstructionSet {
    pub instructions: Vec<Instruction>,
    pub index: usize
}

impl InstructionSet {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            index: 0
        }
    }

    pub fn next(&mut self) -> Instruction {
        if self.index >= self.instructions.len() {
            self.index = 0;
        }

        let instruction = self.instructions[self.index];
        self.index += 1;
        instruction
    }
}

#[derive(Debug, Clone, Copy)]
struct Location(u8,u8,u8);

impl Location {
    pub fn new(bytes: &[u8]) -> Self {
        Self(
            bytes[0] - 65,
            bytes[1] - 65,
            bytes[2] - 65
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    left: Location,
    right: Location
}

struct Solver {
    reader: AsciiReader
}

impl Solver {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            reader: AsciiReader::new(bytes)
        }
    }

    pub fn read_instructions(&mut self) -> Vec<Instruction> {
        let instruction_slice = self.reader.read_until(b'\n').unwrap();
        let mut instructions = Vec::with_capacity(instruction_slice.len());
        for byte in instruction_slice {
            instructions.push(match byte {
                b'L' => Instruction::Left,
                b'R' => Instruction::Right,
                _ => unreachable!()
            });
        }

        instructions
    }

    pub fn read_next_node(&mut self) -> Option<(Node, Location)> {
        if self.reader.eof() {
            return None;
        }

        let location = Location::new(self.reader.read_until(b' ').unwrap());
        self.reader.skip(4);

        let left = Location::new(self.reader.read_until(b',').unwrap());
        self.reader.skip(2);

        let right = Location::new(self.reader.read_until(b')').unwrap());
        self.reader.skip(2);

        Some((Node {
            left,
            right
        },location))
    }

    pub fn solve(&mut self) -> u64 {
        // Get Instructions
        let mut instruction_set = InstructionSet::new(self.read_instructions());
        self.reader.read_until(b'\n');
        self.reader.skip(2);

        // Build graph
        let mut graph: [[[Option<Node>; 26]; 26]; 26] = [[[None; 26]; 26]; 26];
        while let Some((node, location)) = self.read_next_node() {
            graph[location.0 as usize][location.1 as usize][location.2 as usize] = Some(node);
        }

        // Traverse graph
        let mut location = Location(0,0,0);
        let mut num_steps: u64 = 0;
        loop {
            // println!("{:?}", location);
            num_steps += 1;
            match instruction_set.next() {
                Instruction::Left => {
                    location = graph[location.0 as usize][location.1 as usize][location.2 as usize].unwrap().left;
                },
                Instruction::Right => {
                    location = graph[location.0 as usize][location.1 as usize][location.2 as usize].unwrap().right;
                }
            }

            if location.0 == 25 && location.1 == 25 && location.2 == 25 {
                break;
            }
        }

        num_steps
    }
}

#[derive(Parser, Debug)]
pub struct Day8a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day8a {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input)?).solve();
        println!("Day8a: {answer}");
        Ok(())
    }
}
