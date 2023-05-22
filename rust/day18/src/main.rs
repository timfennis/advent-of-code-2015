use std::collections::HashSet;

const EIGHT_CONNECTED: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
fn main() {
    let display = include_str!("../input")
        .lines()
        .enumerate()
        .flat_map(|(line_nr, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(light_nr, state)| {
                    if state == '#' {
                        Some((light_nr as i32, line_nr as i32))
                    } else {
                        None
                    }
                })
        })
        .collect::<HashSet<(i32, i32)>>();

    let mut current = display.clone();
    current.insert((0, 0));
    current.insert((0, 99));
    current.insert((99, 0));
    current.insert((99, 99));

    for i in 0..100 {
        let mut next = HashSet::new();

        for x in 0..100 {
            for y in 0..100 {
                let state = current.contains(&(x, y));

                let mut amount_enabled = 0;
                for (dx, dy) in EIGHT_CONNECTED {
                    let (nx, ny) = (x + dx, y + dy);
                    if current.contains(&(nx, ny)) {
                        amount_enabled += 1;
                    }

                    if amount_enabled >= 4 {
                        break;
                    }
                }

                if state && (amount_enabled == 2 || amount_enabled == 3) {
                    next.insert((x, y));
                } else if !state && amount_enabled == 3 {
                    next.insert((x, y));
                }
            }
        }

        current = next;
        current.insert((0, 0));
        current.insert((0, 99));
        current.insert((99, 0));
        current.insert((99, 99));

        println!("{} = {}", i, current.len());
    }
}
