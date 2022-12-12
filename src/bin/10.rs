use std::time::Instant;

use colored::Colorize;
const DAY_NUMBER: u32 = 10;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

fn solve(input: impl Iterator<Item = String>, start: Instant) {
    let mut out = 0;
    let mut cycle = 0;
    let mut x = 1;
    let mut next_data_point = 20;

    let mut display = [[false; 40]; 6];

    for command in input {
        match command {
            v if &v[0..4] == "addx" => {
                if cycle + 2 >= next_data_point {
                    out += next_data_point * x;
                    next_data_point += 40;
                }

                draw(&mut display, x, cycle);
                draw(&mut display, x, cycle + 1);

                cycle += 2;
                x += v[5..].parse::<i32>().unwrap();
            }
            v if &v[0..4] == "noop" => {
                draw(&mut display, x, cycle);
                cycle += 1;
            }
            _ => unreachable!(),
        }
    }

    println!("{}", out);

    for row in display {
        for pixel in row {
            print!("{}", if pixel { '#' } else { ' ' })
        }
        println!()
    }
}

fn draw(display: &mut [[bool; 40]; 6], x: i32, cycle: i32) {

    let row = cycle / 40;
    let collumn = cycle % 40;

    if ((x - 1)..=(x + 1)).contains(&collumn) {
        display[row as usize][collumn as usize] = true;
    }
}
