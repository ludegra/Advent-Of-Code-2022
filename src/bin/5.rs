use std::{time::Instant, collections::HashMap};

use itertools::Itertools;
const DAY_NUMBER: u32 = 5;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

fn solve(mut input: impl Iterator<Item = String>, start: Instant) {
    let mut stacks = HashMap::new();

    while let Some(c) = input.next() {
        if !c.contains('[') {
            break;
        }

        let chunks = c.chars().chunks(4);
        let c = chunks.into_iter().map(|c| c.collect::<String>().trim().to_owned());
        for (i, chunk) in c.enumerate() {
            if !chunk.is_empty() {
                let stack = stacks.entry(i + 1).or_insert(Vec::new());
                stack.push(chunk.chars().nth(1).unwrap())
            }
        }
    }

    for (_, v) in stacks.iter_mut() {
        *v = v.into_iter().rev().map(|c| c.to_owned()).collect();
    }

    input.next();

    let mut stacks2 = stacks.clone();

    for instruction in input {
        // for (k, v) in stacks2.iter().sorted() {
        //     println!("{}: {:?}", k, v);
        // }
        // println!("");
        let instruction = instruction.split_whitespace().chunks(2);
        let mut instruction = instruction.into_iter();
        let n = instruction.next().unwrap().nth(1).unwrap().parse::<usize>().unwrap();
        let start = instruction.next().unwrap().nth(1).unwrap().parse::<usize>().unwrap();
        let goal = instruction.next().unwrap().nth(1).unwrap().parse::<usize>().unwrap();

        let mut temp_storage = Vec::new();
        let mut temp_storage2 = Vec::new();

        for _ in 0..n {
            temp_storage.push(stacks.get_mut(&start).unwrap().pop().unwrap());
            temp_storage2.push(stacks2.get_mut(&start).unwrap().pop().unwrap());
        }

        stacks.get_mut(&goal).unwrap().extend(temp_storage.into_iter());
        stacks2.get_mut(&goal).unwrap().extend(temp_storage2.into_iter().rev());
    }
    
    // for (k, v) in stacks2.iter().sorted() {
    //     println!("{}: {:?}", k, v);
    // }
    // println!("");

    let out = stacks.iter().sorted().fold(String::new(), |acc, (_k, v)| acc + &v.last().unwrap().to_string());
    println!("{}", out);

    let out = stacks2.iter().sorted().fold(String::new(), |acc, (_k, v)| acc + &v.last().unwrap().to_string());
    println!("{}", out);
    println!("{}\u{00B5}s", start.elapsed().as_micros())
}
