use std::{collections::HashMap, time::Instant};

use colored::{ColoredString, Colorize};
const DAY_NUMBER: u32 = 8;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

fn solve(input: impl Iterator<Item = String>, start: Instant) {
    let input: Vec<Vec<i32>> = input
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();
    let height = input.len();
    let width = input.first().unwrap().len();
    let mut visible = vec![vec![false; width]; height];

    for y in 0..height {
        let mut max = -1;

        for x in 0..width {
            if input[y][x] > max {
                visible[y][x] = true;
                max = max.max(input[y][x])
            } else if max >= 9 {
                break;
            }
        }

        let mut max = -1;

        for x in (0..width).rev() {
            if input[y][x] > max {
                visible[y][x] = true;
                max = max.max(input[y][x])
            } else if max >= 9 {
                break;
            }
        }
    }

    for x in 0..width {
        let mut max = -1;

        for y in 0..height {
            if input[y][x] > max {
                visible[y][x] = true;
                max = max.max(input[y][x])
            } else if max >= 9 {
                break;
            }
        }

        let mut max = -1;

        for y in (0..height).rev() {
            if input[y][x] > max {
                visible[y][x] = true;
                max = max.max(input[y][x])
            } else if max >= 9 {
                break;
            }
        }
    }

    let sum: u32 = visible
        .iter()
        .flatten()
        .into_iter()
        .fold(0, |acc, v| acc + *v as u32);
    println!("{}", sum);

    let mut max_score = 0;
    let mut direction_scores = HashMap::new();

    for x in 0..width {
        for y in 0..height {
            let mut score = 1;
            let mut scores = [0; 4];
            let max = input[y][x];

            if x > 0 {
                let mut direction_score = 0;
                for dx in (0..x).rev() {
                    direction_score += 1;
                    if input[y][dx] >= max {
                        break;
                    }
                }
                score *= direction_score;
                scores[0] = direction_score;
            } else {
                score *= 0;
            }

            if x + 1 < width {
                let mut direction_score = 0;
                for dx in (x + 1)..width {
                    direction_score += 1;
                    if input[y][dx] >= max {
                        break;
                    }
                }
                score *= direction_score;
                scores[1] = direction_score;
            } else {
                score *= 0;
            }

            if y > 0 {
                let mut direction_score = 0;
                for dy in (0..y).rev() {
                    direction_score += 1;
                    if input[dy][x] >= max {
                        break;
                    }
                }
                score *= direction_score;
                scores[2] = direction_score;
            } else {
                score *= 0;
            }

            if y + 1 < height {
                let mut direction_score = 0;
                for dy in (y + 1)..height {
                    direction_score += 1;
                    if input[dy][x] >= max {
                        break;
                    }
                }
                score *= direction_score;
                scores[3] = direction_score;
            } else {
                score *= 0;
            }

            direction_scores.insert(score, ((x, y), scores));
            max_score = max_score.max(score);
        }
    }

    let ((target_x, target_y), diffs) = *direction_scores.get(&max_score).unwrap();
    for y in 0..height {
        for x in 0..width {
            if (target_x - diffs[0] <= x && x <= target_x + diffs[1] && y == target_y)
                || (target_y - diffs[2] <= y && y <= target_y + diffs[3] && x == target_x)
            {
                print!("{}", input[y][x].to_string().cyan())
            } else {
                print!("{}", input[y][x])
            }
        }
        println!();
    }

    println!("{:?}", direction_scores.get(&max_score).unwrap());
    println!("{}", max_score)
}
