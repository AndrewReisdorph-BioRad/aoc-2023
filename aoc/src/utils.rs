use std::{
    error::Error,
    fmt::{self, Debug},
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct SlurpError {
    line: usize,
    msg: String,
}

impl fmt::Display for SlurpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error at line {}: {}", self.line, self.msg)
    }
}

impl Error for SlurpError {}

pub fn slurp_bytes<P>(path: P) -> Result<Vec<u8>, SlurpError>
where
    P: AsRef<Path>,
{
    let mut reader = BufReader::new(File::open(path).expect("Failed to open file"));
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).map_err(|e| SlurpError { line: 0, msg: e.to_string() })?;
    Ok(bytes)
}

/// Slurp file will try to parse the string into `T` as long as T implements FromStr
#[allow(clippy::missing_errors_doc)]
pub fn slurp_file<P, T>(path: P) -> Result<Vec<T>, SlurpError>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as FromStr>::Err: Error,
{
    let reader = File::open(&path).map(BufReader::new).expect("Failed to open file");
    let mut result = vec![];
    for (i, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| SlurpError { line: i, msg: e.to_string() })?;
        result.push(line.parse::<T>().map_err(|e| SlurpError { line: i, msg: e.to_string() })?);
    }
    Ok(result)
}

#[derive(Debug, Clone)]
pub struct ParseError {
    msg: String,
}
impl ParseError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error for command: {}", self.msg)
    }
}

pub struct AsciiReader {
    pub buffer: Vec<u8>,
    pub index: usize,
}

impl AsciiReader {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self { buffer, index: 0 }
    }

    pub fn read_next_number(&mut self) -> Option<i64> {
        if self.index >= self.buffer.len() || self.buffer[self.index] == b'\n' {
            return None;
        }

        while self.buffer[self.index] == b' ' {
            self.index += 1;
        }

        let is_negative = if self.buffer[self.index] == b'-' {
            self.index += 1;
            true
        } else {
            false
        };

        let mut num: i64 = 0;
        while self.buffer[self.index].is_ascii_digit() {
            num *= 10;
            num += (self.buffer[self.index] - b'0') as i64;
            self.index += 1;
        }

        while self.buffer[self.index] == b' ' || self.buffer[self.index] == b',' {
            self.index += 1;
        }

        if is_negative {
            num = num.wrapping_neg();
        }
        // *read_idx += 1;
        Some(num)
    }

    pub fn read_line(&mut self) -> Option<&[u8]> {
        self.read_to(b'\n')
    }

    pub fn read_to(&mut self, char: u8) -> Option<&[u8]> {
        if self.index >= self.buffer.len() {
            return None;
        }

        let start = self.index;
        while (self.index < self.buffer.len()) && self.buffer[self.index] != char && self.buffer[self.index] != b'\n'{
            self.index += 1;
        }
        let end = self.index;

        self.index += 1;

        Some(&self.buffer[start..end])
    }

    pub fn read_until(&mut self, char: u8) -> Option<&[u8]> {
        if self.index >= self.buffer.len() {
            return None;
        }

        let start = self.index;
        while self.buffer[self.index] != char {
            self.index += 1;
        }
        let end = self.index;

        Some(&self.buffer[start..end])
    }

    pub fn skip(&mut self, amount: usize) {
        self.index = (self.index + amount).min(self.buffer.len());
    }

    pub fn eof(&mut self) -> bool {
        self.index >= self.buffer.len()
    }

    pub fn next(&mut self, amount: usize) -> &[u8] {
        let start = self.index;
        let end = start + amount;
        self.index = end;

        &self.buffer[start..end]
    }

    pub fn seek(&mut self, position: usize) {
        self.index = position;
    }

    pub fn at(&self, position: usize) -> u8 {
        self.buffer[position]
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}