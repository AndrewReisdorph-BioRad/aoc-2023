use std::{path::PathBuf, fs::File, io::{BufReader, Read}};

use clap::Parser;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day3 {
    #[clap(long, short)]
    input: PathBuf,
}

impl Day3 {
    fn part_one(&mut self) {
        let f: File = File::open(self.input.as_path()).unwrap();
        let mut input = vec![];
        let mut reader = BufReader::new(f);
        reader.read_to_end(input.as_mut()).unwrap();

//         let input = b"467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
//         ";

        let mut sum: u64 = 0;
        let mut read_idx: usize = 0;
        let mut number_idx: Option<usize> = None;

        // Find the width of the input
        let mut width = 0;
        loop {
            width += 1;
            if input[read_idx] == b'\n' {
                break;
            }
            read_idx += 1;
        }
        read_idx = 0;

        // println!("Width: {}", width);

        'outer: loop {
            // Find a number
            while !input[read_idx].is_ascii_digit() {
                read_idx += 1;
                if read_idx >= input.len() {
                    break 'outer;
                }
            }
            number_idx = Some(read_idx);

            // Check if any digit is adjacent to a symbol
            let mut adjacent_to_symbol = false;
            'check_adjacent: loop {
                // println!("read_idx: {}", read_idx);
                let mut north_idx = None;
                let mut east_idx = None;
                let mut north_east_idx = None;
                let mut south_idx = None;
                let mut south_east_idx = None;
                let mut west_idx = None;
                let mut south_west_idx = None;
                let mut north_west_idx = None;

                // North: Any position after first row
                if read_idx > width {
                    north_idx = Some(read_idx - width);
                }
                // East: Any position not in last column
                if (read_idx + 1) % width != width {
                    east_idx = Some(read_idx + 1);
                }
                // South: Any position before last row
                if read_idx < input.len() - width {
                    south_idx = Some(read_idx + width);
                }
                // West: Any position not in first column
                if read_idx % width != 0 {
                    west_idx = Some(read_idx - 1);
                }

                // North-East: Any position after first row and before last column
                if north_idx.is_some() && east_idx.is_some() {
                    north_east_idx = Some(north_idx.unwrap() + 1);
                }
                // South-East: Any position before last row and before last column
                if south_idx.is_some() && east_idx.is_some() {
                    south_east_idx = Some(south_idx.unwrap() + 1);
                }

                // North-West: Any position after first row and before last column
                if north_idx.is_some() && west_idx.is_some() {
                    north_west_idx = Some(north_idx.unwrap() - 1);
                }
                // South-West: Any position before last row and before last column
                if south_idx.is_some() && west_idx.is_some() {
                    south_west_idx = Some(south_idx.unwrap() - 1);
                }

                for idx in [north_idx, north_east_idx, east_idx, south_east_idx, south_idx, south_west_idx, west_idx, north_west_idx].iter().flatten() {
                    if input[*idx] != b'.' && input[*idx] != b'\n' && !input[*idx].is_ascii_digit() {
                        // println!("idx: {}", idx);
                        adjacent_to_symbol = true;
                        break 'check_adjacent;
                    }
                }
                read_idx += 1;

                if !input[read_idx].is_ascii_digit() {
                    break;
                }
            }

            // Find end of number
            while input[read_idx].is_ascii_digit() {
                read_idx += 1;
            }

            // println!("\n\nDigit: {} Adjacent to symbol: {}", String::from_utf8(input[number_idx.unwrap()..read_idx].to_vec()).unwrap(), adjacent_to_symbol);

