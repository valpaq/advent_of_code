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

// fn second_task() -> u64 {
//     let (h, d, _) = include_str!("../input.txt")
//         .lines()
//         .map(|l| l.split_once(" ").unwrap())
//         .fold((0, 0, 0), |(h, d, a), (k, v)| {
//             match (k, v.parse::<u64>().unwrap()) {
//                 ("forward", v) => (h+v, a*v + d, a),
//                 ("down", v) => (h, d, a+v),
//                 ("up", v) => (h, d, a-v),
//                 _ => unreachable!(),
//             }
//         });
//     h*d
// }

fn main() {
    println!("first res {}", first_task());
    // println!("second res {}", second_task());
}