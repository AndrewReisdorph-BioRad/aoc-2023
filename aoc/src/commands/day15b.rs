use std::{path::PathBuf, ops::Index, collections::HashMap};

use clap::Parser;

use crate::utils::{AsciiReader, slurp_bytes};

use super::{CommandImpl, DynError, day15a::hash};

#[derive(Debug, Clone)]
enum Operation {
    Remove,
    Insert(u32),
}

#[derive(Debug, Clone)]
struct Step {
    label: String,
    hash: u8,
    operation: Operation
}

impl Step {
    fn new(raw: &[u8]) -> Self {
        let mut label = String::new();
        let mut idx = 0_usize;
        let operation = loop {
            let c = raw.index(idx);
            match c {
                b'=' => {
                    let focal_length = String::from_utf8(raw[idx+1..].to_vec()).unwrap();
                    let focal_length = focal_length.parse::<u32>().unwrap();
                    break Operation::Insert(focal_length);
                },
                b'-' => {
                    break Operation::Remove;
                },
                _ => label.push(*c as char)
            }
            idx += 1;
        };
        let hash = hash(label.as_bytes());

        Self {
            label,
            hash,
            operation
        }
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

    fn next_step(&mut self) -> Option<Step> {
        let data = self.reader.read_to(b',');
        Some(Step::new(data?))
    }

    pub fn solve(&mut self) -> u64 {
        let mut sum = 0;
        let mut boxes: HashMap<u8, Vec<Step>> = HashMap::new();

        loop {
            let step = self.next_step();
            if step.is_none() {
                break;
            }
            let step = step.unwrap();

            let mut box_list = boxes.entry(step.hash).or_default();
            let existing = box_list.iter().position(|s| s.label == step.label);

            match step {
                Step { operation: Operation::Insert(focal_length), .. } => {
                    if existing.is_none() {
                        box_list.push(step);
                    } else {
                        box_list[existing.unwrap()].operation = Operation::Insert(focal_length);
                    }
                },
                Step { operation: Operation::Remove, hash, label } => {
                    if let Some(idx) = existing {
                        box_list.remove(idx);
                    }
                }
            }
        }

        for (hash, box_list) in boxes.iter() {
            for (idx, step) in box_list.iter().enumerate() {
                match step.operation {
                    Operation::Insert(focal_length) => {
                        sum += (hash + 1) as u64 * (idx + 1) as u64 * focal_length as u64;
                    },
                    Operation::Remove => {
                        panic!("Remove operation in box list");
                    }
                }
            }
        }

        sum
    }
}

#[derive(Parser, Debug)]
pub struct Day15b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day15b {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day 15 B: {answer}");
        Ok(())
    }
}
