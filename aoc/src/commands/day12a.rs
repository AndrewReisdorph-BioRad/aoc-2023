use std::{path::PathBuf, fmt::Display};

use clap::Parser;

use crate::utils::{AsciiReader, slurp_bytes};

use super::{CommandImpl, DynError};

struct Solver {
    reader: AsciiReader,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

#[derive(Debug, Clone)]
struct SpringRow {
    springs: Vec<Spring>,
    groups: Vec<u32>
}

impl Display for SpringRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut springs = String::new();
        for spring in &self.springs {
            match spring {
                Spring::Operational => springs.push('.'),
                Spring::Damaged => springs.push('#'),
                Spring::Unknown => springs.push('?'),
            }
        }
        let mut groups = String::new();
        for group in &self.groups {
            groups.push_str(&format!("{} ", group));
        }
        write!(f, "{} {}", springs, groups)
    }
}

impl Solver {
    fn new(input: Vec<u8>) -> Self {
        Self {
            reader: AsciiReader::new(input),
        }
    }

    fn next_line(&mut self) -> Option<SpringRow> {
        if self.reader.eof() {
            return None;
        }

        let springs: Vec<Spring> = self.reader.read_until(b' ').unwrap().iter().map(|s| {
            match s {
                b'.' => Spring::Operational,
                b'#' => Spring::Damaged,
                _ => Spring::Unknown
            }
        }).collect();
        self.reader.skip(1);

        let mut groups = vec![];
        while let Some(value) = self.reader.read_next_number() {
            groups.push(value as u32);
        }

        self.reader.skip(1);


        Some(SpringRow {
            springs,
            groups
        })
    }

    fn count_arrangements(row: &SpringRow) -> u32 {
        println!("Counting arrangements for: {:?}", row);
        // Validate that the row spring groups are valid
        // If an unknown spring is found create two new rows
        // with the unknown replaced with damaged and operational respectively
        // Count the arrangement for each row and return the sum

        let mut group_iter = row.groups.iter();
        let mut last_spring = None;
        let mut current_group_remaining: Option<u32> = None;//group_iter.next().unwrap_or(&0);

        for (idx, spring) in row.springs.iter().enumerate() {
            match spring {
                Spring::Unknown => {
                    println!("Found unknown spring at index: {idx}. Testing twice. Once if damaged and once if operational");
                    let mut new_row_damaged = row.clone();
                    new_row_damaged.springs[idx] = Spring::Damaged;
                    let mut new_row_operational = row.clone();
                    new_row_operational.springs[idx] = Spring::Operational;
                    return Self::count_arrangements(&new_row_damaged) + Self::count_arrangements(&new_row_operational);
                },
                Spring::Operational => {
                    if let Some(current_group_remaining) = current_group_remaining {
                        if current_group_remaining > 0 {
                            println!("Found operational spring but expected damaged. Invalid arrangement");
                            return 0;
                        }
                    }
                    current_group_remaining = None;
                },
                Spring::Damaged => {
                    if let Some(remaining) = current_group_remaining {
                        if remaining == 0 {
                            println!("Found damaged spring but expected operational. Invalid arrangement");
                            return 0;
                        } else {
                            current_group_remaining = Some(remaining - 1);
                        }
                    } else {
                        current_group_remaining = Some(*group_iter.next().unwrap_or(&0));
                        if current_group_remaining.unwrap() == 0 {
                            println!("Found damaged spring but no groups remaining. Invalid arrangement");
                            return 0;
                        }
                        current_group_remaining = Some(current_group_remaining.unwrap() - 1);
                    }
                },
            }
            last_spring = Some(*spring);
        }

        if current_group_remaining.is_some() && current_group_remaining.unwrap() > 0 {
            println!("Damage count for group not satisfied but no springs left. Invalid arrangement");
            return 0;
        }

        if group_iter.next().is_some() {
            println!("Groups remaining but springs left. Invalid arrangement");
            return 0;
        }

        println!("\nFound valid arrangement: {row}\n");
        1
    }

    fn solve(&mut self) -> u64 {
        let mut sum: u64 = 0;
        while let Some(row) = self.next_line() {
            println!("\n\n\n");
            let arrangements = Self::count_arrangements(&row) as u64;
            println!("{:?}", row);
            println!("Arrangements: {}", arrangements);
            sum += arrangements;
        }
        sum
    }
}

#[derive(Parser, Debug)]
pub struct Day12a {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day12a {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day12a: {answer}");
        Ok(())
    }
}
