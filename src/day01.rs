use super::Result;

fn parse_depths(input: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    input.lines().map(str::parse).collect()
}

fn increased_count<T: PartialOrd>(iter: impl Iterator<Item = T> + Clone) -> usize {
    iter.clone()
        .zip(iter.skip(1))
        .filter(|(a, b)| a < b)
        .count()
}

/// # Errors
///
/// Will return `Err` if the input contains an invalid integer.
pub fn part1(input: &str) -> Result<()> {
    let depths = parse_depths(input)?;
    let n = increased_count(depths.iter());
    println!("{}", n);
    Ok(())
}

/// # Errors
///
/// Will return `Err` if the input contains an invalid integer.
pub fn part2(input: &str) -> Result<()> {
    let depths = parse_depths(input)?;
    // sliding window of 3 measurements summed
    let sum3 = depths
        .iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
        .map(|((a, b), c)| a + b + c);
    let n = increased_count(sum3);
    println!("{}", n);
    Ok(())
}
