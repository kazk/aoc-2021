use super::Result;

pub fn part1(input: &str) -> Result<usize> {
    let mut points = 0;
    for line in input.lines() {
        points += check_line(line);
    }
    Ok(points)
}

fn error_points(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn check_line(line: &str) -> usize {
    let mut state = Vec::new();
    for c in line.chars() {
        if matches!(c, '(' | '[' | '{' | '<') {
            state.push(c);
        } else {
            match state.pop() {
                Some('(') => {
                    if c != ')' {
                        return error_points(c);
                    }
                }
                Some('[') => {
                    if c != ']' {
                        return error_points(c);
                    }
                }
                Some('{') => {
                    if c != '}' {
                        return error_points(c);
                    }
                }
                Some('<') => {
                    if c != '>' {
                        return error_points(c);
                    }
                }
                Some(_) => unreachable!(),
                None => {
                    return error_points(c);
                }
            }
        }
    }
    0
}

pub fn part2(input: &str) -> Result<usize> {
    let mut scores = Vec::new();
    for line in input.lines() {
        let p = check_line2(line);
        if p != 0 {
            scores.push(p);
        }
    }
    scores.sort_unstable();
    Ok(scores[scores.len() / 2])
}

fn completion_points(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

fn check_line2(line: &str) -> usize {
    let mut state = Vec::new();
    for c in line.chars() {
        if matches!(c, '(' | '[' | '{' | '<') {
            state.push(c);
        } else {
            match state.pop() {
                Some('(') => {
                    if c != ')' {
                        return 0;
                    }
                }
                Some('[') => {
                    if c != ']' {
                        return 0;
                    }
                }
                Some('{') => {
                    if c != '}' {
                        return 0;
                    }
                }
                Some('<') => {
                    if c != '>' {
                        return 0;
                    }
                }
                Some(c) => unreachable!("unexpected {}", c),
                None => {
                    return 0;
                }
            }
        }
    }
    let mut score = 0;
    for c in state.iter().copied().rev() {
        let v = match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            c => unreachable!("unexpected {}", c),
        };
        score = 5 * score + completion_points(v);
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = indoc::indoc! {"
            [({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]
        "};
        assert_eq!(part1(input).unwrap(), 26397);
        assert_eq!(part2(input).unwrap(), 288_957);
    }
}
