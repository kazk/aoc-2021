use std::collections::VecDeque;

use super::Result;

struct Octopus {
    energy: u32,
    flashed_at: u32,
}

impl Octopus {
    fn new(energy: u32) -> Self {
        Octopus {
            energy,
            flashed_at: 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Octopus>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Octopus::new(c.to_digit(10).expect("digit")))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> Result<usize> {
    let mut octos = parse_input(input);
    let h = octos.len();
    let w = octos[0].len();
    Ok((0..100).fold(0, |s, i| s + step(&mut octos, i + 1, h, w)))
}

pub fn part2(input: &str) -> Result<u32> {
    let mut octos = parse_input(input);
    let h = octos.len();
    let w = octos[0].len();
    let all = h * w;
    let mut i = 0;
    loop {
        if step(&mut octos, i + 1, h, w) == all {
            return Ok(i + 1);
        }
        i += 1;
    }
}

fn step(octos: &mut Vec<Vec<Octopus>>, step: u32, height: usize, width: usize) -> usize {
    for row in octos.iter_mut() {
        for octo in row {
            octo.energy += 1;
        }
    }
    let mut count = 0;
    for i in 0..height {
        for j in 0..width {
            if octos[i][j].energy > 9 {
                count += flash(octos, i, j, height, width, step);
            }
        }
    }
    count
}

fn flash(
    octos: &mut Vec<Vec<Octopus>>,
    start_i: usize,
    start_j: usize,
    height: usize,
    width: usize,
    step: u32,
) -> usize {
    let mut queue = VecDeque::from([(start_i, start_j)]);
    let mut count = 0;
    while !queue.is_empty() {
        let (i, j) = queue.pop_front().expect("not empty");
        if octos[i][j].flashed_at == step {
            continue;
        }
        // Continue if this doesn't flash
        if octos[i][j].energy <= 9 {
            continue;
        }

        // Flashing affects neighbors
        octos[i][j].energy = 0;
        octos[i][j].flashed_at = step;
        count += 1;
        // bottom
        if i + 1 < height && octos[i + 1][j].flashed_at != step {
            octos[i + 1][j].energy += 1;
            queue.push_back((i + 1, j));
        }
        // bottom right
        if i + 1 < height && j + 1 < width && octos[i + 1][j + 1].flashed_at != step {
            octos[i + 1][j + 1].energy += 1;
            queue.push_back((i + 1, j + 1));
        }
        // bottom left
        if i + 1 < height && j > 0 && octos[i + 1][j - 1].flashed_at != step {
            octos[i + 1][j - 1].energy += 1;
            queue.push_back((i + 1, j - 1));
        }
        // top
        if i > 0 && octos[i - 1][j].flashed_at != step {
            octos[i - 1][j].energy += 1;
            queue.push_back((i - 1, j));
        }
        // top right
        if i > 0 && j + 1 < width && octos[i - 1][j + 1].flashed_at != step {
            octos[i - 1][j + 1].energy += 1;
            queue.push_back((i - 1, j + 1));
        }
        // top left
        if i > 0 && j > 0 && octos[i - 1][j - 1].flashed_at != step {
            octos[i - 1][j - 1].energy += 1;
            queue.push_back((i - 1, j - 1));
        }
        // right
        if j + 1 < width && octos[i][j + 1].flashed_at != step {
            octos[i][j + 1].energy += 1;
            queue.push_back((i, j + 1));
        }
        // left
        if j > 0 && octos[i][j - 1].flashed_at != step {
            octos[i][j - 1].energy += 1;
            queue.push_back((i, j - 1));
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = indoc::indoc! {"
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        "};
        assert_eq!(part1(input).unwrap(), 1656);
        assert_eq!(part2(input).unwrap(), 195);
    }
}
