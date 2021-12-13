use std::collections::BTreeSet;

use thiserror::Error;

use super::Result;

pub fn part1(input: &str) -> Result<usize> {
    let (dots, folds) = parse_input(input)?;
    let dots = folds
        .iter()
        .take(1)
        .fold(dots, |dots, fold| fold_paper(&dots, fold));
    Ok(dots.iter().copied().collect::<BTreeSet<_>>().len())
}

pub fn part2(input: &str) -> Result<String> {
    let (dots, folds) = parse_input(input)?;
    let dots = folds
        .iter()
        .fold(dots, |dots, fold| fold_paper(&dots, fold));
    Ok(to_paper(&dots))
}

#[derive(Debug, Error)]
enum Error {
    #[error("failed parse dots")]
    ParseDots(#[source] ParseDotError),

    #[error("failed parse fold instructions")]
    ParseFolds(#[source] ParseFoldError),
}

#[derive(Debug, Error)]
enum ParseFoldError {
    #[error("missing prefix `fold along`")]
    MissingPrefix,

    #[error("missing fold direction x or y")]
    MissingDirection,

    #[error("failed to parse int")]
    ParseInt(#[source] std::num::ParseIntError),
}

#[derive(Debug, Error)]
enum ParseDotError {
    #[error("missing comma")]
    MissingComma,

    #[error("failed to parse x")]
    ParseX(#[source] std::num::ParseIntError),

    #[error("failed to parse y")]
    ParseY(#[source] std::num::ParseIntError),
}

type Dot = (usize, usize);

#[derive(Debug, Copy, Clone)]
enum Fold {
    /// Fold along a horizontal line on y
    Up(usize),
    /// Fold along a vertical line on x
    Left(usize),
}

impl std::str::FromStr for Fold {
    type Err = ParseFoldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("fold along ")
            .ok_or(ParseFoldError::MissingPrefix)?;

        if let Some(s) = s.strip_prefix("y=") {
            Ok(Fold::Up(s.parse().map_err(ParseFoldError::ParseInt)?))
        } else if let Some(s) = s.strip_prefix("x=") {
            Ok(Fold::Left(s.parse().map_err(ParseFoldError::ParseInt)?))
        } else {
            Err(ParseFoldError::MissingDirection)
        }
    }
}

fn parse_input(input: &str) -> Result<(Vec<Dot>, Vec<Fold>), Error> {
    let (dots, folds) = input.trim_end().split_once("\n\n").unwrap();
    let dots = parse_dots(dots).map_err(Error::ParseDots)?;
    let folds = folds
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Fold>, ParseFoldError>>()
        .map_err(Error::ParseFolds)?;
    Ok((dots, folds))
}

fn parse_dots(input: &str) -> Result<Vec<Dot>, ParseDotError> {
    input
        .lines()
        .map(|s| {
            let (x, y) = s.split_once(',').ok_or(ParseDotError::MissingComma)?;
            Ok((
                x.parse().map_err(ParseDotError::ParseX)?,
                y.parse().map_err(ParseDotError::ParseY)?,
            ))
        })
        .collect()
}

fn fold_paper(dots: &[Dot], fold: &Fold) -> Vec<Dot> {
    match *fold {
        Fold::Up(line_y) => dots
            .iter()
            .copied()
            .map(|(x, y)| {
                if y > line_y {
                    (x, 2 * line_y - y)
                } else {
                    (x, y)
                }
            })
            .collect(),
        Fold::Left(line_x) => dots
            .iter()
            .copied()
            .map(|(x, y)| {
                if x > line_x {
                    (2 * line_x - x, y)
                } else {
                    (x, y)
                }
            })
            .collect(),
    }
}

fn to_paper(dots: &[Dot]) -> String {
    let dots = dots.iter().copied().collect::<BTreeSet<_>>();
    let max_x = dots.iter().copied().map(|(x, _)| x).max().expect("max x");
    let max_y = dots.iter().copied().map(|(_, y)| y).max().expect("max y");
    (0..=max_y)
        .map(|j| {
            (0..=max_x)
                .map(|i| if dots.contains(&(i, j)) { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = indoc::indoc! {"
            6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0

            fold along y=7
            fold along x=5
        "};
        let paper = indoc::indoc! {"
            #####
            #...#
            #...#
            #...#
            #####
        "};
        assert_eq!(part1(input).unwrap(), 17);
        assert_eq!(part2(input).unwrap(), paper.trim_end());
    }
}
