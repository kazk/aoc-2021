use super::Result;

pub fn part1(input: &str) -> Result<u32> {
    Ok(min_cost_by(&parse_input(input)?, |x| x))
}

pub fn part2(input: &str) -> Result<u32> {
    Ok(min_cost_by(&parse_input(input)?, |x| x * (x + 1) / 2))
}

fn parse_input(input: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    input
        .trim_end()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<u32>, _>>()
}

fn min_cost_by(positions: &[u32], cost_fn: impl Fn(u32) -> u32) -> u32 {
    let max = *positions.iter().max().expect("nonempty input");
    (0..=max)
        .map(|q| {
            positions
                .iter()
                .copied()
                .map(|p| cost_fn(if q < p { p - q } else { q - p }))
                .sum()
        })
        .min()
        .expect("nonempty input")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {"
            16,1,2,0,4,2,7,1,2,14
        "};
        assert_eq!(part1(input).unwrap(), 37);
        assert_eq!(part2(input).unwrap(), 168);
    }
}
