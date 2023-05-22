use std::cmp::max;

fn main() {
    println!("Part 1: {}", calc_best(|_, _, _, _| true));
    println!(
        "Part 2: {}",
        calc_best(|f, c, b, s| { (f * 5) + (c * 8) + (b * 6) + (s) == 500 })
    );
}

fn calc_best(predicate: fn(i32, i32, i32, i32) -> bool) -> i32 {
    let mut best = 0;
    for f in 0..=100 {
        let rem = 100 - f;
        for c in 0..=rem {
            let rem = 100 - f - c;
            for b in 0..=rem {
                let rem = 100 - f - c - b;
                for s in 0..=rem {
                    if predicate(f, c, b, s) {
                        // Frosting: capacity 4, durability -2, flavor 0, texture 0, calories 5
                        // Candy: capacity 0, durability 5, flavor -1, texture 0, calories 8
                        // Butterscotch: capacity -1, durability 0, flavor 5, texture 0, calories 6
                        // Sugar: capacity 0, durability 0, flavor -2, texture 2, calories 1
                        let (fs, cs, bs, ss) = (
                            max(0, (f * 4) - b),                  // capacity
                            max(0, (f * -2) + (c * 5)),           // durability
                            max(0, (0 - c) + (b * 5) + (s * -2)), // flavor
                            s * 2,                                // texture
                        );

                        let score = fs * cs * bs * ss;

                        best = max(best, score);
                    }
                }
            }
        }
    }

    best
}
