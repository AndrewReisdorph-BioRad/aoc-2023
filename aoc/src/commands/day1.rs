use std::fs::{self, File};
use std::io::{Read, BufReader};
use std::path::{Path, PathBuf};

use clap::Parser;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day1 {
    #[clap(long, short)]
    input: PathBuf,
}

impl Day1 {
    pub fn part_one(&self) -> Result<(), DynError> {
        let input = fs::read_to_string(self.input.as_path()).unwrap();
        // println!("EX: {:?}", self.input);
        let mut sum: u64 = 0;

        input.lines().for_each(|line| {
            // println!("Line: {}", line);
            let mut begin_idx = 0;
            let mut end_idx = line.len() - 1;
            let mut first_digit: Option<char> = None;
            let mut last_digit: Option<char> = None;

            loop {
                let char = line.chars().nth(begin_idx).unwrap();
                if char.is_numeric() {
                    first_digit = Some(char);
                    break;
                } else {
                    begin_idx += 1;
                }
            }
            loop {
                let char = line.chars().nth(end_idx).unwrap();
                if char.is_numeric() {
                    last_digit = Some(char);
                    break;
                } else {
                    end_idx -= 1;
                }
            }

            let first: u64 = first_digit.unwrap().to_digit(10).unwrap() as u64;
            let last: u64 = last_digit.unwrap().to_digit(10).unwrap() as u64;

            // println!("Digits: {}{}", first_digit.unwrap(), last_digit.unwrap());

            sum += first * 10 + last;
        });

        // println!("Sum: {}", sum);
        Ok(())
    }

    pub fn part_two(&self) -> Result<(), DynError> {
        let input = fs::read_to_string(self.input.as_path()).unwrap();
        let mut sum: u64 = 0;

        input.lines().for_each(|line| {
            let mut test_idx = 0;

            let mut line_sum: u64 = loop {
                let char = line.chars().nth(test_idx).unwrap();
                match char {
                    '0'..='9' => {
                        break char as u64 - 48;
                    }
                    't' => {
                        // two, three
                        if test_idx + 2 < line.len() {
                            if &line[test_idx..test_idx + 3] == "two" {
                                break 2;
                            } else if (test_idx + 4 < line.len())
                                && &line[test_idx..test_idx + 5] == "three"
                            {
                                break 3;
                            }
                        } else {
                            test_idx += 1;
                            continue;
                        }
                    }
                    'f' => {
                        // four, five
                        if (test_idx + 3) < line.len() {
                            if &line[test_idx..test_idx + 4] == "four" {
                                break 4;
                            } else if &line[test_idx..test_idx + 4] == "five" {
                                break 5;
                            }
                        } else {
                            test_idx += 1;
                            continue;
                        }
                    }
                    's' => {
                        // six, seven
                        if (test_idx + 2 < line.len()) && &line[test_idx..test_idx + 3] == "six" {
                            break 6;
                        } else if (test_idx + 4 < line.len())
                            && &line[test_idx..test_idx + 5] == "seven"
                        {
                            break 7;
                        }
                    }
                    'e' => {
                        // eight
                        if (test_idx + 4 < line.len()) && &line[test_idx..test_idx + 5] == "eight" {
                            break 8;
                        }
                    }
                    'n' => {
                        // nine
                        if (test_idx + 3 < line.len()) && &line[test_idx..test_idx + 4] == "nine" {
                            break 9;
                        }
                    }
                    'o' => {
                        // one
                        if &line[test_idx..test_idx + 3] == "one" {
                            break 1;
                        }
                    }
                    _ => {}
                }

                test_idx += 1;

                if test_idx == line.len() {
                    panic!("No first digit found");
                }
            };

            test_idx = line.len() - 1;

            let last_digit = loop {
                let char = line.chars().nth(test_idx).unwrap();
                match char {
                    '0'..='9' => {
                        break char as u64 - 48;
                    }
                    't' => {
                        // two, three
                        if (test_idx + 2 < line.len()) && &line[test_idx..test_idx + 3] == "two" {
                            break 2;
                        } else if (test_idx + 4 < line.len())
                            && &line[test_idx..test_idx + 5] == "three"
                        {
                            break 3;
                        }
                    }
                    'f' => {
                        // four, five
                        // println!("end_idx: {} len: {}", end_idx, line.len());
                        if (test_idx + 3) < line.len() {
                            if &line[test_idx..test_idx + 4] == "four" {
                                break 4;
                            } else if &line[test_idx..test_idx + 4] == "five" {
                                break 5;
                            }
                        }
                    }
                    's' => {
                        // six, seven
                        if (test_idx + 2) < line.len() {
                            if &line[test_idx..test_idx + 3] == "six" {
                                break 6;
                            } else if (test_idx + 4) < line.len()
                                && &line[test_idx..test_idx + 5] == "seven"
                            {
                                break 7;
                            }
                        }
                    }
                    'e' => {
                        // eight
                        if (test_idx + 4) < line.len() && &line[test_idx..test_idx + 5] == "eight" {
                            break 8;
                        }
                    }
                    'n' => {
                        // nine
                        if (test_idx + 3) < line.len() && &line[test_idx..test_idx + 4] == "nine" {
                            break 9;
                        }
                    }
                    'o' => {
                        // one
                        if (test_idx + 2 < line.len()) && &line[test_idx..test_idx + 3] == "one" {
                            break 1;
                        }
                    }
                    _ => {}
                }

                if test_idx == 0 {
                    panic!("No last digit found");
                }

                test_idx -= 1;
            };

            // println!("Digits: {}{}", first_digit.unwrap(), last_digit.unwrap());
            line_sum += last_digit as u64 * 10;
            sum += line_sum
        });

        println!("Sum: {}", sum);

        // 54249 is too high

        Ok(())
    }

