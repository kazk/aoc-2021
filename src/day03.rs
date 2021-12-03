use std::collections::BTreeMap;

use super::Result;

pub fn part1(input: &str) -> Result<()> {
    let diagnostics = input.lines().collect::<Vec<_>>();
    debug_assert!(!diagnostics.is_empty());
    let width = diagnostics[0].len();

    let gamma = diagnostics
        .iter()
        .flat_map(|bits| bits.chars().enumerate())
        .fold(BTreeMap::<usize, i32>::new(), |mut m, (i, c)| {
            *m.entry(i).or_insert(0) += if c == '1' { 1 } else { -1 };
            m
        })
        .into_values()
        .fold(0, |g, v| (g << 1) + (if v < 0 { 0 } else { 1 }));
    let mask: u32 = (1 << width) - 1;
    let epsilon = !gamma & mask;
    let n = gamma * epsilon;
    println!("{}", n);

    Ok(())
}

pub fn part2(input: &str) -> Result<()> {
    let diagnostics = input.lines().collect::<Vec<_>>();
    debug_assert!(!diagnostics.is_empty());
    let width = diagnostics[0].len();
    let mut oxy = diagnostics.clone();
    let mut co2 = diagnostics;
    for i in 0..width {
        if oxy.len() > 1 {
            let (a, b): (Vec<_>, Vec<_>) = oxy.into_iter().partition(|s| s.as_bytes()[i] == b'1');
            oxy = if a.len() >= b.len() { a } else { b };
        }
        if co2.len() > 1 {
            let (a, b): (Vec<_>, Vec<_>) = co2.into_iter().partition(|s| s.as_bytes()[i] == b'0');
            co2 = if a.len() <= b.len() { a } else { b };
        }
    }

    let oxy = u32::from_str_radix(oxy[0], 2)?;
    let co2 = u32::from_str_radix(co2[0], 2)?;
    let n = oxy * co2;
    println!("{}", n);

    Ok(())
}
