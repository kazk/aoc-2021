use std::collections::BTreeMap;

use super::Result;

pub fn part1(input: &str) -> Result<usize> {
    let outs = input
        .lines()
        .map(|s| {
            let (_, out) = s.split_once(" | ").expect("separator `|`");
            out.split(' ').collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Ok(outs
        .iter()
        .map(|b| {
            b.iter()
                .filter(|s| matches!(s.len(), 2 | 4 | 3 | 7))
                .count()
        })
        .sum())
}

pub fn part2(input: &str) -> Result<u32> {
    let pairs = input
        .lines()
        .map(|s| {
            let (sigs, out) = s.split_once(" | ").expect("separator `|`");
            let sigs = sigs.split(' ').collect::<Vec<_>>();
            let out = out.split(' ').collect::<Vec<_>>();
            (sigs, out)
        })
        .collect::<Vec<_>>();

    Ok(pairs.iter().map(|(a, b)| decode_output(a, b)).sum())
}

fn decode_output(sigs: &[&str], outs: &[&str]) -> u32 {
    let seg_mapping = find_mapping(sigs);

    let values = [
        ("abcefg", '0'),
        ("cf", '1'),
        ("acdeg", '2'),
        ("acdfg", '3'),
        ("bcdf", '4'),
        ("abdfg", '5'),
        ("abdefg", '6'),
        ("acf", '7'),
        ("abcdefg", '8'),
        ("abcdfg", '9'),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_owned(), v))
    .collect::<BTreeMap<String, _>>();

    let out = outs
        .iter()
        .copied()
        .map(|o| {
            let mut chars = o
                .chars()
                .map(|c| seg_mapping.get(&c).expect("valid character [a-g]"))
                .collect::<Vec<_>>();
            chars.sort();
            values
                .get(&chars.into_iter().collect::<String>())
                .expect("valid combination")
        })
        .collect::<String>();

    out.parse().unwrap()
}

fn find_mapping(sigs: &[&str]) -> BTreeMap<char, char> {
    // 1. Identify some digits with unique length:
    //    - '1' is 2 (c, f)
    //    - '7' is 3 (a, c, f)
    //    - '4' is 4 (b, c, d, f)
    //    - '8' is 7 (a, b, c, d, e, f, g)
    let dig_1 = *sigs.iter().find(|s| s.len() == 2).expect("digit '1'");
    let dig_7 = *sigs.iter().find(|s| s.len() == 3).expect("digit '7'");
    let dig_4 = *sigs.iter().find(|s| s.len() == 4).expect("digit '4'");
    let dig_8 = *sigs.iter().find(|s| s.len() == 7).expect("digit '8'");

    // 2. Identify segments 'b', 'e', 'f' by their unique frequencies:
    //    - 'b' is on 6 times
    //    - 'e' is on 4 times
    //    - 'f' is on 9 times
    let freqs = sigs.iter().fold(BTreeMap::new(), |m, s| {
        s.chars().fold(m, |mut m, c| {
            *m.entry(c).or_insert(0) += 1;
            m
        })
    });
    let seg_b = freqs
        .iter()
        .find_map(|(&k, &v)| (v == 6).then(|| k))
        .expect("segment 'b'");
    let seg_e = freqs
        .iter()
        .find_map(|(&k, &v)| (v == 4).then(|| k))
        .expect("segment 'e'");
    let seg_f = freqs
        .iter()
        .find_map(|(&k, &v)| (v == 9).then(|| k))
        .expect("segment 'f'");

    // 3. With 'f' defined, 'c' is the other segment in '1' (not f)
    let seg_c = dig_1.chars().find(|&c| c != seg_f).expect("segment 'c'");

    // 4. With 'c' and 'f' defined, 'a' is the last unknown in '7' (not c, f)
    let seg_a = dig_7
        .chars()
        .find(|&c| c != seg_c && c != seg_f)
        .expect("segment 'a'");

    // 5. 'd' is the last unknown in '4' (not b, c, f)
    let seg_d = dig_4
        .chars()
        .find(|&c| c != seg_b && c != seg_c && c != seg_f)
        .expect("segment 'd'");

    // 6. 'g' is the last unknown in '8' (not a, b, c, d, e, f)
    let seg_g = dig_8
        .chars()
        .find(|&c| c != seg_a && c != seg_b && c != seg_c && c != seg_d && c != seg_e && c != seg_f)
        .expect("segment 'g'");

    BTreeMap::from([
        (seg_a, 'a'),
        (seg_b, 'b'),
        (seg_c, 'c'),
        (seg_d, 'd'),
        (seg_e, 'e'),
        (seg_f, 'f'),
        (seg_g, 'g'),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = indoc::indoc! {"
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "};
        assert_eq!(part1(input).unwrap(), 26);
        assert_eq!(part2(input).unwrap(), 61229);
    }
}
