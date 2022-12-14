use std::time::Instant;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
const DAY_NUMBER: u32 = 13;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Package {
    Array(Vec<Self>),
    Value(u32),
}

fn solve(input: impl Iterator<Item = String>, start: Instant) {
    let mut input = input.filter(|x| !x.is_empty()).collect::<Vec<_>>();

    let chunks = input.iter().chunks(2);
    let chunks = chunks
        .into_iter()
        .map(|mut c| (c.next().unwrap(), c.next().unwrap()));

    let mut out = 0;

    for (i, (lhs, rhs)) in chunks.enumerate() {
        // println!("{} vs {}", lhs, rhs);
        let lhs: Package = serde_json::from_str(&lhs).unwrap();
        let rhs: Package = serde_json::from_str(&rhs).unwrap();

        let result = is_sorted(lhs, rhs).unwrap();
        // println!("{}\n", result);

        out += result as usize * (i + 1);
    }

    println!("{}", out)

    input.sort_by(|lhs, rhs| {
        let lhs: Package = serde_json::from_str(&lhs).unwrap();
        let rhs: Package = serde_json::from_str(&rhs).unwrap();

        let result = is_sorted(lhs, rhs);
    })
}

fn is_sorted(lhs: Package, rhs: Package) -> Option<bool> {
    match (lhs, rhs) {
        (Package::Value(a), Package::Value(b)) => match (a, b) {
            _ if a < b => Some(true),
            _ if a == b => None,
            _ if a > b => Some(false),
            _ => unreachable!(),
        },
        (Package::Array(arr_a), Package::Array(arr_b)) => {
            let mut iter_a = arr_a.into_iter();
            let mut iter_b = arr_b.into_iter();

            let mut out = None;

            while let None = out {
                let lhs = iter_a.next();
                let rhs = iter_b.next();

                match (lhs, rhs) {
                    (Some(lhs), Some(rhs)) => out = is_sorted(lhs, rhs),
                    (None, Some(_)) => out = Some(true),
                    (Some(_), None) => out = Some(false),
                    (None, None) => break,
                }
            }

            out
        }
        (Package::Array(arr), rhs) => is_sorted(Package::Array(arr), Package::Array(vec![rhs])),
        (lhs, Package::Array(arr)) => is_sorted(Package::Array(vec![lhs]), Package::Array(arr)),
    }
}
