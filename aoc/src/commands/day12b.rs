use std::{fmt::Display, path::PathBuf, vec, collections::HashMap, ops::IndexMut, process::id};

use clap::Parser;

use crate::utils::{slurp_bytes, AsciiReader};

use super::{CommandImpl, DynError};

struct Solver {
    reader: AsciiReader,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum SpringRowStatus {
    Valid,
    Invalid,
    Incomplete,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct SpringRow {
    springs: Vec<Spring>,
    groups: Vec<u32>,
}

impl SpringRow {
    fn expand(&mut self, factor: u16) {
        let mut new_springs = vec![];
        let mut new_groups = vec![];
        for count in 0..factor {
            new_springs.append(&mut self.springs.clone());
            if count < factor - 1 {
                new_springs.push(Spring::Unknown);
            }
            new_groups.append(&mut self.groups.clone());
        }
        self.springs = new_springs;
        self.groups = new_groups;
    }

    fn reduce_with_damage(&mut self, no_unknowns: bool) -> bool {
        if self.groups.is_empty() {
            // println!("reduce_with_damage groups is empty");
            return false;
        }

        let mut group_idx: usize = 0;
        let mut damaged_remaining = self.groups[0] as usize;
        let mut in_damage_group = false;

        let mut cut_idx = None;
        let mut found_unknown_group = false;
        let mut found_unknown = false;

        for (idx, spring) in self.springs.iter().enumerate() {
            match spring {
                Spring::Unknown => {
                    if in_damage_group {
                        if damaged_remaining > 0 {
                            damaged_remaining -= 1;
                        } else {
                            group_idx += 1;
                            damaged_remaining = if group_idx >= self.groups.len() {
                                0
                            } else {
                                self.groups[group_idx] as usize
                            };
                            in_damage_group = false;
                            // println!("Leaving damage group: {idx}  damaged_remaining: {damaged_remaining}");
                        }
                    } else if found_unknown_group || damaged_remaining == 0 || no_unknowns {
                        cut_idx = Some(idx);
                        break;
                    } else if damaged_remaining > 0 {
                        damaged_remaining -= 1;
                        in_damage_group = true;
                        if found_unknown {
                            cut_idx = Some(idx);
                            break;        
                        }
                    }
                    found_unknown = true;
                },
                Spring::Operational => {
                    if in_damage_group && damaged_remaining > 0 {
                        // println!("Found operational spring but expected damaged. Invalid arrangement");
                        return false;
                    } else if damaged_remaining == 0 {
                        // CHECK THIS
                        if group_idx < self.groups.len() {
                            group_idx += 1;
                        }
                        damaged_remaining = if group_idx >= self.groups.len() {
                            0
                        } else {
                            self.groups[group_idx] as usize
                        }
                    }
                    in_damage_group = false;
                },
                Spring::Damaged => {
                    in_damage_group = true;
                    if damaged_remaining == 0 {
                        // println!("Found damaged spring but expected operational. Invalid arrangement. {idx}");
                        return false;
                    }
                    damaged_remaining -= 1;
                },
            }
        }

        if cut_idx.is_none() {
            if damaged_remaining > 0 || group_idx < self.groups.len() - 1 { // CHECK THIS
                // println!("All springs scanned but damage groups remain. Invalid arrangement");
                return false;
            }
            self.springs = vec![];
            self.groups = vec![];
            return true;
        } else {
            let cut_idx = cut_idx.unwrap();
            // println!("cut_idx: {cut_idx} group_idx {group_idx}");
            let damage_springs_remain: bool = self.springs.iter().skip(cut_idx).any(|s| *s == Spring::Damaged);
            if !damage_springs_remain && damaged_remaining == 0 && group_idx > self.groups.len() - 1 {
                self.springs = vec![];
                self.groups = vec![];
            } else {
                self.springs = self.springs[cut_idx..].to_vec();
                self.groups = self.groups[group_idx..].to_vec();    
            }
        }

        true
    }

    fn reduce_with_operational(&mut self) -> bool {
        let mut found_damaged = false;
        let mut idx = 0;
        while self.springs[idx] != Spring::Unknown {
            if self.springs[idx] == Spring::Damaged {
                found_damaged = true;
            }
            idx += 1;
        }
        self.springs[idx] = Spring::Operational;

        while idx < self.springs.len() && self.springs[idx] == Spring::Operational {
            idx += 1;
        }

        // idx += 1;

        if found_damaged || (idx < self.springs.len() && self.springs[idx] == Spring::Damaged) {
            // println!("{self}");
            self.reduce_with_damage(true)
        } else {
            // println!("reduce operational: found no damaged");
            self.springs = self.springs[idx..].to_vec();
            true
        }
        // true
    }

    fn status(&self) -> SpringRowStatus {
        let mut group_iterator = self.groups.iter();
        let mut damaged_remaining = *group_iterator.next().unwrap_or(&0);
        let mut in_damage_group = false;

        for spring in &self.springs {
            match spring {
                Spring::Unknown => return SpringRowStatus::Incomplete,
                Spring::Operational => {
                    if in_damage_group && damaged_remaining > 0 {
                        // println!("Found operational spring but expected damaged. Invalid arrangement");
                        return SpringRowStatus::Invalid;
                    } else if damaged_remaining == 0 {
                        damaged_remaining = *group_iterator.next().unwrap_or(&0);
                    }
                    in_damage_group = false;
                },
                Spring::Damaged => {
                    in_damage_group = true;
                    if damaged_remaining == 0 {
                        // println!("Found damaged spring but expected operational. Invalid arrangement");
                        return SpringRowStatus::Invalid;
                    }
                    damaged_remaining -= 1;
                },
            }
        }

        if damaged_remaining > 0 {
            // println!("All springs scanned but damage groups remain. Invalid arrangement");
            return SpringRowStatus::Invalid;
        }

        SpringRowStatus::Valid
    }

}

enum RowKind {
    Valid,
    Invalid,
    Incomplete(Vec<SpringRow>),
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
            groups.push_str(&format!("{},", group));
        }
        write!(f, "{} {}", springs, groups)
    }
}

