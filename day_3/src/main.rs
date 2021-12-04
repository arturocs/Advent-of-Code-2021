fn add_bit_to_count(n_0: i32, n_1: i32, c: char) -> (i32, i32) {
    match c {
        '0' => (n_0 + 1, n_1),
        '1' => (n_0, n_1 + 1),
        _ => panic!("Unrecognised character: {}", c),
    }
}

fn reduction_function_1(acc: Vec<(i32, i32)>, x: &[char]) -> Vec<(i32, i32)> {
    x.iter()
        .zip(acc.into_iter())
        .map(|(&c, (_0, _1))| add_bit_to_count(_0, _1, c))
        .collect()
}

fn reduction_function_2(acc: Vec<Vec<char>>, i: usize, c1: char, c2: char) -> Vec<Vec<char>> {
    if acc.len() <= 1 {
        return acc;
    }
    let (a, b) = acc
        .iter()
        .fold((0, 0), |(n_0, n_1), x| add_bit_to_count(n_0, n_1, x[i]));
    let c = if a > b { c1 } else { c2 };
    acc.into_iter().filter(|l| l[i] == c).collect()
}

fn bin_string_to_u32(string: &str) -> u32 {
    u32::from_str_radix(string, 2).unwrap()
}

fn solve_part_1(input: &[Vec<char>], n_bits: usize, c1: char, c2: char) -> u32 {
    let bin_string: String = input
        .iter()
        .fold(vec![(0, 0); n_bits], |acc, x| reduction_function_1(acc, x))
        .into_iter()
        .map(|(a, b)| if a > b { c1 } else { c2 })
        .collect();
    bin_string_to_u32(&bin_string)
}

fn solve_part_2(input: Vec<Vec<char>>, n_bits: usize, c1: char, c2: char) -> u32 {
    let bin_string: String = (0..n_bits)
        .fold(input, |acc, i| reduction_function_2(acc, i, c1, c2))
        .first()
        .unwrap()
        .iter()
        .collect();
    bin_string_to_u32(&bin_string)
}

fn main() {
    let input: Vec<Vec<char>> = include_str!("../input")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let n_bits = input.first().unwrap().len();

    let gamma_rate = solve_part_1(&input, n_bits, '0', '1');
    let epsilon_rate = solve_part_1(&input, n_bits, '1', '0');

    println!("Part one: {}", gamma_rate * epsilon_rate);

    let o2_rating = solve_part_2(input.clone(), n_bits, '0', '1');
    let co2_rating = solve_part_2(input, n_bits, '1', '0');

    println!("Part two: {}", o2_rating * co2_rating);
}
