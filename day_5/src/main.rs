use nalgebra::{DMatrix, Vector2};
use std::cmp::{max, Ordering};

fn parse_vector2(l: &mut std::str::Split<&str>) -> Vector2<usize> {
    Vector2::from_iterator(l.next().unwrap().split(',').map(|i| i.parse().unwrap()))
}

fn range_from_coord(x1: usize, x2: usize) -> Box<dyn Iterator<Item = usize>> {
    match x1.cmp(&x2) {
        Ordering::Less => Box::new(x1..=x2),
        Ordering::Equal => Box::new(std::iter::repeat(x1)),
        Ordering::Greater => Box::new((x2..=x1).rev()),
    }
}

fn generate_intermediate_points(
    p1: Vector2<usize>,
    p2: Vector2<usize>,
) -> impl Iterator<Item = Vector2<usize>> {
    let range_1 = range_from_coord(p1.x, p2.x);
    let range_2 = range_from_coord(p1.y, p2.y);
    range_1.zip(range_2).map(|(x, y)| Vector2::new(x, y))
}

fn main() {
    let input: Vec<_> = include_str!("../input")
        .lines()
        .map(|l| l.split(" -> "))
        .map(|mut l| [parse_vector2(&mut l), parse_vector2(&mut l)])
        .collect();

    let size = 1 + input.iter().flatten().map(|i| max(i.x, i.y)).max().unwrap();

    let part1_matrix = input
        .iter()
        .filter(|&i| i[0].x == i[1].x || i[0].y == i[1].y)
        .flat_map(|&[p1, p2]| generate_intermediate_points(p1, p2))
        .fold(DMatrix::<u16>::zeros(size, size), |mut acc, i| {
            acc[(i.y, i.x)] += 1;
            acc
        });

    let part2_matrix = input
        .iter()
        .flat_map(|&[p1, p2]| generate_intermediate_points(p1, p2))
        .fold(DMatrix::<u16>::zeros(size, size), |mut acc, i| {
            acc[(i.y, i.x)] += 1;
            acc
        });

    let result_1 = part1_matrix.iter().filter(|&&i| i > 1).count();
    let result_2 = part2_matrix.iter().filter(|&&i| i > 1).count();

    println!("Part one: {}", result_1);
    println!("Part two: {}", result_2);
}
