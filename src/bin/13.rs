use std::time::Instant;

use itertools::Itertools;
const DAY_NUMBER: u32 = 13;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

#[derive(Debug Serialize)]
enum Packet {
    Array(Box<Vec<Self>>),
    Value(usize)
}

fn solve(mut input: impl Iterator<Item = String>, start: Instant) {
    let input = input.chunks(3).into_iter().map(|mut chunk| (chunk.next().unwrap(), chunk.next().unwrap()));


}
