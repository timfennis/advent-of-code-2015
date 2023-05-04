fn main() {
    let input = include_str!("../input").lines();


    let answer_1: usize = input
        .clone()
        // For the decoded line we need to count the characters instead of the bytes
        .map(|line| line.len() - decode(line).chars().count())
        .sum();

    let answer_2: usize = input
        .clone()
        .map(|line| encode(line).len() - line.len())
        .sum();

    println!("Part 1: {answer_1}");
    println!("Part 2: {answer_2}");
}

fn encode(input: &str) -> String {
    let mut buf = String::new();
    buf.push('"');
    for c in input.chars() {
        match c {
            '"' => {
                buf.push('\\');
                buf.push('"');
            }
            '\\' => {
                buf.push('\\');
                buf.push('\\');
            }
            _ => {
                buf.push(c);
            }
        }
    }
    buf.push('"');
    buf
}

fn decode(input: &str) -> String {
    let mut buf = String::new();
    let mut input = input.chars();
    while let Some(c) = input.next() {
        if c == '\\' {
            match input.next() {
                Some('\\') => {
                    buf.push('\\');
                }
                Some('"') => {
                    buf.push('"');
                }
                Some('x') => {
                    let hex = input.next().and_then(|a| {
                        input.next().and_then(|b| {
                            u8::from_str_radix(&format!("{a}{b}"), 16)
                                .map(|c| c as char)
                                .ok()
                        })
                    });
                    if let Some(c) = hex {
                        buf.push(c);
                    } else {
                        panic!("what is happening here");
                    }
                }
                _ => panic!("syntax error?"),
            }
        } else if c == '"' {
        } else {
            buf.push(c)
        }
    }
    buf
}
