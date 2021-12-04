fn sum_command(input: &[(&str, i64)], command: &str) -> i64 {
    input
        .iter()
        .filter_map(|&(c, n)| if c == command { Some(n) } else { None })
        .sum()
}

fn main() {
    let input: Vec<_> = include_str!("../input")
        .lines()
        .map(|i| {
            let mut words = i.split_whitespace();
            let command = words.next().unwrap();
            let number: i64 = words.next().unwrap().parse().unwrap();
            (command, number)
        })
        .collect();

    let sum_forward = sum_command(&input, "forward");
    let sum_down = sum_command(&input, "down");
    let sum_up = sum_command(&input, "up");

    let h_position = sum_forward;
    let v_position = sum_down - sum_up;

    println!("Part one: {}", h_position * v_position);

    let (_aim, depth, h_position) =
        input
            .into_iter()
            .fold((0, 0, 0), |(acc_a, acc_d, acc_h), (c, n)| match c {
                "forward" => (acc_a, acc_d + acc_a * n, acc_h + n),
                "down" => (acc_a + n, acc_d, acc_h),
                "up" => (acc_a - n, acc_d, acc_h),
                _ => panic!("Unknown command"),
            });

    println!("Part two: {}", h_position * depth);
}
