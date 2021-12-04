fn sum_unmarked_numbers(board: &[Vec<(i32, bool)>]) -> i32 {
    board
        .iter()
        .flatten()
        .filter_map(|&(n, b)| if !b { Some(n) } else { None }).inspect(|i|println!("{}",i))
        .sum()
}

fn check_boards(boards: &[Vec<Vec<(i32, bool)>>]) -> Option<(i32, usize)> {
    for (n, board) in boards.iter().enumerate() {
        if board.iter().find(|row| row.iter().all(|i| i.1)).is_some() {
            return Some((sum_unmarked_numbers(board), n));
        }
        if (0..board.len())
            .find(|&i| (0..board.len()).all(|j| board[j][i].1))
            .is_some()
        {
            return Some((sum_unmarked_numbers(board), n));
        }
    }
    None
}

fn update_boards(boards: &mut [Vec<Vec<(i32, bool)>>], n: i32) {
    boards.iter_mut().flatten().flatten().for_each(|(i, b)| {
        if *i == n {
            *b = true
        }
    })
}

fn main() {
    let mut input = include_str!("../input").lines();

    let draw_numbers: Vec<i32> = input
        .next()
        .unwrap()
        .split(',')
        .map(|i| i.parse().unwrap())
        .collect();

    let mut boards: Vec<Vec<Vec<(i32, bool)>>> = input
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split_whitespace()
                .map(|i| (i.parse().unwrap(), false))
                .collect()
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|i| i.to_vec())
        .collect();


    dbg!(draw_numbers.len());
    let results: Vec<_> = draw_numbers
        .into_iter()
        .filter_map(|n| {
            update_boards(&mut boards, n);
            check_boards(&boards).map(|(sum, i)| {
                boards[i] = vec![vec![(-1,false);5];5];
                //dbg!(i,n);
                sum * n
            })
        })
        //.filter(|&i|i!=0)
        .collect();

    dbg!(&results);
    dbg!(&results.len());
    //println!("Part one: {}", results.first().unwrap());
    println!("Part two: {}", results.last().unwrap());
}
