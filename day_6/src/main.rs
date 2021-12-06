use std::collections::BTreeMap;

fn calculate_population(initial_population: &BTreeMap<i32, u128>, days: u32) -> u128 {
    (0..days)
        .fold(initial_population.clone(), |acc, _| {
            let mut new_generation: BTreeMap<_, _> = acc
                .into_iter()
                .filter(|&(a, _)| a >= 0)
                .map(|(a, b)| (a - 1, b))
                .collect();

            if new_generation.get(&-1).is_some() {
                *new_generation.entry(6).or_insert(0) += new_generation[&-1];
                *new_generation.entry(8).or_insert(0) += new_generation[&-1];
                new_generation.insert(-1, 0);
            }
            new_generation
        })
        .into_iter()
        .fold(0, |acc, (_, n)| acc + n)
}

fn main() {
    let input: BTreeMap<i32, u128> = include_str!("../input")
        .split(",")
        .map(|n| n.trim().parse().unwrap())
        .fold(BTreeMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

    let result_1 = calculate_population(&input, 80);
    let result_2 = calculate_population(&input, 256);

    println!("Part one: {}", result_1);
    println!("Part two: {}", result_2);
}