    pub fn part_one_fast(&self) -> Result<(), DynError> {
        let f = File::open(self.input.as_path()).unwrap();
        let mut buffer = vec![];
        let mut reader = BufReader::new(f);
        reader.read_to_end(buffer.as_mut()).unwrap();
        // f.read_to_end(&mut buffer).unwrap();
        let file_length: usize = buffer.len();

        let mut sum: u32 = 0;
        let mut read_idx: usize = 0;
        let mut last_digit = 0;

        loop {
            // Find the first digit
            loop {
                if (buffer[read_idx] & 0b1000000) == 0 {
                    sum += (buffer[read_idx] as u32 - 48) * 10;
                    break;
                }
                read_idx += 1;
            }

            // Find the end of the line
            loop {
                if buffer[read_idx] == b'\n' {
                    break;
                }

                last_digit = if (buffer[read_idx] & 0b1000000) == 0 { buffer[read_idx] - 48 } else { last_digit };

                read_idx += 1;
            }

            sum += last_digit as u32;
            // last_digit = 0;
            read_idx += 1;
            if read_idx == file_length as usize {
                break;
            }
        }

        println!("Sum: {}", sum);

        Ok(())
    }

    pub fn part_two_fast(&self) -> Result<(), DynError> {
        let f = File::open(self.input.as_path()).unwrap();
        let mut buffer = vec![];
        let mut reader = BufReader::new(f);
        reader.read_to_end(buffer.as_mut()).unwrap();
        let file_length: usize = buffer.len();

        let mut sum: u32 = 0;
        let mut read_idx: usize = 0;

        loop {
            // Find the first digit
            loop {
                if (buffer[read_idx] & 0b1000000) == 0 {
                    sum += (buffer[read_idx] as u32 - 48) * 10;
                    break;
                } else if read_idx < file_length - 2 {
                    let test_slice = &buffer[read_idx..read_idx + 3];
                    let next_char = if read_idx < file_length - 3 { Some(buffer[read_idx + 3]) } else { None };
                    let next_two_chars = if read_idx < file_length - 4 { Some(&buffer[read_idx + 3..read_idx+5]) } else { None };
                    match test_slice {
                        b"one" => {
                            sum += 10;
                            read_idx += 3;
                            break;
                        },
                        b"two" => {
                            sum += 20;
                            read_idx += 3;
                            break;
                        },
                        b"thr" => 
                            if let Some(b"ee") = next_two_chars {
                                sum += 30;
                                read_idx += 5;
                                break;
                            }
                        ,
                        b"fou" => {
                            if let Some(b'r') = next_char {
                                sum += 40;
                                read_idx += 4;
                                break;
                            }
                        },
                        b"fiv" => {
                            if let Some(b'e') = next_char {
                                sum += 50;
                                read_idx += 4;
                                break;
                            }
                        },
                        b"six" => {
                            sum += 60;
                            read_idx += 3;
                            break;
                        },
                        b"sev" => {
                            if let Some(b"en") = next_two_chars {
                                sum += 70;
                                read_idx += 5;
                                break;
                            }
                        },
                        b"eig" => {
                            if let Some(b"ht") = next_two_chars {
                                sum += 80;
                                read_idx += 5;
                                break;
                            }
                        },
                        b"nin" => {
                            if let Some(b'e') = next_char {
                                sum += 90;
                                read_idx += 4;
                                break;
                            }
                        },
                        _ => {}
                    }
                } else {
                    panic!()
                }

                read_idx += 1;
            }

            // Find the end of the line
            loop {
                if buffer[read_idx] == b'\n' {
                    break;
                }
                read_idx += 1;
            }

            // Find the last digit
            let mut last_digit_idx = read_idx - 1;
            loop {
                if (buffer[last_digit_idx] & 0b1000000) == 0 {
                    sum += buffer[last_digit_idx] as u32 - 48;
                    break;
                } else if last_digit_idx < file_length - 2 {
                    let test_slice = &buffer[last_digit_idx..last_digit_idx + 3];
                    let next_char = if last_digit_idx < file_length - 3 { Some(buffer[last_digit_idx + 3]) } else { None };
                    let next_two_chars = if last_digit_idx < file_length - 4 { Some(&buffer[last_digit_idx + 3..last_digit_idx+5]) } else { None };
                    match test_slice {
                        b"one" => {
                            sum += 1;
                            break;
                        },
                        b"two" => {
                            sum += 2;
                            break;
                        },
                        b"thr" => 
                            if let Some(b"ee") = next_two_chars {
                                sum += 3;
                                break;
                            }
                        ,
                        b"fou" => {
                            if let Some(b'r') = next_char {
                                sum += 4;
                                break;
                            }
                        },
                        b"fiv" => {
                            if let Some(b'e') = next_char {
                                sum += 5;
                                break;
                            }
                        },
                        b"six" => {
                            sum += 6;
                            break;
                        },
                        b"sev" => {
                            if let Some(b"en") = next_two_chars {
                                sum += 7;
                                break;
                            }
                        },
                        b"eig" => {
                            if let Some(b"ht") = next_two_chars {
                                sum += 8;
                                break;
                            }
                        },
                        b"nin" => {
                            if let Some(b'e') = next_char {
                                sum += 9;
                                break;
                            }
                        },
                        _ => {}
                    }
                }
                last_digit_idx -= 1;
            }

            read_idx += 1;
            if read_idx == file_length as usize {
                break;
            }
        }

        println!("Sum: {}", sum);

        Ok(())
    }
}

impl CommandImpl for Day1 {
    fn main(&self) -> Result<(), DynError> {
        self.part_two()
    }
}
