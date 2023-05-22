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

    // Find the minimum number of containers that can be used to get to the total of 150
    let min = options.iter().min().unwrap();

    // Count the number of arrangements that match the minimum amount
    let count = options.iter().filter(|v| *v == min).count();
    println!("Part 2: {count}");
}

/// Search all combinations of `containers` that make up a total of `size_remaining`
/// This method only works if `containers` is sorted from high to low.
fn dfs(size_remaining: usize, container_count: usize, containers: &[usize]) -> Vec<usize> {
    let mut results = Vec::new();

    for (idx, cur_size) in containers.iter().enumerate() {
        if size_remaining == 150 {
            println!("idx = {idx} cur_size = {cur_size}");
        }

        if *cur_size == size_remaining {
            results.push(container_count);
        } else if *cur_size < size_remaining {
            let potential_containers = containers[idx + 1..]
                .iter()
                .filter(|cc| **cc <= size_remaining)
                .map(|x| *x)
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
    }

    return results;
}
