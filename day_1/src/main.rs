fn count_increases(input: &[u64]) -> u64 {
    input.windows(2).filter(|i| i[0] < i[1]).count() as u64
}

fn main() {
    let input: Vec<u64> = include_str!("../input")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();

    let number_of_increases_1 = count_increases(&input);
    println!("Part one: {}", number_of_increases_1);
    
    let sum_of_windows: Vec<u64> = input.windows(3).map(|i| i.iter().sum()).collect();
    let number_of_increases_2 = count_increases(&sum_of_windows);
    println!("Part two: {}", number_of_increases_2);
}
