use std::path::PathBuf;

use clap::Parser;

use super::{CommandImpl, DynError};

#[derive(Parser, Debug)]
pub struct Day2 {
    #[clap(long, short)]
    input: PathBuf,
}

impl CommandImpl for Day2 {
    fn main(&self) -> Result<(), DynError> {
        println!("EX: {:?}", self.input);
        Ok(())
    }
}
