use std::collections::HashMap;

use thiserror::Error;

use super::Result;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: i32,
    y: i32,
}

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
        Ok(Point {
            x: x.parse().map_err(ParsePointError::ParseX)?,
            y: y.parse().map_err(ParsePointError::ParseY)?,
        })
    }
}

impl Point {
    fn move_by(self, dx: i32, dy: i32) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
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

    #[error("invalid segment: must be horizontal, vertical or 45 degrees diagnoal")]
    InvalidSegment,
}

impl std::str::FromStr for LineSegment {
    type Err = ParseLineSegmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(" -> ")
            .ok_or(ParseLineSegmentError::MissingArrow)?;
        let p1: Point = a.parse().map_err(ParseLineSegmentError::ParsePointA)?;
        let p2: Point = b.parse().map_err(ParseLineSegmentError::ParsePointB)?;
        let (dx, dy) = (p2.x - p1.x, p2.y - p1.y);
        if dx == 0 || dy == 0 || dx.abs() == dy.abs() {
            Ok(LineSegment((p1, p2)))
        } else {
            Err(ParseLineSegmentError::InvalidSegment)
        }
    }
}

impl LineSegment {
    fn is_diagonal(&self) -> bool {
        let LineSegment((p1, p2)) = self;
        (p2.x - p1.x).abs() == (p2.y - p1.y).abs()
    }

    fn into_points(self) -> impl Iterator<Item = Point> {
        let LineSegment((p1, p2)) = self;
        let (dx, dy) = ((p2.x - p1.x).signum(), (p2.y - p1.y).signum());
        std::iter::successors(Some(p1), move |&p| (p != p2).then(|| p.move_by(dx, dy)))
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
