use std::io::Read;

use aoc2021::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    println!("running part 1");
    let timer = took::Timer::new();
    println!("{}", aoc2021::day13::part1(&input)?);
    println!("took {}", timer.took());

    println!("running part 2");
    let timer = took::Timer::new();
    println!("{}", aoc2021::day13::part2(&input)?);
    println!("took {}", timer.took());

    Ok(())
}
