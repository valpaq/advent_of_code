#![feature(array_windows)]

fn first_task() -> usize {
    include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i64>>()
        .array_windows()
        .filter(|[a, b]| a < b)
        .count()
}

fn second_task() -> usize {
    include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i64>>()
        .array_windows()
        .filter(|[a, _b, _c, d]| a < d)
        .count()
}

fn main() {
    println!("first res {}", first_task());
    println!("second res {}", second_task());
}