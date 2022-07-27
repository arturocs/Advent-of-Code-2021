fn check_syntax(line: &str, stack: &mut Vec<char>) -> Option<char> {
    for c in line.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' => {
                if let Some(expected) = stack.pop() {
                    if expected != c {
                        return Some(c);
                    }
                } else {
                    return None;
                }
            }
            _ => {}
        }
    }
    None
}

fn expected_character(line: &str) -> Option<char> {
    let mut stack = vec![];
    check_syntax(line, &mut stack)
}

fn autocomplete_missing_part(line: &str) -> Option<Vec<char>> {
    let mut stack = vec![];
    let expected_char = check_syntax(line, &mut stack);
    (stack.len() > 0 && expected_char.is_none()).then(|| stack)
}

fn char_to_points_part_1(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}
fn char_to_points_part_2(c: char) -> i64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

fn main() {
    let input: Vec<_> = include_str!("../input").lines().collect();
    let result_1: i64 = input
        .iter()
        .cloned()
        .filter_map(expected_character)
        .map(char_to_points_part_1)
        .sum();
        
    println!("Part One: {result_1}");

    let mut scores: Vec<_> = input
        .into_iter()
        .filter_map(autocomplete_missing_part)
        .map(|s| {
            s.into_iter()
                .rev()
                .fold(0, |acc, x| acc * 5 + char_to_points_part_2(x))
        })
        .collect();
    scores.sort_unstable();

    let result_2 = scores[scores.len() / 2];
    println!("Part Two: {result_2}");
}
