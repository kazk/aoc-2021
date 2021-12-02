#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

pub mod day01;
pub mod day02;
