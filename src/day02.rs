use thiserror::Error;

use super::Result;

#[derive(Debug, Error)]
enum ParseCommandError {
    #[error("missing command name")]
    MissingName,

    #[error("missing command value")]
    MissingValue,

    #[error("failed to parse value")]
    ParseValue(#[source] std::num::ParseIntError),

    #[error("unknown command: {0}")]
    UnknownCommand(String),
}

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl std::str::FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pair = s.splitn(2, ' ');
        let cmd = pair.next().ok_or(ParseCommandError::MissingName)?;
        let val = pair.next().ok_or(ParseCommandError::MissingValue)?;
        let val = val.parse::<i32>().map_err(ParseCommandError::ParseValue)?;
        match cmd {
            "forward" => Ok(Self::Forward(val)),
            "up" => Ok(Self::Up(val)),
            "down" => Ok(Self::Down(val)),
            _ => Err(ParseCommandError::UnknownCommand(cmd.to_owned())),
        }
    }
}

fn parse_commands(input: &str) -> Result<Vec<Command>, ParseCommandError> {
    input.lines().map(str::parse).collect()
}

/// # Errors
///
/// Will return `Err` if the input contains an invalid command.
pub fn part1(input: &str) -> Result<()> {
    let mut h = 0;
    let mut v = 0;
    for cmd in parse_commands(input)? {
        match cmd {
            Command::Forward(n) => h += n,
            Command::Down(n) => v += n,
            Command::Up(n) => v -= n,
        }
    }
    println!("{}", h * v);
    Ok(())
}

/// # Errors
///
/// Will return `Err` if the input contains an invalid command.
pub fn part2(input: &str) -> Result<()> {
    let mut h = 0;
    let mut v = 0;
    let mut aim = 0;
    for cmd in parse_commands(input)? {
        match cmd {
            Command::Forward(n) => {
                h += n;
                v += aim * n;
            }
            Command::Down(n) => aim += n,
            Command::Up(n) => aim -= n,
        }
    }
    println!("{}", h * v);
    Ok(())
}
