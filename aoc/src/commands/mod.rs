pub mod day0;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5a;
pub mod day5b;
pub mod day6a;
pub mod day6b;
pub mod day7a;
pub mod day7b;
// pub mod day8a;
// pub mod day8b;
pub mod day9a;
pub mod day9b;
pub mod day10a;
pub mod day10b;
pub mod day11a;
pub mod day11b;
pub mod day12a;
pub mod day12b;
pub mod day13a;
pub mod day13b;
pub mod day14a;
pub mod day14b;
pub mod day15a;
pub mod day15b;
pub mod day16a;
pub mod day16b;

use std::error::Error;

use enum_dispatch::enum_dispatch;

pub type DynError = Box<dyn Error + 'static>;

#[enum_dispatch]
pub trait CommandImpl {
    fn main(&mut self) -> Result<(), DynError>;
}
