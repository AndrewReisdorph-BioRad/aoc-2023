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
pub mod day8a;
pub mod day8b;

use std::error::Error;

use enum_dispatch::enum_dispatch;

pub type DynError = Box<dyn Error + 'static>;

#[enum_dispatch]
pub trait CommandImpl {
    fn main(&mut self) -> Result<(), DynError>;
}
