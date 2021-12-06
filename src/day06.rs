use thiserror::Error;

use super::Result;

#[derive(Debug, Error)]
enum Error {
    #[error("failed to parse timer")]
    ParseTimer(#[source] std::num::ParseIntError),

    #[error("timer with value {0} is invalid")]
    InvalidTimer(u8),
}

fn parse_initial_state(input: &str) -> Result<[usize; 9], Error> {
    let vec = input
        .trim_end()
        .split(',')
        .map(|s| {
            let t = s.parse().map_err(Error::ParseTimer)?;
            if t > 8 {
                Err(Error::InvalidTimer(t))
            } else {
                Ok(t)
            }
        })
        .collect::<Result<Vec<u8>, _>>()?;
    Ok(vec.iter().fold([0; 9], |mut state, &t| {
        state[t as usize] += 1;
        state
    }))
}

//  0  1  2  3  4  5  6  7  8
// [0, 0, 0, 0, 0, 0, 0, 0, 0]
// [0, 1, 1, 2, 1, 0, 0, 0, 0] 3, 4, 3, 1, 2
// [1, 1, 2, 1, 0, 0, 0, 0, 0] 2, 3, 2, 0, 1
// [1, 2, 1, 0, 0, 0, 1, 0, 1] 1, 2, 1, 6, 0, 8
// [2, 1, 0, 0, 0, 1, 1, 1, 1] 0, 1, 0, 5, 6, 7, 8
// [1, 0, 0, 0, 1, 1, 3, 1, 2] 6, 0, 6, 4, 5, 6, 7, 8, 8
fn simulate_days(mut state: [usize; 9], days: usize) -> [usize; 9] {
    for _ in 0..days {
        // current 0 to 6 and 8
        state.rotate_left(1);
        state[6] += state[8];
    }
    state
}

pub fn part1(input: &str) -> Result<usize> {
    let state = parse_initial_state(input)?;
    Ok(simulate_days(state, 80).iter().sum())
    // Ok(count_after_days(state, 80))
}

pub fn part2(input: &str) -> Result<usize> {
    let state = parse_initial_state(input)?;
    Ok(simulate_days(state, 256).iter().sum())
    // Ok(count_after_days(state, 256))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = indoc! {"
            3,4,3,1,2
        "};
        assert_eq!(part1(input).unwrap(), 5934);
        assert_eq!(part2(input).unwrap(), 26_984_457_539);
    }
}

// Original solution that counts the number of fish recursively with memoization
#[allow(dead_code)]
fn count_after_days(state: [usize; 9], days: usize) -> usize {
    state
        .into_iter()
        .enumerate()
        .fold(0, |s, (t, c)| s + c * count0(days.saturating_sub(t)))
}

#[allow(dead_code)]
// The number of lanternfish after `days` starting from one with timer `0`.
fn count0(days: usize) -> usize {
    use once_cell::sync::Lazy;
    use std::{collections::BTreeMap, sync::Mutex};
    static MEMO: Lazy<Mutex<BTreeMap<usize, usize>>> = Lazy::new(|| Mutex::new(BTreeMap::new()));

    fn inner(days: usize) -> usize {
        if days == 0 {
            1
        } else {
            count0(days.saturating_sub(7)) + count0(days.saturating_sub(9))
        }
    }

    if let Some(&cached) = MEMO.lock().unwrap().get(&days) {
        return cached;
    }

    let res = inner(days);
    MEMO.lock().unwrap().insert(days, res);
    res
}
