pub mod day0;
pub mod day1;
pub mod day2;
pub mod day3;

use std::error::Error;

use enum_dispatch::enum_dispatch;

pub type DynError = Box<dyn Error + 'static>;

#[enum_dispatch]
pub trait CommandImpl {
    fn main(&mut self) -> Result<(), DynError>;
}
