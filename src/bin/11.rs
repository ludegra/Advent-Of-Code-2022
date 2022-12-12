use std::{collections::HashMap, time::Instant};

use itertools::Itertools;
const DAY_NUMBER: u32 = 11;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

enum Operation {
    Multiplied(u64),
    Added(u64),
    Squared
}

struct Monkey {
    pub items: Vec<u64>,
    pub operation: Operation,
    pub test: Box<dyn Fn(bool) -> u64>,
    pub test_value: u64,
}

fn solve(_input: impl Iterator<Item = String>, start: Instant) {
    let mut monkies = vec![
        Monkey {
            items: vec![79, 98],
            operation: Operation::Multiplied(19),
            test: Box::new(|test_result| if test_result { 2 } else { 3 }),
            test_value: 23,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            operation: Operation::Added(6),
            test: Box::new(|test_result| if test_result { 2 } else { 0 }),
            test_value: 19,
        },
        Monkey {
            items: vec![79, 60, 97],
            operation: Operation::Squared,
            test: Box::new(|test_result| if test_result { 1 } else { 3 }),
            test_value: 13,
        },
        Monkey {
            items: vec![74],
            operation: Operation::Added(3),
            test: Box::new(|test_result| if test_result { 0 } else { 1 }),
            test_value: 17,
        }, // Monkey {
           //     items: vec![74, 64, 74, 63, 53],
           //     operation: Operation::Multiplied(7),
           //     test: Box::new(|test_result| if test_result { 1 } else { 6 }),
           //     test_value : 5,
           // },
           // Monkey {
           //     items: vec![69, 99, 95, 62],
           //     operation: Operation::Squared,
           //     test: Box::new(|test_result| if test_result { 2 } else { 5 }),
           //     test_value: 17
           // },
           // Monkey {
           //     items: vec![59, 81],
           //     operation: Operation::Added(8),
           //     test: Box::new(|test_result| if test_result { 4 } else { 3 }),
           //     test_value: 7
           // },
           // Monkey {
           //     items: vec![50, 67, 63, 57, 63, 83, 97],
           //     operation: Operation::Added(4),
           //     test: Box::new(|test_result| if test_result { 0 } else { 7 }),
           //     test_value: 13
           // },
           // Monkey {
           //     items: vec![61, 94, 85, 52, 81, 90, 94, 70],
           //     operation: Operation::Added(3),
           //     test: Box::new(|test_result| if test_result { 7 } else { 3 }),
           //     test_value: 19
           // },
           // Monkey {
           //     items: vec![69],
           //     operation: Operation::Added(5),
           //     test: Box::new(|test_result| if test_result { 4 } else { 2 }),
           //     test_value: 3
           // },
           // Monkey {
           //     items: vec![54, 55, 58],
           //     operation: Operation::Added(7),
           //     test: Box::new(|test_result| if test_result { 1 } else { 5 }),
           //     test_value: 11
           // },
           // Monkey {
           //     items: vec![79, 51, 83, 88, 93, 76],
           //     operation: Operation::Multiplied(3),
           //     test: Box::new(|test_result| if test_result { 0 } else { 6 }),
           //     test_value: 2
           // },
    ]
    .into_iter()
    .enumerate()
    .collect::<HashMap<usize, Monkey>>();

    part1();

    let mut active_scores: HashMap<usize, u32> = HashMap::new();

    // TODO: logic

    println!("{:?}", active_scores);
    let mut sorted = active_scores.values().map(|v| v.to_owned()).sorted().rev();
    let product = sorted.next().unwrap() * sorted.next().unwrap();

    println!("{}", product);
}

