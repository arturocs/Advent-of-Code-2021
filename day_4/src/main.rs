use nalgebra::Matrix5;

fn sum_unmarked_numbers(board: &Matrix5<(i32, bool)>) -> i32 {
    board
        .iter()
        .filter_map(|&(n, b)| if !b { Some(n) } else { None })
        .sum()
}

fn check_boards(boards: &[Matrix5<(i32, bool)>]) -> Option<(i32, usize)> {
    boards.iter().enumerate().find_map(|(n, board)| {
        if board.row_iter().any(|row| row.iter().all(|i| i.1))
            || board.column_iter().any(|col| col.iter().all(|i| i.1))
        {
            Some((sum_unmarked_numbers(board), n))
        } else {
            None
        }
    })
}

fn update_boards(boards: &mut [Matrix5<(i32, bool)>], n: i32) {
    boards.iter_mut().flatten().for_each(|(i, b)| {
        if *i == n {
            *b = true
        }
    })
}

fn main() {
    let mut input = include_str!("../input").lines();
    let draw_numbers = input.next().unwrap().split(',').map(|i| i.parse().unwrap());

    let mut boards: Vec<Matrix5<(i32, bool)>> = input
        .flat_map(|l| l.split_whitespace().map(|i| (i.parse().unwrap(), false)))
        .collect::<Vec<_>>()
        .chunks(25)
        .map(|i| Matrix5::from_column_slice(i))
        .collect();

    let results: Vec<_> = draw_numbers
        .filter_map(|n| {
            update_boards(&mut boards, n);
            check_boards(&boards).map(|(sum, i)| {
                boards[i] = Matrix5::repeat((-1, false));
                sum * n
            })
        })
        .filter(|&i| i != 0)
        .collect();

    dbg!(&results);
    dbg!(&results.len());
    println!("Part one: {}", results.first().unwrap());
    println!("Part two: {}", results.last().unwrap());
}
