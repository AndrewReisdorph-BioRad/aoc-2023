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
    Day4(day4::Day4),
    Day5a(day5a::Day5a),
    Day5b(day5b::Day5b),
    Day6a(day6a::Day6a),
    Day6b(day6b::Day6b),
    Day7a(day7a::Day7a),
    Day7b(day7b::Day7b),
}
fn main() -> Result<(), DynError> {
    let mut opts = Opts::parse();

    opts.subcommand.main()
}
