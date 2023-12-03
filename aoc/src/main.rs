#[allow(unused)]
pub mod commands;
pub mod utils;

use commands::*;
use enum_dispatch::enum_dispatch;

use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[enum_dispatch(CommandImpl)]
#[derive(Parser, Debug)]
enum SubCommand {
    Day0(day0::Day0),
    Day1(day1::Day1),
    Day2(day2::Day2),
    Day3(day3::Day3),
}
fn main() -> Result<(), DynError> {
    let mut opts = Opts::parse();

    opts.subcommand.main()
}
