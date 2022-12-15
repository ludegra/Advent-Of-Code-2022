use std::{collections::HashSet, time::{Instant, Duration}, thread};
const DAY_NUMBER: u32 = 14;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

fn solve(input: impl Iterator<Item = String>, start: Instant) {
    let mut grid: HashSet<(usize, usize)> = HashSet::new();

    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;

    for path in input {
        let mut vertices = path.split(" -> ").map(|s| {
            let mut coords = s.split(',');
            (
                coords.next().unwrap().parse::<usize>().unwrap(),
                coords.next().unwrap().parse::<usize>().unwrap(),
            )
        });

        let mut prev = vertices.next().unwrap();

        min_x = min_x.min(prev.0);
        min_y = min_y.min(prev.1);
        max_x = max_x.max(prev.0);
        max_y = max_y.max(prev.1);

        for vertex in vertices {
            min_x = min_x.min(vertex.0);
            min_y = min_y.min(vertex.1);
            max_x = max_x.max(vertex.0);
            max_y = max_y.max(vertex.1);

            match (vertex.0.abs_diff(prev.0), vertex.1.abs_diff(prev.1)) {
                (_, 0) => {
                    for x in prev.0.min(vertex.0)..=prev.0.max(vertex.0) {
                        grid.insert((x, vertex.1));
                    }
                }
                (0, _) => {
                    for y in prev.1.min(vertex.1)..=prev.1.max(vertex.1) {
                        grid.insert((vertex.0, y));
                    }
                }
                _ => unreachable!()
            };

            prev = vertex;
        }
    }

    let mut sand_locations = HashSet::new();

    'outer: loop {
        let mut sand_pos = (500, 0);

        while let Some(new_position) = new_position(sand_pos, &grid) {
            if new_position.1 > max_y {
                break 'outer
            }
            sand_pos = new_position;
        }
        
        sand_locations.insert(sand_pos);
        grid.insert(sand_pos);
    }
    println!("{}", sand_locations.len());

    for x in 0..1000 {
        grid.insert((x, max_y + 2));
    }

    max_y += 2;

    while !sand_locations.contains(&(500, 0)) {
        let mut sand_pos = (500, 0);

        while let Some(new_position) = new_position(sand_pos, &grid) {
            sand_pos = new_position;

            min_x = min_x.min(new_position.0);
            max_x = max_x.max(new_position.0);
        }
        
        sand_locations.insert(sand_pos);
        grid.insert(sand_pos);
    }

    let mut string = String::from("\x1B[2J\x1B[1;1H");
    for y in 0..=max_y {
    string.push_str(&format!("{:03} ", y));
            for x in (min_x - 1)..=(max_x + 1) {
            string.push_str(&format!("{}", 
            if sand_locations.contains(&(x, y)) {
                'o'
            }
            else if grid.contains(&(x, y)) {
                '#'
            } else {
                '.'
            }))
        }
        string.push('\n')
    }
    println!("{}", string);
    println!("{}", sand_locations.len())
}

fn new_position(sand_pos: (usize, usize), grid: &HashSet<(usize, usize)>) -> Option<(usize, usize)> {
    if !grid.contains(&(sand_pos.0, sand_pos.1 + 1)) {
        Some((sand_pos.0, sand_pos.1 + 1))
    }
    else if !grid.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
        Some((sand_pos.0 - 1, sand_pos.1 + 1))
    }
    else if !grid.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
        Some((sand_pos.0 + 1, sand_pos.1 + 1))
    }
    else {
        None
    }
}
