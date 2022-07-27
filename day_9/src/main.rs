use std::collections::BTreeSet;
fn check_if_its_low_point(v: u32, values: &[Vec<u32>], i: usize, j: usize) -> bool {
    let rows = values.len();
    let columns = values[0].len();
    let a = i + 2 > rows || v < values[i + 1][j];
    let b = i as i64 - 1 < 0 || v < values[i - 1][j];
    let c = j + 2 > columns || v < values[i][j + 1];
    let d = j as i64 - 1 < 0 || v < values[i][j - 1];
    a && b && c && d
}
fn get_low_points(values: &[Vec<u32>]) -> Vec<(usize, usize)> {
    values
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(move |&(j, v)| check_if_its_low_point(*v, values, i, j))
                .map(move |(j, _)| (i, j))
        })
        .collect()
}
fn sum_of_risk_points(values: &[Vec<u32>]) -> u32 {
    get_low_points(values)
        .into_iter()
        .map(|(i, j)| values[i][j] + 1)
        .sum()
}

fn find_basin_recursive(
    values: &[Vec<u32>],
    basin: &mut BTreeSet<(usize, usize)>,
    i: usize,
    j: usize,
) {
    let rows = values.len();
    let columns = values[0].len();
    let add_i = |i, b: i64| (i as i64 + b).clamp(0, rows as i64 - 1) as usize;
    let add_j = |j, b: i64| (j as i64 + b).clamp(0, columns as i64 - 1) as usize;
    let v = values[i][j];
    let up = values[add_i(i, 1)][j];
    let down = values[add_i(i, -1)][j];
    let left = values[i][add_j(j, -1)];
    let right = values[i][add_j(j, 1)];
    basin.insert((i, j));
    if v < up && up < 9 {
        let current_i = add_i(i, 1);
        basin.insert((current_i, j));
        find_basin_recursive(values, basin, current_i, j);
    }
    if v < down && down < 9 {
        let current_i = add_i(i, -1);
        basin.insert((current_i, j));
        find_basin_recursive(values, basin, current_i, j);
    }
    if v < left && left < 9 {
        let current_j = add_j(j, -1);
        basin.insert((i, current_j));
        find_basin_recursive(values, basin, i, current_j);
    }
    if v < right && right < 9 {
        let current_j = add_j(j, 1);
        basin.insert((i, current_j));
        find_basin_recursive(values, basin, i, current_j);
    }
}

fn find_basin(values: &[Vec<u32>], i: usize, j: usize) -> BTreeSet<(usize, usize)> {
    let mut basin = BTreeSet::new();
    find_basin_recursive(values, &mut basin, i, j);
    basin
}

fn sum_of_largest_basins(values: &[Vec<u32>]) -> usize {
    let low_points = get_low_points(&values);

    let mut used_poisitions = BTreeSet::new();
    let mut basins = vec![];
    for (i, j) in low_points {
        if !used_poisitions.contains(&(i, j)) {
            let mut basin = find_basin(&values, i, j);
            basins.push(basin.len());
            used_poisitions.append(&mut basin)
        }
    }
    basins.sort_unstable_by(|a, b| b.cmp(a));
    basins[..3].iter().product()
}

fn main() {
    let values: Vec<_> = include_str!("../input")
        .lines()
        .map(|l| l.trim().chars().map(|i| i.to_digit(10).unwrap()).collect())
        .filter(|i: &Vec<_>| i.len() > 0)
        .collect();

    let result_1 = sum_of_risk_points(&values);

    println!("Part One: {}", result_1);

    let result_2 = sum_of_largest_basins(&values);
    println!("Part Two: {}", result_2);
}
