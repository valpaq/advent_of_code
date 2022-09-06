#![feature(drain_filter)]
const WIDTH: usize = 12;
const COUNT: usize = 1000;

pub fn first_task() -> usize {
    let gamma = include_str!("../input.txt")
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .fold(vec![0; WIDTH], |count, bits| {
            count
                .into_iter()
                .enumerate()
                .map(|(i, n)| n + ((bits & 1 << i) >> i))
                .collect()
        })
        .into_iter()
        .enumerate()
        .map(|(i, b)| ((b >= COUNT / 2) as usize) << i)
        .sum::<usize>();

    gamma * (!gamma & ((1 << WIDTH) - 1))
}

pub fn second_task() -> usize {
    let nums: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect::<Vec<_>>();

    let oxy: usize = (0..WIDTH)
        .rev()
        .scan(nums.clone(), |oxy, i| {
            let one = oxy.iter().filter(|n| *n & 1 << i > 0).count() >= (oxy.len() + 1) / 2;
            oxy.drain_filter(|n| (*n & 1 << i > 0) != one);
            oxy.first().copied()
        })
        .last()
        .unwrap();

    let co2: usize = (0..WIDTH)
        .rev()
        .scan(nums, |co2, i| {
            let one = co2.iter().filter(|n| *n & 1 << i > 0).count() >= (co2.len() + 1) / 2;
            co2.drain_filter(|n| (*n & 1 << i > 0) == one);
            co2.first().copied()
        })
        .last()
        .unwrap();

    oxy * co2
}

fn main() {
    println!("first res {}", first_task());
    println!("second res {}", second_task());
}
