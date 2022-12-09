use std::time::Instant;
const DAY_NUMBER: u32 = 3;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

fn solve(input: impl Iterator<Item = String>, start: Instant) {
    let input = input.collect::<Vec<_>>();
    let sum = input.iter().map(|l| {
        let first = l[..(l.len() / 2)].to_string();
        let second = l[(l.len() / 2)..].to_string();

        (first, second)
    }).fold(0, |acc, (first, second)| {
        for c in first.chars() {
            if second.contains(c) {
                return acc + get_priority(c)
            }
        }
        acc
    });

    println!("{}", sum);

    let mut sum = 0;

    let mut input = input.into_iter();
    for _ in (0..input.len()).step_by(3) {
        let first = input.next().unwrap();
        let second = input.next().unwrap();
        let third = input.next().unwrap();

        for c in first.chars() {
            if second.contains(c) && third.contains(c) {
                sum += get_priority(c);
                break;
            }
        }
    }

    println!("{}", sum);
}

fn get_priority(c: char) -> u32 {
    let out = match c {
        'a'..='z' => c as u32 - ('a' as u32 - 1),
        'A'..='Z' => c as u32 - ('A' as u32) + 27,
        _ => panic!("Invalid")
    };

    out
}
