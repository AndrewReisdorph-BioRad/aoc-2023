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
    // Day8a(day8a::Day8a),
    // Day8b(day8b::Day8b),
    Day9a(day9a::Day9a),
    Day9b(day9b::Day9b),
    Day10a(day10a::Day10a),
    Day10b(day10b::Day10b),
    Day11a(day11a::Day11a),
    Day11b(day11b::Day11b),
    Day12a(day12a::Day12a),
    Day12b(day12b::Day12b),
    Day13a(day13a::Day13a),
    Day13b(day13b::Day13b),
    Day14a(day14a::Day14a),
    Day14b(day14b::Day14b),
}
fn main() -> Result<(), DynError> {
    let mut opts = Opts::parse();

    opts.subcommand.main()
}
