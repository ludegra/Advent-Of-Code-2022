use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Write,
    time::Instant,
};

use itertools::Itertools;
const DAY_NUMBER: u32 = 16;

fn main() {
    let input = aoc_2022::read_lines(DAY_NUMBER);
    let start = Instant::now();
    solve(input, start);
}

#[derive(Debug, Clone)]
struct Valve {
    pub rate: u32,
    pub connections: Vec<String>,
    pub active: bool,
}

fn solve(input: impl Iterator<Item = String>, start: Instant) {
    let mut valves = HashMap::new();

    for line in input {
        let re = regex::Regex::new(";?,? (rate=)?").unwrap();
        let mut split = re.split(&line);

        let name = split.nth(1).unwrap().to_owned();
        let rate = split.nth(2).unwrap().parse::<u32>().unwrap();
        let connections = split.skip(4).map(|s| s.to_owned()).collect::<Vec<_>>();

        print!("{}, ", name);
        print!("{}, ", rate);
        println!("{:?}", connections);

        valves.insert(
            name,
            Valve {
                rate,
                connections,
                active: false,
            },
        );
    }

    let mut distances = HashMap::new();
    for (valve, _) in valves.iter().filter(|v| v.1.rate != 0 || v.0 == "AA") {
        println!("{}", valve);
        let mut distance_map = djikstra(&valves, valve);
        distance_map.retain(|valve, _| valves[valve].rate != 0);
        distances.insert(valve.clone(), distance_map);
    }
    // println!();

    let mut f = File::create("./test").unwrap();
    let paths = generate_path("AA", 26, HashSet::new(), &distances);
    writeln!(f, "{:#?}", paths).unwrap();

    // let (mitigated, path) = test_path("AA", HashSet::new(), 30, &distances, &valves, 0, &mut f);

    // println!("{:?}", path);
    // println!("{}", mitigated);
}

fn djikstra(graph: &HashMap<String, Valve>, start: &str) -> HashMap<String, u32> {
    let mut dist = HashMap::new();

    let mut graph = graph.clone();

    for name in graph.keys() {
        dist.insert(name.clone(), u32::MAX);
    }

    *dist.get_mut(start).unwrap() = 0;

    while !graph.is_empty() {
        let index = graph
            .iter()
            .min_by(|a, b| dist[a.0].cmp(&dist[b.0]))
            .unwrap()
            .0
            .clone();
        let node = graph.remove(&index).unwrap();

        for connection in node.connections {
            let alt = dist[&index] + 1;
            if alt < dist[&connection] {
                *dist.get_mut(&connection).unwrap() = alt;
            }
        }
    }

    dist
}

fn test_path(
    valve: &str,
    mut active: HashSet<String>,
    time: u32,
    distances: &HashMap<String, HashMap<String, u32>>,
    valves: &HashMap<String, Valve>,
    indent: u32,
    f: &mut File,
) -> (u32, Vec<String>) {
    // let indent_string = (0..indent).fold(String::new(), |acc, _| acc + " ");
    // writeln!(f, "{}{}: {}", indent_string, valve, 31 - time).unwrap();
    active.insert(valve.to_string());
    let local_distances = &distances[valve];
    let rate = active.iter().fold(0, |acc, v| acc + valves[v].rate);

    let path = vec![valve.to_string()];
    let mut out_path = Vec::new();
    let mut out = rate * time;

    for (other, distance) in local_distances {
        if active.contains(other) {
            continue;
        }
        let dt = distance + 1;
        if time < dt {
            continue;
        }

        let (to_be_mitigated, sub_path) = test_path(
            other,
            active.clone(),
            time - dt,
            distances,
            valves,
            indent + 1,
            f,
        );

        let netto_preasure = to_be_mitigated + rate * dt;

        if out < netto_preasure {
            out_path = [path.clone(), sub_path].into_iter().flatten().collect();
        }

        out = out.max(netto_preasure);
    }
    if out_path.is_empty() {
        out_path = path;
    }

    // writeln!(f, "{}path: {:?}", indent_string, out_path).unwrap();
    (out, out_path)
}

fn generate_path(
    valve: &str,
    time: u32,
    mut active: HashSet<String>,
    distances: &HashMap<String, HashMap<String, u32>>,
) -> HashSet<Vec<String>> {
    active.insert(valve.to_string());
    let path = vec![valve.to_string()];
    let local_distances = &distances[valve];

    let mut paths = HashSet::new(); 
    for (other, distance) in local_distances {
        if active.contains(other) {
            continue;
        }

        let dt = distance + 1;

        if dt > time {
            continue;
        }

        for sub_path in  generate_path(
            other,
            time - dt,
            active.clone(),
            distances
        ) {
            let mut full_path = path.clone();
            full_path.extend(sub_path); 
            paths.insert(full_path);
        }
    }

    paths
}
