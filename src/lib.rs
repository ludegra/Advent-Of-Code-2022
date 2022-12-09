use std::{io::{BufRead, BufReader}, fs::File};

pub fn read_lines(day_number: u32) -> impl Iterator<Item = String> {
    BufReader::new(File::open(format!("./inputs/day{}.in", day_number)).unwrap())
        .lines()
        .map(|line| line.unwrap())
}