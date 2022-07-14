#![feature(array_windows)]

fn first_task() -> u64 {
    let (f, d) = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0), |(f, d), (k, v)| {
            match (k, v.parse::<u64>().unwrap()) {
                ("forward", v) => (f + v, d),
                ("down", v) => (f, d + v),
                ("up", v) => (f, d - v),
                _ => unreachable!(),
            }
        });
    f*d
}

fn second_task() -> u64 {
    let (h, d, _) = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0, 0), |(h, d, a), (k, v)| {
            match (k, v.parse::<u64>().unwrap()) {
                ("forward", v) => (h+v, a*v + d, a),
                ("down", v) => (h, d, a+v),
                ("up", v) => (h, d, a-v),
                _ => unreachable!(),
            }
        });
    h*d
}

fn main() {
    println!("first res {}", first_task());
    println!("second res {}", second_task());
}