impl Solver {
    fn new(input: Vec<u8>) -> Self {
        Self { reader: AsciiReader::new(input) }
    }

    fn next_line(&mut self) -> Option<SpringRow> {
        if self.reader.eof() {
            return None;
        }

        let springs: Vec<Spring> = self
            .reader
            .read_until(b' ')
            .unwrap()
            .iter()
            .map(|s| match s {
                b'.' => Spring::Operational,
                b'#' => Spring::Damaged,
                _ => Spring::Unknown,
            })
            .collect();
        self.reader.skip(1);

        let mut groups = vec![];
        while let Some(value) = self.reader.read_next_number() {
            groups.push(value as u32);
        }

        self.reader.skip(1);

        Some(SpringRow { springs, groups })
    }

    fn check_row(row: &SpringRow) -> RowKind {
        // println!("{row}");
        // Validate that the row spring groups are valid
        // If an unknown spring is found create two new rows
        // with the unknown replaced with damaged and operational respectively
        // Count the arrangement for each row and return the sum

        let mut group_iter = row.groups.iter().enumerate();
        let mut last_spring = None;
        let mut in_damage_group = false;
        let mut damage_group_idx: usize = 0;
        let mut damage_group_remaining = 0;
        let mut is_first_damage_group = true;

        for (idx, spring) in row.springs.iter().enumerate() {
            match spring {
                Spring::Unknown => {
                    // println!(
                    //     "Found unknown              {}^",
                    //     String::from_utf8(vec![b' '; idx]).unwrap()
                    // );
                    let mut row_variants = vec![];
                    // Check if the current expected damaged group can be filled in
                    let mut can_fit_current_group = true;
                    let rest_of_row_has_damaged_springs =
                        row.springs.iter().skip(idx).any(|s| *s == Spring::Damaged);

                    // println!("Checking if remaining damage groups can fit");
                    let mut space_required: usize = {
                        if in_damage_group {
                            damage_group_remaining + 1
                        } else if is_first_damage_group {
                            row.groups[damage_group_idx] as usize + 1
                        } else {
                            0
                        }
                    };
                    // println!("damage_group_idx: {:?} damage_group_remaining: {damage_group_remaining}", damage_group_idx);
                    for group_idx in (damage_group_idx + 1)..row.groups.len() {
                        // println!("Adding {} + 1", row.groups[group_idx]);
                        space_required += row.groups[group_idx] as usize;
                        space_required += 1;
                    }
                    space_required = space_required.saturating_sub(1);

                    let space_available = row.springs.len() - idx;


                    if damage_group_remaining == 0 && damage_group_idx == row.groups.len() - 1 && !rest_of_row_has_damaged_springs {
                        // println!("Damage groups exhausted and there are no more damaged springs in the row. Found valid arrangement");
                        return RowKind::Valid;
                    }


                    if (idx == 0 || !in_damage_group || damage_group_remaining == 0) && (space_required + 1) <= space_available {
                        // println!("No springs remain in damage group. Checking with next spring operational");
                        row_variants.push({
                            let mut new_row = row.clone();
                            new_row.springs[idx] = Spring::Operational;
                            new_row
                        });

                        if in_damage_group {
                            // println!("End of damage group. Skip checking if the next group can be inserted.");
                            return RowKind::Incomplete(row_variants);
                        }
                    }

                    if (idx == 0 || last_spring == Some(Spring::Operational) || in_damage_group)
                        && damage_group_idx < row.groups.len()
                    {
                        // println!(
                        //     "Space required: {} Space Available: {}",
                        //     space_required, space_available
                        // );
                        if space_required > space_available {
                            // println!("Not enough space to fit remaining damage groups. Invalid arrangement");
                        } else {
                            // println!("Remaining damage groups CAN fit");

                            // Create a new row with the unknown replaced with damaged
                            // println!("Creating new row with unknown replaced with damaged");
                            let mut new_row = row.clone();
                            let num_damaged: usize = {
                                if in_damage_group {
                                    damage_group_remaining
                                } else if is_first_damage_group {
                                    row.groups[damage_group_idx] as usize
                                } else {
                                    if damage_group_idx < row.groups.len() - 1 {
                                        row.groups[damage_group_idx + 1] as usize
                                    } else {
                                        0
                                    }
                                }
                            };
                            // println!("num_damaged: {}", num_damaged);
                            let mut new_row_valid = true;
                            for i in 0..num_damaged {
                                if row.springs[idx + i as usize] == Spring::Operational {
                                    // println!("Damage group collides with operational spring. Invalid arrangement");
                                    new_row_valid = false;
                                    break;
                                }
                                new_row.springs[idx + i as usize] = Spring::Damaged;
                            }
    
                            if (idx + num_damaged as usize) < row.springs.len() {
                                if new_row.springs[idx + num_damaged as usize] == Spring::Damaged {
                                    // println!("Group collides with existing damaged spring. Invalid arrangement");
                                    new_row_valid = false;
                                } else {
                                    new_row.springs[idx + num_damaged as usize] = Spring::Operational;
                                }
                            }
    
                            if new_row_valid {
                                row_variants.push(new_row);
                            }
                        }
                    }

                    return RowKind::Incomplete(row_variants);
                }
                Spring::Operational => {
                    if in_damage_group {
                        if damage_group_remaining > 0 {
                            // println!("Found operational spring but expected damaged. Invalid arrangement");
                            return RowKind::Invalid;
                        }
                        in_damage_group = false;
                    }
                }
                Spring::Damaged => {
                    if in_damage_group {
                        if damage_group_remaining == 0 {
                            // println!("{idx} Found damaged spring after last spring in damage group. Invalid arangement");
                            return RowKind::Invalid;
                        }
                        damage_group_remaining -= 1;
                    } else {
                        if is_first_damage_group {
                            is_first_damage_group = false;
                        } else {
                            damage_group_idx += 1;
                        }
                        if damage_group_idx >= row.groups.len() {
                            // println!("Found damaged spring but no damage groups remaining. Invalid arrangement");
                            return RowKind::Invalid;
                        }
                        in_damage_group = true;
                        damage_group_remaining = row.groups[damage_group_idx] as usize - 1;
                    }
                }
            }
            last_spring = Some(*spring);
        }

        // if current_group_remaining.is_some() && current_group_remaining.unwrap() > 0 {
        //     println!("Damage count for group not satisfied but no springs left. Invalid arrangement");
        //     return RowKind::Invalid;
        // }

        // if group_iter.next().is_some() {
        //     println!("Groups remaining but springs left. Invalid arrangement");
        //     return RowKind::Invalid;
        // }
        // println!(
        //     "damage_group_remaining: {damage_group_remaining} damage_group_idx: {}",
        //     damage_group_idx
        // );
        if damage_group_remaining > 0 || damage_group_idx < row.groups.len() - 1 {
            // println!("Scanned all springs but damage groups remain. Invalid arrangement");
            return RowKind::Invalid;
        }

        // println!("Found valid arrangement");
        RowKind::Valid
    }

