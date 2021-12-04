use std::{collections::HashSet, num::ParseIntError};

use thiserror::Error;

use super::Result;

#[derive(Debug, Error)]
enum Error {
    #[error("missing drawn numbers")]
    MissingDraws,

    #[error("missing boards")]
    MissingBoards,

    #[error("failed to parse drawn numbers")]
    ParseDraws(#[source] ParseIntError),

    #[error("failed to parse boards")]
    ParseBoard(#[source] ParseBoardError),
}

#[derive(Debug, Error)]
enum ParseBoardError {
    #[error("found {0} rows instead of 5")]
    InvalidRowCount(usize),

    #[error("found {1} columns at row {0} instead of 5")]
    InvalidColCount(usize, usize),

    #[error("failed to parse number")]
    ParseNumber(#[source] ParseIntError),

    #[error("found multiple {0} on board")]
    DuplicateNumber(u32),
}

#[derive(Debug, Default)]
struct Board {
    nums: [[u32; 5]; 5],
    marked: [[bool; 5]; 5],
    // Frequencies of marked rows and columns.
    // Bingo if any of them are 5, i.e, any row/column marked 5 times.
    rows: [u8; 5],
    cols: [u8; 5],
    // Sum of unmarked numbers
    sum: u32,
    score: Option<u32>,
}

impl Board {
    fn has_won(&self) -> bool {
        self.score.is_some()
    }

    /// Returns the score if `n` makes the board win.
    /// If the board is already completed, the recorded score is returned.
    /// Returns `None` otherwise.
    fn mark_number(&mut self, n: u32) -> Option<u32> {
        if self.has_won() {
            return self.score;
        }

        for i in 0..5 {
            for j in 0..5 {
                // Check marked to prevent duplicate `n` from affecting the frequency
                if self.nums[i][j] == n && !self.marked[i][j] {
                    self.marked[i][j] = true;
                    self.rows[i] += 1;
                    self.cols[j] += 1;
                    self.sum -= n;
                    if self.rows[i] == 5 || self.cols[j] == 5 {
                        self.score = Some(n * self.sum);
                        return self.score;
                    }
                }
            }
        }

        None
    }
}

impl std::str::FromStr for Board {
    type Err = ParseBoardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.split('\n').collect::<Vec<_>>();
        if rows.len() != 5 {
            return Err(ParseBoardError::InvalidRowCount(rows.len()));
        }

        let mut nums = [[0; 5]; 5];
        let mut sum = 0;
        let mut seen = HashSet::with_capacity(25);
        for (i, row) in rows.iter().enumerate() {
            let cols = row.split_ascii_whitespace();
            let count = cols.clone().count();
            if count != 5 {
                return Err(ParseBoardError::InvalidColCount(i, count));
            }

            for (j, col) in cols.enumerate() {
                let val = col.parse().map_err(ParseBoardError::ParseNumber)?;
                if !seen.insert(val) {
                    return Err(ParseBoardError::DuplicateNumber(val));
                }
                sum += val;
                nums[i][j] = val;
            }
        }

        Ok(Self {
            nums,
            sum,
            ..Self::default()
        })
    }
}

fn parse_draws_and_boards(input: &str) -> Result<(Vec<u32>, Vec<Board>), Error> {
    let mut inputs = input.trim_end().split("\n\n");
    let draws = inputs
        .next()
        .unwrap_or_default()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<u32>, ParseIntError>>()
        .map_err(Error::ParseDraws)?;
    if draws.is_empty() {
        return Err(Error::MissingDraws);
    }

    let boards = inputs
        .map(str::parse)
        .collect::<Result<Vec<Board>, ParseBoardError>>()
        .map_err(Error::ParseBoard)?;
    if boards.is_empty() {
        return Err(Error::MissingBoards);
    }
    Ok((draws, boards))
}

pub fn part1(input: &str) -> Result<String> {
    let (draws, mut boards) = parse_draws_and_boards(input)?;
    for n in draws {
        for board in &mut boards {
            if let Some(score) = board.mark_number(n) {
                return Ok(format!("{}", score));
            }
        }
    }
    Ok("no winner".to_owned())
}

pub fn part2(input: &str) -> Result<String> {
    let (draws, mut boards) = parse_draws_and_boards(input)?;
    let mut remaining = boards.len();
    let mut last_score = None;
    for n in draws {
        for board in &mut boards {
            if !board.has_won() {
                if let Some(score) = board.mark_number(n) {
                    last_score = Some(score);
                    remaining -= 1;
                }
            }
        }
        if remaining == 0 {
            break;
        }
    }
    if let Some(score) = last_score {
        Ok(format!("{}", score))
    } else {
        Ok("no winner".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {"
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
        "};
        assert_eq!(part1(input).unwrap(), "4512");
        assert_eq!(part2(input).unwrap(), "1924");
    }
}
