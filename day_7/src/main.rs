fn main() {
    let input: Vec<i32> = include_str!("../input")
        .split(',')
        .map(|i| i.trim().parse().unwrap())
        .collect();

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let result_1: i32 = (min..max)
        .map(|i| input.iter().map(|&j| (i - j).abs()).sum())
        .min()
        .unwrap();

    let result_2: i32 = (min..max)
        .map(|i| input.iter().flat_map(|&j| (0..=(i - j).abs())).sum())
        .min()
        .unwrap();

    println!("Part One: {}", result_1);
    println!("Part Two: {}", result_2);
}
