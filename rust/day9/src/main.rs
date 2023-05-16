use std::collections::{HashMap, HashSet};

type Graph = HashMap<(String, String), usize>;

fn main() {
    let graph = build_graph();
    let (shortest_path, longest_path) = search(&graph);
    println!("Part 1: {}", shortest_path);
    println!("Part 2: {}", longest_path);
}

fn build_graph() -> Graph {
    let lines = include_str!("../input").lines();

    let mut graph: Graph = HashMap::new();
    for line in lines {
        let parts = line.split(' ').collect::<Vec<_>>();

        let from = parts.first().expect("Cannot parse from");
        let to = parts.get(2).expect("Cannot parse to");
        let distance = parts
            .get(4)
            .and_then(|dst| dst.parse::<usize>().ok())
            .expect("Cannot parse distance");

        graph.insert((from.to_string(), to.to_string()), distance);
        graph.insert((to.to_string(), from.to_string()), distance);
    }

    graph
}

fn search(graph: &Graph) -> (usize, usize) {
    let mut buffer: Vec<(Vec<&str>, usize)> = Vec::with_capacity(100);

    let starting_points = graph
        .iter()
        .flat_map(|((from, to), _)| vec![from, to])
        .collect::<HashSet<_>>();

    for start in &starting_points {
        buffer.push((vec![start], 0));
    }

    let mut best = usize::MAX;
    let mut worst = 0;

    while let Some((cur_path, cur_dist)) = buffer.pop() {
        // If the current path is a complete path (it includes all locations at least once we
        // can use its length to figure out if it's the the best or worst
        if cur_dist < best && cur_path.len() == starting_points.len() {
            best = cur_dist;
            continue;
        }

        if cur_dist > worst && cur_path.len() == starting_points.len() {
            worst = cur_dist;
            continue;
        }

        // If the current path is incomplete we figure out where we are now
        let current_location = cur_path.last().unwrap();

        // Based on our current location find all the locations we can move to
        let destinations = graph
            .iter()
            .filter(|((from, _), _)| from == current_location);

        // For every potential destination figure out if we've already been there
        // if not add the new path to the search buffer
        for ((_, new_dest), dst_to_dest) in destinations {
            if !cur_path.contains(&new_dest.as_str()) {
                // So far this is the fastest way that I've been able to find to add a single
                // element to a vector
                let new_path = cur_path
                    .iter()
                    .chain([&new_dest.as_str()])
                    .cloned()
                    .collect();
                buffer.push((new_path, cur_dist + dst_to_dest));
            }
        }
    }

    (best, worst)
}
