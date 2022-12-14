use std::{collections::HashSet, time::{Instant, Duration}, thread::sleep};

use itertools::Itertools;
const DAY_NUMBER: u32 = 9;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

fn solve(input: impl Iterator<Item = String>, start: Instant) {
    let input = input.collect::<Vec<String>>();
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(tail_position);

    for command in &input {
        let mut split = command.split_whitespace();
        let direction = match split.next().unwrap() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        };
        let distance = split.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..distance {
            head_position.0 += direction.0;
            head_position.1 += direction.1;

            while i32::abs(head_position.0 - tail_position.0) > 1
                || i32::abs(head_position.1 - tail_position.1) > 1
            {
                if i32::abs(head_position.0 - tail_position.0) > 1
                    && i32::abs(head_position.1 - tail_position.1) > 0
                {
                    tail_position.0 += (head_position.0 - tail_position.0)
                        / i32::abs(head_position.0 - tail_position.0);
                    tail_position.1 += (head_position.1 - tail_position.1)
                        / i32::abs(head_position.1 - tail_position.1);
                } else if i32::abs(head_position.1 - tail_position.1) > 1
                    && i32::abs(head_position.0 - tail_position.0) > 0
                {
                    tail_position.0 += (head_position.0 - tail_position.0)
                        / i32::abs(head_position.0 - tail_position.0);
                    tail_position.1 += (head_position.1 - tail_position.1)
                        / i32::abs(head_position.1 - tail_position.1);
                } else if i32::abs(head_position.0 - tail_position.0) > 1 {
                    tail_position.0 += direction.0;
                } else if i32::abs(head_position.1 - tail_position.1) > 1 {
                    tail_position.1 += direction.1;
                }

                visited.insert(tail_position);
            }
            // println!("{:?}, {:?}", head_position, tail_position);
        }
    }

    println!("{}", visited.len());

    let mut knots = [(0, 0); 10];
    let mut visited = HashSet::new();
    visited.insert(knots.last().unwrap().clone());

    for command in input {
        let mut split = command.split_whitespace();
        let direction = match split.next().unwrap() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        };
        let distance = split.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..distance {
            knots[0].0 += direction.0;
            knots[0].1 += direction.1;

            for i in 1..knots.len() {
                let prev = knots[i - 1].clone();
                let current = &mut knots[i];
                let mut dx = current.0 - prev.0;
                let mut dy = current.1 - prev.1;

                while i32::abs(dx) > 1 || i32::abs(dy) > 1 {
                    let direction = match (dx, dy) {
                        (dx, 0) if dx < 0 => (1, 0),
                        (dx, 0) if dx > 0 => (-1, 0),
                        (0, dy) if dy < 0 => (0, 1),
                        (0, dy) if dy > 0 => (0, -1),
                        (dx, dy) if dx < 0 && dy < 0 => (1, 1),
                        (dx, dy) if dx > 0 && dy < 0 => (-1, 1),
                        (dx, dy) if dx < 0 && dy > 0 => (1, -1),
                        (dx, dy) if dx > 0 && dy > 0 => (-1, -1),
                        _ => unreachable!(),
                    };
                    current.0 += direction.0;
                    current.1 += direction.1;
                    dx = current.0 - prev.0;
                    dy = current.1 - prev.1;
                }
                visited.insert(knots.last().unwrap().clone());
                for y in (-15..15).rev() {
                    for x in -15..15 {
                        if knots.contains(&(x, y)) {
                            print!("{}", knots.iter().position(|p| *p == (x, y)).unwrap())
                        } else {
                            match visited.contains(&(x, y)) {
                                true => print!("#"),
                                false => print!(".")
                            }
                        }
                    }
                    println!()
                }
                println!("{}, current knot: {}\n", command, i);
                sleep(Duration::from_millis(10));
            }
        }
    }

    for y in (-15..15).rev() {
        for x in -15..15 {
            if visited.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }

    // println!("{:?}", visited.iter().sorted().collect::<Vec<_>>());
    println!("{}", visited.len());
}