fn part1() {
    let mut monkies = vec![
        Monkey {
            items: vec![79, 98],
            operation: Operation::Multiplied(19),
            test: Box::new(|test_result| if test_result { 2 } else { 3 }),
            test_value: 23,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            operation: Operation::Added(6),
            test: Box::new(|test_result| if test_result { 2 } else { 0 }),
            test_value: 19,
        },
        Monkey {
            items: vec![79, 60, 97],
            operation: Operation::Squared,
            test: Box::new(|test_result| if test_result { 1 } else { 3 }),
            test_value: 13,
        },
        Monkey {
            items: vec![74],
            operation: Operation::Added(3),
            test: Box::new(|test_result| if test_result { 0 } else { 1 }),
            test_value: 17,
        }, // Monkey {
           //     items: vec![74, 64, 74, 63, 53],
           //     operation: Operation::Multiplied(7),
           //     test: Box::new(|test_result| if test_result { 1 } else { 6 }),
           //     test_value : 5,
           // },
           // Monkey {
           //     items: vec![69, 99, 95, 62],
           //     operation: Operation::Squared,
           //     test: Box::new(|test_result| if test_result { 2 } else { 5 }),
           //     test_value: 17
           // },
           // Monkey {
           //     items: vec![59, 81],
           //     operation: Operation::Added(8),
           //     test: Box::new(|test_result| if test_result { 4 } else { 3 }),
           //     test_value: 7
           // },
           // Monkey {
           //     items: vec![50, 67, 63, 57, 63, 83, 97],
           //     operation: Operation::Added(4),
           //     test: Box::new(|test_result| if test_result { 0 } else { 7 }),
           //     test_value: 13
           // },
           // Monkey {
           //     items: vec![61, 94, 85, 52, 81, 90, 94, 70],
           //     operation: Operation::Added(3),
           //     test: Box::new(|test_result| if test_result { 7 } else { 3 }),
           //     test_value: 19
           // },
           // Monkey {
           //     items: vec![69],
           //     operation: Operation::Added(5),
           //     test: Box::new(|test_result| if test_result { 4 } else { 2 }),
           //     test_value: 3
           // },
           // Monkey {
           //     items: vec![54, 55, 58],
           //     operation: Operation::Added(7),
           //     test: Box::new(|test_result| if test_result { 1 } else { 5 }),
           //     test_value: 11
           // },
           // Monkey {
           //     items: vec![79, 51, 83, 88, 93, 76],
           //     operation: Operation::Multiplied(3),
           //     test: Box::new(|test_result| if test_result { 0 } else { 6 }),
           //     test_value: 2
           // },
    ]
    .into_iter()
    .enumerate()
    .collect::<HashMap<usize, Monkey>>();

    let mut active_scores: HashMap<usize, u32> = HashMap::new();

    for _ in 0..20 {
        for key in 0..monkies.keys().len() {
            println!("Monkey {}:", key);
            let current_monkey = monkies.get_mut(&key).unwrap();
            let mut next_monkies: HashMap<u64, Vec<u64>> = HashMap::new();
            while !current_monkey.items.is_empty() {
                let item = current_monkey.items.remove(0);
                let new_fear_level = match current_monkey.operation {
                    Operation::Added(value) => item + value,
                    Operation::Multiplied(value) => item * value,
                    Operation::Squared => item * item
                };
                println!("\t{} -> {}", item, new_fear_level);
                let next_monkey =
                    (current_monkey.test)(new_fear_level % current_monkey.test_value == 0);
                println!("\tNew monkey: {}", next_monkey);
                next_monkies
                    .entry(next_monkey)
                    .or_default()
                    .push(new_fear_level);
                let active_score = active_scores.entry(key).or_default();
                *active_score += 1;
                println!();
            }
            for (k, v) in next_monkies {
                let next_monkey = monkies.get_mut(&(k as usize)).unwrap();
                next_monkey.items.extend(&v);
            }
        }
    }

    println!("{:?}", active_scores);
    let mut sorted = active_scores.values().map(|v| v.to_owned()).sorted().rev();
    let product = sorted.next().unwrap() * sorted.next().unwrap();

    println!("{}", product);
}
