use arrayvec::ArrayVec;
use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};

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

/// Check if the haystack (which is a base16 stack) contains the provided value
fn path_contains_node(node: usize, path: usize) -> bool {
    path & 0xF == node
        || path >> 4 & 0xF == node
        || path >> 8 & 0xF == node
        || path >> 12 & 0xF == node
        || path >> 16 & 0xF == node
        || path >> 20 & 0xF == node
        || path >> 24 & 0xF == node
        || path >> 28 & 0xF == node
}

fn index_of(needle: &str, haystack: &Vec<&str>) -> usize {
    let mut i = 1;
    for c in haystack {
        if c == &needle {
            return i;
        }
        i += 1;
    }

    panic!("empty haystack");
}

fn search(graph: &Graph) -> (usize, usize) {
    let starting_points = graph
        .iter()
        .flat_map(|((from, to), _)| vec![from, to])
        .collect::<HashSet<_>>();

    let starting_points = starting_points
        .iter()
        .map(|e| e.as_str())
        .collect::<Vec<_>>();

    let point_count = starting_points.len();

    assert!(point_count < 16);

    // Create a graph that can be searched quickly
    let graph = graph
        .iter()
        .map(|((from, to), distance)| {
            (index_of(&from, &starting_points) << 32)
                | (index_of(&to, &starting_points) << 16)
                | *distance
        })
        .collect::<Vec<usize>>();

    let mut buffer = ArrayVec::<(usize, usize), 32>::new();

    for (idx, _start) in starting_points.iter().enumerate() {
        buffer.push((idx + 1, 0));
    }

    let mut best = usize::MAX;
    let mut worst = 0;

    while let Some((cur_path, cur_dist)) = buffer.pop() {
        // If the current path is a complete path (it includes all locations at least once we
        // can use its length to figure out if it's the the best or worst
        if cur_path >> ((point_count - 1) * 4) > 0 {
            best = min(best, cur_dist);
            worst = max(worst, cur_dist);
            continue;
        }

        // Based on our current location find all the locations we can move to
        // For every potential destination figure out if we've already been there
        // if not add the new path to the search buffer
        for &packed in &graph {
            let new_dest = (packed >> 16) & 0xFFFF;
            if (packed >> 32) == (cur_path & 0xF) && !path_contains_node(new_dest, cur_path) {
                // Determine the new path by multiplying by 16 (or shifting left 4 times)
                let new_path = (cur_path << 4) + new_dest;

                // Add the new path to the sarch buffer
                buffer.push((new_path, cur_dist + (packed & 0xFFFF)));
            }
        }
    }

    (best, worst)
}
