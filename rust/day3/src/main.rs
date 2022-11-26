use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input").expect("Unable to read input file");
    println!("Houses visited: {}", calculate_houses_visited(&input));
    println!(
        "Houses visited part 2: {}",
        calculate_houses_visited_robo(&input)
    );
}

fn calculate_houses_visited(pattern: &str) -> usize {
    let mut locations = HashMap::new();
    locations.insert((0, 0), 1);

    let mut location = (0, 0);

    for m in pattern.chars() {
        match m {
            '>' => location = (location.0 + 1, location.1),
            '<' => location = (location.0 - 1, location.1),
            '^' => location = (location.0, location.1 - 1),
            'v' => location = (location.0, location.1 + 1),
            _ => {}
        }

        match locations.get(&location) {
            Some(amount) => {
                locations.insert(location, amount + 1);
            }
            None => {
                locations.insert(location, 1);
            }
        }
    }

    return locations.len();
}

fn calculate_houses_visited_robo(pattern: &str) -> usize {
    let mut locations = HashMap::new();
    locations.insert((0, 0), 2);

    let mut santa_location = (0, 0);
    let mut robo_location = (0, 0);
    let mut i = 0;

    for m in pattern.chars() {
        let location = if i % 2 == 0 {
            santa_location = modify_location(m, santa_location);
            santa_location
        } else {
            robo_location = modify_location(m, robo_location);
            robo_location
        };

        i += 1;

        match locations.get(&location) {
            Some(amount) => {
                locations.insert(location, amount + 1);
            }
            None => {
                locations.insert(location, 1);
            }
        }
    }

    return locations.len();
}

fn modify_location(direction: char, (x, y): (i32, i32)) -> (i32, i32) {
    return match direction {
        '>' => (x + 1, y),
        '<' => (x - 1, y),
        '^' => (x, y - 1),
        'v' => (x, y + 1),
        _ => panic!("Invalid input"),
    };
}

#[cfg(test)]
mod tests {
    use crate::{calculate_houses_visited, calculate_houses_visited_robo};

    #[test]
    fn test_example_input() {
        assert_eq!(calculate_houses_visited(">"), 2);
        assert_eq!(calculate_houses_visited("^>v<"), 4);
        assert_eq!(calculate_houses_visited("^v^v^v^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_example_input_part2() {
        assert_eq!(calculate_houses_visited_robo("^v"), 3);
        assert_eq!(calculate_houses_visited_robo("^>v<"), 3);
        assert_eq!(calculate_houses_visited_robo("^v^v^v^v^v"), 11);
    }
}
