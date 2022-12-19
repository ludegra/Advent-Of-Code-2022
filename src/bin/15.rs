use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};
const DAY_NUMBER: u32 = 15;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

fn solve(input: impl Iterator<Item = String>, start: Instant) {
    
    let mut sensors = HashMap::new();
    let mut beacons = HashSet::new();
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for line in input {
        let mut split = line.split(": ");
        let mut sensor_coords = split.next().unwrap()[10..]
            .split(", ")
            .map(|s| s[2..].parse::<i32>().unwrap());
        let sensor_x = sensor_coords.next().unwrap();
        let sensor_y = sensor_coords.next().unwrap();

        let mut beacon_coords = split.next().unwrap()[21..]
            .split(", ")
            .map(|s| s[2..].parse::<i32>().unwrap());
        let beacon_x = beacon_coords.next().unwrap();
        let beacon_y = beacon_coords.next().unwrap();

        beacons.insert((beacon_x, beacon_y));

        let radius = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);

        min_x = min_x.min(sensor_x - radius as i32);
        max_x = max_x.max(sensor_x + radius as i32);
        min_y = min_y.min(sensor_y - radius as i32);
        max_y = max_y.max(sensor_y + radius as i32);

        sensors.insert((sensor_x, sensor_y), radius);
    }

    let mut blocked_locations = 0;
    println!("{} -> {}", min_x, max_x);

    // Funkar inte längre
    let y = 2000000;
    let mut x = min_x - 1;
    'outer: while x <= max_x {
        x += 1;
        for ((sensor_x, sensor_y), radius) in sensors.iter() {
            if sensor_x.abs_diff(x) + sensor_y.abs_diff(y) <= *radius && !beacons.contains(&(x, y)) {
                let y_diff = sensor_y.abs_diff(y);
                let x_diff = x - sensor_x;
                x += *radius as i32 - (y_diff as i32 + x_diff);
                blocked_locations += *radius as i32 - (y_diff as i32 + x_diff);
                continue 'outer;
            }
        }
    }

    println!("Part 1: {} ({}ms)", blocked_locations, start.elapsed().as_millis());
    println!("(Part 1 är trasig)");
    let start = Instant::now();

    let search_area = 4000000;
    let mut out = (0, 0);
    'y_loop: for y in 0..=search_area {
        let mut x = -1;

        'x_loop: while x <= search_area {
            x += 1;

            for ((sensor_x, sensor_y), radius) in sensors.iter() {
                if sensor_x.abs_diff(x) + sensor_y.abs_diff(y) <= *radius {
                    let y_diff = sensor_y.abs_diff(y);
                    let x_diff = x - sensor_x;

                    x += *radius as i32 - (y_diff as i32 + x_diff);
                    continue 'x_loop;
                }
            }
            println!("out: ({}, {})", x, y);
            out = (x, y);
            break 'y_loop;
        }
    }

    let out = out.0 as u64 * 4000000 + out.1 as u64;
    println!("Part 2: {} ({}s)", out, start.elapsed().as_secs());
}
