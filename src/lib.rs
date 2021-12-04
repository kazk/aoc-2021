#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
