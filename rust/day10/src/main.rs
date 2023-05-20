fn main() {
    let input = std::env::args().nth(1).expect("poop");

    let mut cur = input.clone();
    
    for i in 1..=50 {
        cur = next(&cur);
        if i == 40 || i == 50 {
            println!("len[{}] = {}", i, cur.len());
        }
    }
}

fn next(input: &str) -> String {
    let mut previous_count = 0;
    let mut previous = None;
    let mut answer = String::new();
    for current_char in input.chars() {
        if let Some(previous_char) = previous {
            if current_char == previous_char {
                previous_count += 1;
            } else {
                answer.push_str(&format!("{}{}", previous_count, previous_char));
                previous = Some(current_char);
                previous_count = 1;
            }
        } else {
            previous = Some(current_char);
            previous_count = 1;
        }
    }

    answer.push_str(&format!("{}{}", previous_count, previous.unwrap()));
    answer
}