    fn count_arrangements(row: &SpringRow) -> u64 {
        let mut variants: Vec<SpringRow> = vec![row.clone()];

        let mut sum = 0;

        loop {
            if variants.is_empty() {
                break;
            }
            let check_row = variants.pop().unwrap();
            match Self::check_row(&check_row) {
                RowKind::Valid => {
                    // println!("Got Valid");
                    sum += 1;
                }
                RowKind::Invalid => {
                    continue;
                }
                RowKind::Incomplete(mut new_rows) => {
                    variants.append(&mut new_rows);
                }
            }
        }

        sum
    }


    fn count_arrangements_dynamic(row: &SpringRow, lookup: &mut HashMap<SpringRow, u64>) -> u64 {
        // println!("count_arrangements_dynamic: {row}");
        let mut variants: Vec<SpringRow> = vec![row.clone()];

        let mut sum = 0;

        if lookup.contains_key(row) {
            let count = lookup[row];
            // println!("Found in lookup: {count}");
            return count;
        }

        let status = row.status();

        if status == SpringRowStatus::Valid {
            // println!("Row is Valid");
            return 1;
        }

        if status == SpringRowStatus::Invalid {
            // println!("Row is Invalid");
            return 0;
        }

        // println!("Row is Incomplete");

        // status == SpringRowStatus::Incomplete

        let mut damaged_count = 0;
        let mut damaged_variant = row.clone();
        if damaged_variant.reduce_with_damage(false) {
            // println!("{row} - damaged_variant: {damaged_variant}");
            damaged_count = Self::count_arrangements_dynamic(&damaged_variant, lookup);
        } else {
            // println!("damaged variant invalid");
        }
        lookup.insert(damaged_variant.clone(), damaged_count);

        let mut operational_count = 0;
        let mut operational_variant = row.clone();
        if operational_variant.reduce_with_operational() {
            // println!("row: {row} damaged_variant: {damaged_variant} - operational_variant: {operational_variant}");
            if operational_variant == damaged_variant && operational_variant.springs.len() > 0 {
                // println!("**SAMESIES**");
            }
                operational_count = Self::count_arrangements_dynamic(&operational_variant, lookup)
        } else {
            // println!("operational variant invalid");
        }
        lookup.insert(operational_variant, operational_count);

        damaged_count + operational_count
    }

