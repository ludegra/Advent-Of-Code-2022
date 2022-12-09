use std::time::Instant;

use itertools::Itertools;
const DAY_NUMBER: u32 = 6;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

fn solve(mut input: impl Iterator<Item = String>, start: Instant) {
    let input = input.next().unwrap();

    for (i, (a, b, c, d)) in input.chars().tuple_windows().enumerate() {
        if [a, b, c, d].iter().all_unique() {
            println!("{}", i + 4);
            println!("{}\u{00B5}s", start.elapsed().as_micros());
            break;
        }
    }

    let start = Instant::now();
    let chars = input.chars().collect::<Vec<_>>();

    for i in 0.. {
        if chars[i..(i + 14)].iter().all_unique() {
            println!("{}", i + 14);
            println!("{}\u{00B5}s", start.elapsed().as_micros());
            break;
        }
    }
}
