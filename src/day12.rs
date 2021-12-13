use std::collections::{BTreeMap, BTreeSet};

use super::Result;

fn parse_input(input: &str) -> BTreeMap<&str, BTreeSet<&str>> {
    input.lines().fold(BTreeMap::new(), |mut m, s| {
        let (a, b) = s.split_once('-').expect("separator `-`");
        m.entry(a).or_insert_with(BTreeSet::new).insert(b);
        m.entry(b).or_insert_with(BTreeSet::new).insert(a);
        m
    })
}

pub fn part1(input: &str) -> Result<usize> {
    let graph = parse_input(input);
    let mut current = Vec::new();
    let mut paths = Vec::new();
    let mut visited = BTreeMap::new();
    find_paths1(
        &graph,
        "start",
        "end",
        &mut current,
        &mut paths,
        &mut visited,
    );
    Ok(paths.len())
}

pub fn part2(input: &str) -> Result<usize> {
    let graph = parse_input(input);
    let mut current = Vec::new();
    let mut paths = Vec::new();
    let mut visited = BTreeMap::new();
    find_paths2(
        &graph,
        "start",
        "end",
        &mut current,
        &mut paths,
        &mut visited,
    );
    Ok(paths.len())
}

fn find_paths1<'a>(
    graph: &BTreeMap<&'a str, BTreeSet<&'a str>>,
    u: &'a str,
    v: &'a str,
    current: &mut Vec<&'a str>,
    paths: &mut Vec<Vec<&'a str>>,
    visited: &mut BTreeMap<&'a str, bool>,
) {
    let is_small = u.chars().all(|c| c.is_ascii_lowercase());
    if is_small {
        if *visited.get(u).unwrap_or(&false) {
            return;
        }
        visited.insert(u, true);
    }
    current.push(u);

    if u == v {
        paths.push(current.clone());
    } else if let Some(connected) = graph.get(u) {
        for next in connected {
            find_paths1(graph, next, v, current, paths, visited);
        }
    }

    current.pop();
    if is_small {
        visited.insert(u, false);
    }
}

fn find_paths2<'a>(
    graph: &BTreeMap<&'a str, BTreeSet<&'a str>>,
    u: &'a str,
    v: &'a str,
    current: &mut Vec<&'a str>,
    paths: &mut Vec<Vec<&'a str>>,
    visited: &mut BTreeMap<&'a str, usize>,
) {
    let is_small = u.chars().all(|c| c.is_ascii_lowercase());
    if is_small {
        if *visited.get(u).unwrap_or(&0) != 0 {
            if u == "start" || u == "end" {
                return;
            }
            if visited.values().any(|&v| v > 1) {
                return;
            }
        }

        *visited.entry(u).or_insert(0) += 1;
    }
    current.push(u);

    if u == v {
        paths.push(current.clone());
    } else if let Some(connected) = graph.get(u) {
        for next in connected {
            find_paths2(graph, next, v, current, paths, visited);
        }
    }

    current.pop();
    if is_small {
        *visited.get_mut(u).unwrap() -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = indoc::indoc! {"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        "};
        assert_eq!(part1(input).unwrap(), 10);
        assert_eq!(part2(input).unwrap(), 36);
    }

    #[test]
    fn test_example2() {
        let input = indoc::indoc! {"
            dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc
        "};
        assert_eq!(part1(input).unwrap(), 19);
        assert_eq!(part2(input).unwrap(), 103);
    }

    #[test]
    fn test_example3() {
        let input = indoc::indoc! {"
            fs-end
            he-DX
            fs-he
            start-DX
            pj-DX
            end-zg
            zg-sl
            zg-pj
            pj-he
            RW-he
            fs-DX
            pj-RW
            zg-RW
            start-pj
            he-WI
            zg-he
            pj-fs
            start-RW
        "};
        assert_eq!(part1(input).unwrap(), 226);
        assert_eq!(part2(input).unwrap(), 3509);
    }
}
