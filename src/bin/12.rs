use std::{time::Instant, collections::HashMap};
const DAY_NUMBER: u32 = 12;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

#[derive(Debug, Clone)]
struct Node {
    pub id: usize,
    pub neighbors: Vec<usize>,
}

fn solve(input: impl Iterator<Item = String>, start: Instant) {
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
            let mut height = grid[y][x] as usize;
            if grid[y][x] == 'S' {
                start = y * len_x + x;
                height = 'a' as usize
            }
            else if grid[y][x] == 'E' {
                end = y * len_x + x;
                height = 'z' as usize
            }

            neighbors.retain(|neighbor| (grid[neighbor / len_x][neighbor % len_x] as usize).abs_diff(height) > 1);


            graph[y].push(Node {
                id: y * len_x + x,
                neighbors
            })
        }
    }

    let dist = djikstra(graph.into_iter().flatten().collect(), start);

    println!("{}", dist.get(&end).unwrap())
}

fn find_neighbors(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let len_y = grid.len();
    let len_x = grid[0].len();
    
    if x > 1 {
        out.push(y * len_x + x - 1);
    }
    if x < grid[0].len() - 1 {
        out.push(y * len_x + x + 1);
    }
    if y > 1 {
        out.push((y - 1) * len_x + x);
    }
    if y < grid.len() - 1 {
        out.push((y + 1) * len_x + x)
    }

    out
}

fn djikstra(mut graph: Vec<Node>, start: usize) -> HashMap<usize, u32> {
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();

    for node in graph.iter() {
        dist.insert(node.id, u32::MAX);
    }

    *dist.get_mut(&start).unwrap() = 0;

    while !graph.is_empty() {
        let node = graph.pop().unwrap();
        // TODO: Find node with least distance
        for neighbor in node.neighbors {
            let alt = dist.get(&node.id).unwrap() + 1;

            if alt < *dist.get(&neighbor).unwrap() {
                *dist.get_mut(&neighbor).unwrap() = alt;
                prev.insert(neighbor, node.id);
            }
        }
    }

    dist
}
