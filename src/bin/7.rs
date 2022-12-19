use std::{time::Instant, collections::HashMap, fmt::Display};

use itertools::Itertools;
const DAY_NUMBER: u32 = 7;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

#[derive(Debug, Clone)]
enum Item {
    Directory(HashMap<String, Self>),
    File(usize)
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn traverse(tree: &Item, f: &mut std::fmt::Formatter<'_>, layer: usize, name: &str) -> std::fmt::Result {
            let indent = (0..layer).fold(String::new(), |acc, _| acc + "\t");
            match tree {
                Item::Directory(children) => {
                    writeln!(f, "{}- {}:", indent, name)?;
                    for (name, child) in children {
                        traverse(child, f, layer + 1, name)?;
                    }
                },
                Item::File(size) => {
                    writeln!(f, "{}- {} {}", indent, name, size)?;
                }
            };
            Ok(())
        }
        traverse(self, f, 0, "/")
    }
}

fn solve(mut input: impl Iterator<Item = String>, start: Instant) {
    input.next();

    let input = input.join("\n");
    let input = input.trim().split("$").collect::<Vec<_>>();

    let mut root = Item::Directory(HashMap::new());
    let mut path = Vec::new();

    for line in input {
        if line.is_empty() {
            continue;
        }

        let mut line = if line.starts_with("cd") {
            line.trim().split(" ")
        }
        else {
            line.trim().split("\n")
        };

        let command = line.next().unwrap();

        match &command[..2] {
            "cd" => {
                let arg = &command[3..];
                if arg == ".." {
                    path.pop().unwrap();
                }
                else {
                    path.push(arg.to_string());
                }
            },
            "ls" => {
                let args = line;
                let currend_directory = follow_path(&path, &mut root);
                if let Item::Directory(children) = currend_directory {
                    for arg in args {
                        let mut split = arg.split_whitespace();
                        let determiner = split.next().unwrap();
                        if let Ok(size) = determiner.parse::<usize>() {
                            children.insert(split.next().unwrap().to_string(), Item::File(size));
                        }
                        else {
                            children.insert(split.next().unwrap().to_string(), Item::Directory(HashMap::new()));
                        }
                    }
                }
            },
            _ => unreachable!()
        }
    }
    // println!("{}", root);

    let out = sum_dirs_smaller_than(&root, 100000);
    println!("{}", out);

    let unused = 70000000 - get_size(&root);
    let needed = 30000000 - unused;
    // println!("{}", needed);
    let mut options = find_dirs_bigger_than(&root, needed);
    options.sort_by(|a, b| get_size(a).cmp(&get_size(b)));

    // println!("{:?}", options);
    println!("{}", get_size(&options.first().unwrap()))
}

fn follow_path<'a>(path: &[String], tree: &'a mut Item) -> &'a mut Item {
    if path.is_empty() {
        return tree;
    }
    match tree {
        Item::File(_) => unreachable!(),
        Item::Directory(children) => {
            let next = path.first().unwrap().trim();
            follow_path(&path[1..], children.get_mut(next).unwrap())
        }
    }
}

fn get_size(item: &Item) -> usize {
    match item {
        Item::File(size) => *size,
        Item::Directory(children) => {
            children.values().fold(0, |acc, child| acc + get_size(child))
        }
    }
}

fn sum_dirs_smaller_than(root: &Item, size: usize) -> usize {
    match root {
        Item::File(_) => 0,
        Item::Directory(children) => {
            let self_size = get_size(root);
            let mut out = 0;
            if self_size <= size {
                out += self_size;
            }

            out + children.values().fold(0, |acc, child| acc + sum_dirs_smaller_than(child, size))
        }
    }
}

fn find_dirs_bigger_than(root: &Item, size: usize) -> Vec::<Item> {
    match root {
        Item::File(_) => vec![],
        Item::Directory(children) => {
            let mut out = Vec::new();
            if get_size(root) >= size {
                out.push(root.clone());
            }
            out.extend(children.values().filter(|child| get_size(child) >= size).fold(Vec::new(), |mut acc, child| {
                acc.extend(find_dirs_bigger_than(child, size));
                acc
            }));
            out
        }
    }
}