    fn solve(&mut self) -> u64 {
        let mut row_number = 1;
        let mut sum: u64 = 0;
        while let Some(mut row) = self.next_line() {
            // row.expand(5);
            // println!("Row: {} -- {}", row_number, row);
            let mut lookup: HashMap<SpringRow, u64> = HashMap::new();
            row.reduce_with_damage(true);
            // println!("Initial reduction: {row}");
            let arrangements = Self::count_arrangements_dynamic(&row, &mut lookup);
            // println!("Arrangements: {}\n", arrangements);
            sum += arrangements;
            row_number += 1;
        }
        sum
    }
}

#[derive(Parser, Debug)]
pub struct Day12b {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day12b {
    fn main(&mut self) -> Result<(), DynError> {
        let answer = Solver::new(slurp_bytes(&self.input).unwrap()).solve();
        println!("Day12b: {answer}");
        /*
        Benchmark 1: ./target/release/aoc day12b --input aoc/inputs/day12_small.txt
          Time (mean ± σ):     919.3 ms ±   4.8 ms    [User: 914.6 ms, System: 0.8 ms]
          Range (min … max):   910.8 ms … 927.4 ms    10 runs

❯ hyperfine --warmup 5 './target/release/aoc day12b --input aoc/inputs/day12.txt'
Benchmark 1: ./target/release/aoc day12b --input aoc/inputs/day12.txt
  Time (mean ± σ):      12.7 ms ±   0.6 ms    [User: 11.8 ms, System: 0.5 ms]
  Range (min … max):    11.7 ms …  16.3 ms    176 runs

  ❯ hyperfine --warmup 5 './target/release/aoc day12b --input aoc/inputs/day12.txt'
Benchmark 1: ./target/release/aoc day12b --input aoc/inputs/day12.txt
  Time (mean ± σ):      10.3 ms ±   0.8 ms    [User: 9.3 ms, System: 0.7 ms]
  Range (min … max):     9.0 ms …  16.2 ms    229 runs

  ❯ hyperfine --warmup 5 './target/release/aoc day12b --input aoc/inputs/day12.txt'
Benchmark 1: ./target/release/aoc day12b --input aoc/inputs/day12.txt
  Time (mean ± σ):       9.1 ms ±   0.7 ms    [User: 8.1 ms, System: 0.5 ms]
  Range (min … max):     8.1 ms …  13.5 ms    238 runs
                */

        // 1047509559069226 is too high
        //  160500973317706
        //  158238160354952 is too low
        Ok(())
    }
}
