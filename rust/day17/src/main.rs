use std::cmp::Ordering;

fn main() {
    let mut containers = include_str!("../input")
        .lines()
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<_>>();

    containers.sort();
    containers.reverse();

    // Find all different ways we can arrange the containers in order to get to an exact
    // maximum size of 150
    let options = dfs(150, 0, &containers);
    println!("Part 1: {}", options.len());
    assert_eq!(4372, options.len()); // assert the answer is correct

    // Find the minimum number of containers that can be used to get to the total of 150
    let min = options.iter().min().unwrap();

    // Count the number of arrangements that match the minimum amount
    let count = options.iter().filter(|v| *v == min).count();
    println!("Part 2: {count}");
    assert_eq!(4, count); // assert the answer is correct
}

/// Search all combinations of `containers` that make up a total of `size_remaining`
/// This method only works if `containers` is sorted from high to low.
fn dfs(size_remaining: usize, container_count: usize, containers: &[usize]) -> Vec<usize> {
    let mut results = Vec::new();

    for (idx, cur_size) in containers.iter().enumerate() {
        match size_remaining.cmp(cur_size) {
            Ordering::Equal => {
                results.push(container_count);
            }
            Ordering::Greater => {
                let potential_containers = containers[idx + 1..]
                    .iter()
                    .filter(|cc| **cc <= size_remaining)
                    .cloned()
                    .collect::<Vec<_>>();

                if !potential_containers.is_empty() {
                    let intermediate = dfs(
                        size_remaining - cur_size,
                        container_count + 1,
                        &potential_containers,
                    );
                    results.extend_from_slice(&intermediate);
                }
            }
            _ => {}
        }
    }

    results
}
