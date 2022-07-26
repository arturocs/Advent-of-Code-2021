use std::collections::{BTreeMap, BTreeSet};
use itertools::Itertools;

fn count_digits(input: &[(Vec<&str>, Vec<&str>)], lenght: usize) -> usize {
    input
        .iter()
        .flat_map(|s| &s.1)
        .filter(|s| s.len() == lenght)
        .count()
}

fn solve_easy_pattern(pattern: &str, number: i32, dictionary: &mut BTreeMap<i32, BTreeSet<char>>) {
    pattern.chars().for_each(|c| {
        dictionary
            .entry(number)
            .or_insert(BTreeSet::from([c]))
            .insert(c);
    })
}

fn generate_segment_dictionary(patterns: &[&str]) -> BTreeMap<i32, BTreeSet<char>> {
    let mut dictionary = BTreeMap::new();
    for pattern in patterns.iter() {
        match pattern.len() {
            2 => solve_easy_pattern(pattern, 1, &mut dictionary),
            4 => solve_easy_pattern(pattern, 4, &mut dictionary),
            3 => solve_easy_pattern(pattern, 7, &mut dictionary),
            7 => solve_easy_pattern(pattern, 8, &mut dictionary),
            _ => {}
        }
    }

    // 3 is the only number with five segments that shares all the segments that 7 has
    let three = patterns
        .iter()
        .find(|s| s.len() == 5 && dictionary[&7].iter().all(|&i| s.contains(i)))
        .unwrap()
        .chars()
        .collect();
    dictionary.insert(3, three);

    let nine: BTreeSet<_> = dictionary[&4].union(&dictionary[&3]).cloned().collect();
    dictionary.insert(9, nine);

    // The only segment that differences 8 and 9 is the bottom left one
    let bottom_left_segment = dictionary[&8]
        .difference(&dictionary[&9])
        .cloned()
        .next()
        .unwrap();

    // 2 is the only five segment number that uses the bottom left segment
    let two = patterns
        .iter()
        .find(|s| s.len() == 5 && s.contains(bottom_left_segment))
        .unwrap()
        .chars()
        .collect();
    dictionary.insert(2, two);

    // 5 is the last five segment number we haven't find out yet
    let five = patterns
        .iter()
        .find(|s| s.len() == 5 && dictionary.values().all(|i| i.ne(&s.chars().collect())))
        .unwrap()
        .chars()
        .collect();
    dictionary.insert(5, five);

    let middle_segment = *dictionary[&4]
        .difference(&dictionary[&7])
        .cloned()
        .collect::<BTreeSet<_>>()
        .intersection(&dictionary[&2])
        .next()
        .unwrap();

    // 0 is like 8 but with the middle segment removed
    let zero = dictionary[&8]
        .iter()
        .filter(|&&i| i != middle_segment)
        .cloned()
        .collect();
    dictionary.insert(0, zero);

    // 6 is the last six segment number we haven't find out yet
    let six = patterns
        .iter()
        .find(|s| s.len() == 6 && dictionary.values().all(|i| i.ne(&s.chars().collect())))
        .unwrap()
        .chars()
        .collect();
    dictionary.insert(6, six);
    dictionary
}

fn solve_line((patterns, value): (Vec<&str>, Vec<&str>)) -> i32 {
    let dictionary = generate_segment_dictionary(&patterns);
    let inverted_dictionary: BTreeMap<_, _> = dictionary.into_iter().map(|(k, v)| (v, k)).collect();
    (0..value.len())
        .rev()
        .zip(value)
        .map(|(i, digit)| inverted_dictionary[&digit.chars().collect()] * 10i32.pow(i as u32))
        .sum()
}

fn parse_line(line: &str) -> (Vec<&str>, Vec<&str>) {
    line.split('|')
        .map(|i| i.split_whitespace().collect())
        .collect_tuple()
        .unwrap()
}

fn main() {
    let input: Vec<_> = include_str!("../input").lines().map(parse_line).collect();

    let result_1: usize = [2, 4, 3, 7]
        .into_iter()
        .map(|i| count_digits(&input, i))
        .sum();

    println!("Part One: {}", result_1);

    let result_2: i32 = input.into_iter().map(solve_line).sum();
    println!("Part Two: {}", result_2);
}
