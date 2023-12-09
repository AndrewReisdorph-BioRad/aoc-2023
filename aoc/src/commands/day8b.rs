use std::path::PathBuf;

use clap::Parser;

use crate::utils::{AsciiReader, slurp_bytes};

use super::{CommandImpl, DynError};

struct Cycle {
    pub period: u64,
    pub z_indices: Vec<u64>
}

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

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn index(&self) -> usize {
        self.index
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

#[derive(Debug, Clone, Copy, PartialEq)]
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

        let mut test_locations = Vec::new();

        let mut z_cycle_idx: Vec<u64> = vec![];

        // Build graph
        let mut graph: [[[Option<Node>; 26]; 26]; 26] = [[[None; 26]; 26]; 26];
        while let Some((node, location)) = self.read_next_node() {
            if location.2 == 0 {
                test_locations.push(location);
            }

            // if location.2 == 25 {
            //     num_zs += 1;
            // }

            graph[location.0 as usize][location.1 as usize][location.2 as usize] = Some(node);
        }

        // println!("Num Zs: {}", num_zs);

        let mut z_product: u64 = 1;

        for location in test_locations.iter_mut() {
            instruction_set.reset();
            let mut num_steps: u64 = 0;
            let mut steps = vec![];
            let mut z_step_indices = vec![];
            'outer: loop {
                steps.push((*location, instruction_set.index()));
                // Keep track of where Z's are found
                if location.2 == 25 {
                    z_step_indices.push(num_steps);
                    z_cycle_idx.push(num_steps);
                    break;
                }

                // println!("{:?}", location);
                num_steps += 1;

                // Move to the next node
                match instruction_set.next() {
                    Instruction::Left => {
                        *location = graph[location.0 as usize][location.1 as usize][location.2 as usize].unwrap().left;
                    },
                    Instruction::Right => {
                        *location = graph[location.0 as usize][location.1 as usize][location.2 as usize].unwrap().right;
                    }
                }
                
                // Check for cycles
                // for (idx, step) in steps.iter().enumerate() {
                //     if step.0 == *location && step.1 == instruction_set.index(){
                //         // println!("{:?}", location);
                //         // println!("Cycle {} <-> {} Length: {} - Z's {z_step_indices:?}", idx, num_steps, num_steps - (idx as u64));
                //         // z_product *= z_step_indices[0] as u64;
                //         z_cycle_idx.push(z_step_indices[0]);
                //         break 'outer;
                //     }
                // }
            }
            // println!("{:?}", num_steps - 1);
        }


        // Traverse graph
        // let mut num_steps: u64 = 0;
        // loop {
        //     let mut all_locations_end_with_z = true;
        //     // println!("{:?}", test_locations);
        //     num_steps += 1;
        //     let instruction = instruction_set.next();

        //     for location in test_locations.iter_mut() {
        //         match instruction {
        //             Instruction::Left => {
        //                 *location = graph[location.0 as usize][location.1 as usize][location.2 as usize].unwrap().left;
        //             },
        //             Instruction::Right => {
        //                 *location = graph[location.0 as usize][location.1 as usize][location.2 as usize].unwrap().right;
        //             }
        //         }
        //         if location.2 != 25 {
        //             all_locations_end_with_z = false;
        //         }
        //     }

        //     // println!("{:?}", test_locations);

        //     if all_locations_end_with_z {
        //         break;
        //     }
        // }

        lcmx::lcmx(&z_cycle_idx).unwrap()
    }
}

#[derive(Parser, Debug)]
pub struct Day8b {
    #[clap(long, short)]
    input: PathBuf,
}

#[derive(Debug)]
struct Foo {
    pub base: u64,
    pub value: u64
}

impl CommandImpl for Day8b {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input)?).solve();
        println!("Day8b: {answer}");
        // 16579584610977290608777500 is too high
        // 16579584610977290608789412 is too high
        // 14386467658893820428288 is too high

        // 10151663816849

        // panic!();

        // let mut data = vec![
        //     Foo {base: 21883, value: 21883},
        //     Foo {base: 19667, value: 19667},
        //     Foo {base: 14681, value: 14681},
        //     Foo {base: 16897, value: 16897},
        //     Foo {base: 13019, value: 13019},
        //     Foo {base: 11911, value: 11911},
        // ];


        Ok(())
    }
}
