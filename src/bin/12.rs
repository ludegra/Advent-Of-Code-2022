use std::{
    collections::{HashMap, HashSet},
    time::{Instant},
};

use colored::Colorize;
use itertools::Itertools;
const DAY_NUMBER: u32 = 12;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

#[derive(Debug, Clone)]
struct Node {
    pub _id: usize,
    pub neighbors: Vec<usize>,
}

fn solve(input: impl Iterator<Item = String>, _start: Instant) {
    let grid = input
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = 0;
    let mut end = 0;

    let mut graph = vec![Vec::new(); grid.len()];

    let len_y = grid.len();
    let len_x = grid[0].len();

    for y in 0..len_y {
        for x in 0..len_x {
            let mut neighbors = find_neighbors(&grid, x, y);
            let mut height = grid[y][x] as isize;
            if grid[y][x] == 'S' {
                end = y * len_x + x;
                height = 'a' as isize
            } else if grid[y][x] == 'E' {
                start = y * len_x + x;
                height = 'z' as isize
            }

            // println!("Node: {}, height: {}", y * len_x + x, height,);
            // for neighbor in &neighbors {
            //     println!(
            //         "  Neighbor: {}, height: {}, diff: {}",
            //         neighbor,
            //         grid[neighbor / len_x][neighbor % len_x] as usize,
            //         (grid[neighbor / len_x][neighbor % len_x] as usize).abs_diff(height)
            //     )
            // }
            // println!();

            neighbors.retain(|neighbor| {
                if grid[neighbor / len_x][neighbor % len_x] != 'S' {
                    height - grid[neighbor / len_x][neighbor % len_x] as isize <= 1
                } else {
                    ('a' as isize) - height as isize <= 1
                }
            });

            graph[y].push(Node {
                _id: y * len_x + x,
                neighbors,
            })
        }
    }

    println!("{} -> {}", start, end);

    let (_prev, dist) = bfs(
        graph.into_iter().flatten().enumerate().collect(),
        start,
        len_x,
        len_y,
        grid.clone(),
    );

    println!("{}", dist[&end]);

    let min = grid
        .iter()
        .flatten()
        .map(|s| {
            if *s == 'S' {
                'a'
            } else if *s == 'E' {
                'z'
            } else {
                *s
            }
        })
        .enumerate()
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .take_while(|s| s.1 == 'a')
        .fold(usize::MAX, |acc, i| {
            if let Some(dist) = dist.get(&i.0) {
                acc.min(*dist)
            } else {
                acc
            }
        });

    println!("{}", min);
}

fn find_neighbors(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let len_y = grid.len();
    let len_x = grid[0].len();

    if x > 0 {
        out.push(y * len_x + x - 1);
    }
    if x < len_x - 1 {
        out.push(y * len_x + x + 1);
    }
    if y > 0 {
        out.push((y - 1) * len_x + x);
    }
    if y < len_y - 1 {
        out.push((y + 1) * len_x + x)
    }

    out
}

fn bfs(
    graph: HashMap<usize, Node>,
    start: usize,
    len_x: usize,
    len_y: usize,
    grid: Vec<Vec<char>>,
) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let mut explored = HashSet::new();
    let mut queue = Vec::new();
    let mut prev = HashMap::new();
    let mut dist = HashMap::new();

    explored.insert(start);
    queue.push(start);

    let mut i = 0;
    while !queue.is_empty() {
        let mut string = String::from("\x1B[2J");
        for y in 0..len_y {
            for x in 0..len_x {
                string.push_str(&format!(
                    "{}",
                    if explored.contains(&(y * len_x + x)) {
                        grid[y][x].to_string().cyan()
                    } else {
                        grid[y][x].to_string().white()
                    }
                ))
            }
            string.push('\n')
        }
        println!("{}", string);

        for v in queue.clone().into_iter() {
            dist.insert(v, i);
            queue.remove(0);
            for neighbor in &graph.get(&v).unwrap().neighbors {
                if explored.contains(&neighbor) {
                    continue;
                }
                prev.insert(*neighbor, v);
                explored.insert(*neighbor);
                queue.push(*neighbor);
            }
        }

        i += 1;
    }

    // println!("{:#?}", prev);
    (prev, dist)
}

// fn djikstra(mut graph: HashMap<usize, Node>, start: usize) -> HashMap<usize, usize> {
//     let mut dist = HashMap::new();
//     let mut prev = HashMap::new();

//     for (index, _) in graph.iter() {
//         dist.insert(*index, usize::MAX);
//     }

//     *dist.get_mut(&start).unwrap() = 0;

//     while !graph.is_empty() {
//         let index = graph
//             .iter()
//             .min_by(|a, b| dist.get(a.0).unwrap().cmp(dist.get(b.0).unwrap()))
//             .unwrap()
//             .0
//             .clone();
//         let node = graph.remove(&index).unwrap();

//         for neighbor in &node.neighbors {
//             let alt = dist.get(&index).unwrap() + 1;

//             if alt < *dist.get(&neighbor).unwrap() {
//                 *dist.get_mut(&neighbor).unwrap() = alt;
//                 prev.insert(*neighbor, index);
//             }
//         }
//     }

//     dist
// }
