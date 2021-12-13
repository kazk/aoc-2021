use std::collections::{BTreeSet, VecDeque};

use super::Result;

pub fn part1(input: &str) -> Result<u32> {
    let heights: Vec<Vec<u32>> = input
        .lines()
        .map(|r| {
            r.chars()
                .map(|c| c.to_digit(10).expect("digit"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = heights.len();
    let width = heights[0].len();
    let mut score = 0;
    for i in 0..height {
        for j in 0..width {
            let v = heights[i][j];
            if i > 0 && v >= heights[i - 1][j] {
                continue;
            }
            if i + 1 < height && v >= heights[i + 1][j] {
                continue;
            }
            if j > 0 && v >= heights[i][j - 1] {
                continue;
            }
            if j + 1 < width && v >= heights[i][j + 1] {
                continue;
            }
            score += v + 1;
        }
    }

    Ok(score)
}

pub fn part2(input: &str) -> Result<usize> {
    let heights: Vec<Vec<u32>> = input
        .lines()
        .map(|r| {
            r.chars()
                .map(|c| c.to_digit(10).expect("digit"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = heights.len();
    let width = heights[0].len();
    let mut basins = Vec::new();
    for i in 0..height {
        for j in 0..width {
            let v = heights[i][j];
            if i > 0 && v >= heights[i - 1][j] {
                continue;
            }
            if i + 1 < height && v >= heights[i + 1][j] {
                continue;
            }
            if j > 0 && v >= heights[i][j - 1] {
                continue;
            }
            if j + 1 < width && v >= heights[i][j + 1] {
                continue;
            }
            basins.push(basin_size(i, j, height, width, &heights));
        }
    }
    basins.sort_by(|a, b| b.cmp(a));
    Ok(basins.iter().take(3).product())
}

fn basin_size(
    start_i: usize,
    start_j: usize,
    height: usize,
    width: usize,
    map: &[Vec<u32>],
) -> usize {
    let mut visited = BTreeSet::from([(start_i, start_j)]);
    let mut queue = VecDeque::from([(start_i, start_j)]);
    let mut size = 0;
    while !queue.is_empty() {
        let (i, j) = queue.pop_front().unwrap();
        size += 1;
        if i + 1 < height
            && map[i + 1][j] != 9
            && map[i][j] < map[i + 1][j]
            && !visited.contains(&(i + 1, j))
        {
            queue.push_back((i + 1, j));
            visited.insert((i + 1, j));
        }
        if i > 0
            && map[i - 1][j] != 9
            && map[i][j] < map[i - 1][j]
            && !visited.contains(&(i - 1, j))
        {
            queue.push_back((i - 1, j));
            visited.insert((i - 1, j));
        }
        if j + 1 < width
            && map[i][j + 1] != 9
            && map[i][j] < map[i][j + 1]
            && !visited.contains(&(i, j + 1))
        {
            queue.push_back((i, j + 1));
            visited.insert((i, j + 1));
        }
        if j > 0
            && map[i][j - 1] != 9
            && map[i][j] < map[i][j - 1]
            && !visited.contains(&(i, j - 1))
        {
            queue.push_back((i, j - 1));
            visited.insert((i, j - 1));
        }
    }

    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = indoc::indoc! {"
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        "};
        assert_eq!(part1(input).unwrap(), 15);
        assert_eq!(part2(input).unwrap(), 1134);
    }
}