            if adjacent_to_symbol {
                let number = String::from_utf8(input[number_idx.unwrap()..read_idx].to_vec()).unwrap().parse::<u32>().unwrap();
                sum += number as u64;
            }
            // panic!()
        }

        // 556590 is too high
        // 556367
        // 463346 is too low
        println!("Sum: {}", sum);


    }

    fn part_two(&mut self) {
        let f: File = File::open(self.input.as_path()).unwrap();
        let mut input = vec![];
        let mut reader = BufReader::new(f);
        reader.read_to_end(input.as_mut()).unwrap();
//         let input = b"467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..
//         ";

        let mut sum: u64 = 0;
        let mut read_idx: usize = 0;
        let mut number_idx: Option<usize> = None;

        // Find the width of the input
        let mut width = 0;
        loop {
            width += 1;
            if input[read_idx] == b'\n' {
                break;
            }
            read_idx += 1;
        }
        read_idx = 0;

        'outer: loop {
            // Find a '*'
            while input[read_idx] != b'*' {
                read_idx += 1;
                if read_idx >= input.len() {
                    break 'outer;
                }
            }

            let mut north_idx = None;
            let mut east_idx = None;
            let mut north_east_idx = None;
            let mut south_idx = None;
            let mut south_east_idx = None;
            let mut west_idx = None;
            let mut south_west_idx = None;
            let mut north_west_idx = None;

            // North: Any position after first row
            if read_idx > width {
                north_idx = Some(read_idx - width);
            }
            // East: Any position not in last column
            if (read_idx + 1) % width != width {
                east_idx = Some(read_idx + 1);
            }
            // South: Any position before last row
            if read_idx < input.len() - width {
                south_idx = Some(read_idx + width);
            }
            // West: Any position not in first column
            if read_idx % width != 0 {
                west_idx = Some(read_idx - 1);
            }

            // North-East: Any position after first row and before last column
            if north_idx.is_some() && east_idx.is_some() {
                north_east_idx = Some(north_idx.unwrap() + 1);
            }
            // South-East: Any position before last row and before last column
            if south_idx.is_some() && east_idx.is_some() {
                south_east_idx = Some(south_idx.unwrap() + 1);
            }

            // North-West: Any position after first row and before last column
            if north_idx.is_some() && west_idx.is_some() {
                north_west_idx = Some(north_idx.unwrap() - 1);
            }
            // South-West: Any position before last row and before last column
            if south_idx.is_some() && west_idx.is_some() {
                south_west_idx = Some(south_idx.unwrap() - 1);
            }

            let mut numeric_neighbors: Vec<usize> = Vec::with_capacity(8);
            // The order of these matters for the neighbor check below. Check in order of increasing idx value
            for idx in [north_west_idx, north_idx, north_east_idx, east_idx, west_idx, south_west_idx, south_idx, south_east_idx].into_iter().flatten() {
                if input[idx].is_ascii_digit() {
                    numeric_neighbors.push(idx);
                }
            }

            // A gear is any * symbol that is adjacent to exactly two part numbers. 
            if numeric_neighbors.len() < 2 {
                read_idx += 1;
                continue;
            }

            // Numeric neighbors may be touching each other, which would reduce the total number neighbor count
            let mut neighbors: Vec<usize> = Vec::with_capacity(2);
            let mut neighbor_start = numeric_neighbors[0];
            let mut last_neighbor = neighbor_start;
            for neighbor in numeric_neighbors[1..].iter() {
                if *neighbor != last_neighbor + 1 {
                    neighbors.push(neighbor_start);
                    neighbor_start = *neighbor;
                }
                last_neighbor = *neighbor;
            }
            neighbors.push(neighbor_start);
            read_idx += 1;

            if neighbors.len() != 2 {
                continue;
            }

            let gear_ratio = self.get_number(&input[..], neighbors[0]) * self.get_number(&input[..], neighbors[1]);
            sum += gear_ratio as u64;
        }
        println!("sum: {}", sum);

    }

    fn get_number(&self, input: &[u8], idx: usize) -> u32 {
        let mut right_idx = idx + 1;
        let mut left_idx = if idx > 0 { idx - 1 } else { 0 };
        let mut start_idx = idx;
        let mut end_idx = idx;
        while input[left_idx].is_ascii_digit() {
            start_idx = left_idx;
            if left_idx == 0 {
                break;
            }
            left_idx -= 1;
        }
        while input[right_idx].is_ascii_digit() {
            end_idx = right_idx;
            right_idx += 1;
        }
        // println!("Found: {:?}", String::from_utf8(input[start_idx..end_idx + 1].to_vec()));
        String::from_utf8(input[start_idx..end_idx + 1].to_vec()).unwrap().parse::<u32>().unwrap()
    }
}

impl CommandImpl for Day3 {
    fn main(&mut self) -> Result<(), DynError> {
        self.part_two();
        Ok(())
    }
}
