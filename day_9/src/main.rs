fn check_if_is_low_point(v: u32, values: &[Vec<u32>], i: usize, j: usize) -> u32 {
    let rows = values.len();
    let columns = values[0].len();
    let a = i + 2 > rows || v < values[i + 1][j];
    let b = i as i64 - 1 < 0 || v < values[i - 1][j];
    let c = j + 2 > columns || v < values[i][j + 1];
    let d = j as i64 - 1 < 0 || v < values[i][j - 1];
    if a && b && c && d {
        v + 1
    } else {
        0
    }
}
fn main() {
    let values: Vec<_> = include_str!("../input")
        .lines()
        .map(|l| l.trim().chars().map(|i| i.to_digit(10).unwrap()).collect())
        .filter(|i: &Vec<u32>| i.len() > 0)
        .collect();
        
    let result_1: u32 = values
        .iter()
        .enumerate()
        .map(|(i, row)| -> u32 {
            row.iter()
                .enumerate()
                .map(|(j, &v)| check_if_is_low_point(v, &values, i, j))
                .sum()
        })
        .sum();

    println!("Part One: {}", result_1);
}
