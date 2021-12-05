use std::collections::HashMap;

use thiserror::Error;

use super::Result;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point((i32, i32));

#[derive(Debug, Error)]
enum ParsePointError {
    #[error("missing comma between x and y")]
    MissingComma,

    #[error("failed to parse x")]
    ParseX(#[source] std::num::ParseIntError),

    #[error("failed to parse y")]
    ParseY(#[source] std::num::ParseIntError),
}

impl std::str::FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParsePointError::MissingComma)?;
        Ok(Point((
            x.parse().map_err(ParsePointError::ParseX)?,
            y.parse().map_err(ParsePointError::ParseY)?,
        )))
    }
}

#[derive(Copy, Clone, Debug)]
struct LineSegment((Point, Point));

#[derive(Debug, Error)]
enum ParseLineSegmentError {
    #[error("missing point separator arrow `->`")]
    MissingArrow,

    #[error("failed to parse the first point")]
    ParsePointA(#[source] ParsePointError),

    #[error("failed to parse the second point")]
    ParsePointB(#[source] ParsePointError),
}

impl std::str::FromStr for LineSegment {
    type Err = ParseLineSegmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(" -> ")
            .ok_or(ParseLineSegmentError::MissingArrow)?;
        Ok(LineSegment((
            a.parse().map_err(ParseLineSegmentError::ParsePointA)?,
            b.parse().map_err(ParseLineSegmentError::ParsePointB)?,
        )))
    }
}

impl LineSegment {
    fn is_diagonal(&self) -> bool {
        let LineSegment((Point((x1, y1)), Point((x2, y2)))) = self;
        (x2 - x1).abs() == (y2 - y1).abs()
    }

    fn into_points(self) -> impl Iterator<Item = Point> {
        let LineSegment((Point((x1, y1)), Point((x2, y2)))) = self;
        match (x2 - x1, y2 - y1) {
            // Vertical line
            (0, dy) => {
                let sign = dy.signum();
                Box::new((0..=dy.abs()).map(move |d| Point((x1, y1 + d * sign))))
                    as Box<dyn Iterator<Item = Point>>
            }

            // Horizontal line
            (dx, 0) => {
                let sign = dx.signum();
                Box::new((0..=dx.abs()).map(move |d| Point((x1 + d * sign, y1))))
                    as Box<dyn Iterator<Item = Point>>
            }

            // Diagnoal line
            (dx, dy) if dx.abs() == dy.abs() => {
                let sign_x = dx.signum();
                let sign_y = dy.signum();
                Box::new((0..=dx.abs()).map(move |d| Point((x1 + d * sign_x, y1 + d * sign_y))))
                    as Box<dyn Iterator<Item = Point>>
            }

            (_, _) => Box::new(std::iter::empty()) as Box<dyn Iterator<Item = Point>>,
        }
    }
}

fn parse_line_segments(input: &str) -> Result<Vec<LineSegment>, ParseLineSegmentError> {
    input.lines().map(str::parse).collect()
}

fn count_overlaps(iter: impl Iterator<Item = Point>, n: usize) -> usize {
    iter.fold(HashMap::with_capacity(1000), |mut m, p| {
        *m.entry(p).or_insert(0) += 1;
        m
    })
    .into_values()
    .filter(|&c| c >= n)
    .count()
}

pub fn part1(input: &str) -> Result<usize> {
    let points = parse_line_segments(input)?
        .into_iter()
        .filter(|x| !x.is_diagonal())
        .flat_map(LineSegment::into_points);
    Ok(count_overlaps(points, 2))
}

pub fn part2(input: &str) -> Result<usize> {
    let points = parse_line_segments(input)?
        .into_iter()
        .flat_map(LineSegment::into_points);
    Ok(count_overlaps(points, 2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {"
            0,9 -> 5,9
            8,0 -> 0,8
            9,4 -> 3,4
            2,2 -> 2,1
            7,0 -> 7,4
            6,4 -> 2,0
            0,9 -> 2,9
            3,4 -> 1,4
            0,0 -> 8,8
            5,5 -> 8,2
        "};
        assert_eq!(part1(input).unwrap(), 5);
        assert_eq!(part2(input).unwrap(), 12);
    }
}
