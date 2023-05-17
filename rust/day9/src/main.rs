use arrayvec::ArrayVec;
use std::cmp::{max, min};
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

fn index_of(needle: &str, haystack: &HashSet<&str>) -> usize {
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
    // Walk the graph to figure out all potential points where we could start
    // We dedup the points by collecting into a set
    let starting_points = graph
        .iter()
        .flat_map(|((from, to), _)| vec![from.as_str(), to.as_str()])
        .collect::<HashSet<_>>();

    let point_count = starting_points.len();

    // We're going to be using a usize as a stack of u4 ints, we need to ensure that no point
    // index exceedes the value 16 otherwise numbers may overflow
    assert!(point_count < 16);

    // Pack the graph into a Vec<usize> like this:
    // 0000000000000000 0000000000000000 0000000000000000 0000000000000000
    //                     from_index        to_index         distance
    // The theory behind this is that this vec is much faster to traverse than other datastructures
    // with tuples etc
    let graph = graph
        .iter()
        .map(|((from, to), distance)| {
            (index_of(&from, &starting_points) << 32)
                | (index_of(&to, &starting_points) << 16)
                | *distance
        })
        .collect::<Vec<usize>>();

    // We are cheating a little here because we've determined ahead of time that the buffer will
    // never exceed the capacity of 32 removing the need to reallocate the buffer while running
    let mut buffer = Vec::<(usize, usize)>::with_capacity(32);

    // Put every index between 1 and the point_count in the buffer to consider every points as
    // a potential starting point
    for idx in 1..=point_count {
        buffer.push((idx, 0));
    }

    let mut best = usize::MAX;
    let mut worst = 0;

    while let Some((cur_path, cur_dist)) = buffer.pop() {
        // Based on our current location find all the locations we can move to for every potential
        // destination figure out if we've already been there, if not we add the new path to the
        // search buffer
        for &packed in &graph {
            let new_dest = (packed >> 16) & 0xFFFF;

            // Find all entries in the graph that have our current point as a starting point and
            // who's destination is not already part of the current path. We can find the current
            // point by performing a bitwise AND between the current path and 0xF (get the last 4
            // bits)
            if (packed >> 32) == (cur_path & 0xF) && !path_contains_node(new_dest, cur_path) {
                // If we haven't traveled here yet we can add the new destination to the path
                let new_path = (cur_path << 4) + new_dest;
                let new_dist = (packed & 0xFFFF) + cur_dist;

                // If the current path is a complete path (it includes all locations at least once,
                // we can use its length to figure out if it's the the best or worst
                // HACK: we rightshift the current path by `point_count + 1 * 4` and check if these
                //       bits are set by comparing against 0 to see if this is a complete path.
                //       This only works because 0 isn't a valid point index
                if new_path >> ((point_count - 1) * 4) > 0 {
                    best = min(best, new_dist);
                    worst = max(worst, new_dist);
                } else {
                    // Add the new path to the end of the search buffer and add the distance
                    // between the nodes to the distance value
                    buffer.push((new_path, new_dist));
                }
            }
        }
    }

    (best, worst)
}
