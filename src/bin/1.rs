use std::time::Instant;
const DAY_NUMBER: u32 = 1;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

fn solve(input: impl Iterator<Item = String>, start: Instant) {
    let mut current = 0;
    let mut totals = Vec::new();
    
    for line in input {
        if !line.is_empty() {
            let l = line.parse::<u32>().unwrap();
            current += l;
        }
        else {
            totals.push(current);
            current = 0;
        }
    }

    totals.sort();

    println!("{}", totals.last().unwrap());
    println!("{}ns", start.elapsed().as_nanos());
    let part1_done = Instant::now();

    let last_three: u32 = totals.into_iter().rev().take(3).sum();
    println!("{}", last_three);
    println!("{}ns", (start.elapsed() - part1_done.elapsed()).as_nanos())
}