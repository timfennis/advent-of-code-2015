use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-zA-Z0-9 ]+) -> (\w+)").unwrap();
        static ref RE2: Regex = Regex::new(r"(\w+ )?([A-Z]+) (\w+)").unwrap();
    }
    let input = std::fs::read_to_string("input").unwrap();
    let mut registers = HashMap::new();

    let mut loops = 1;
    loop {
        for inst in input.split("\n") {
            for cap in RE.captures_iter(inst) {
                let left = &cap[1];
                let right = &cap[2];

                // Skip the entire instruction if the register already has a value
                if registers.contains_key(right) {
                    continue;
                }
                
                let literal_left = left
                    .parse::<u16>()
                    .ok()
                    .or_else(|| registers.get(left).map(|it| *it));

                match literal_left {
                    Some(ll) => {
                        registers.insert(right.to_string(), ll as u16);
                    }
                    None => {}
                }

                for cap in RE2.captures_iter(left) {
                    let l1 = cap.get(1).map(|it| it.as_str().trim());
                    let op = cap.get(2).map(|it| it.as_str().trim());
                    let l2 = cap.get(3).map(|it| it.as_str().trim());

                    let ll1 = l1
                        .and_then(|it| it.parse::<u16>().ok())
                        .or_else(|| l1.and_then(|it| registers.get(it)).map(|x| *x));
                    let ll2 = l2
                        .and_then(|it| it.parse::<u16>().ok())
                        .or_else(|| l2.and_then(|it| registers.get(it)).map(|x| *x));

                    if let (Some(ll1), Some(op), Some(ll2)) = (ll1, op, ll2) {
                        match op {
                            "RSHIFT" => {
                                registers.insert(right.to_string(), ll1 >> ll2);
                            }
                            "LSHIFT" => {
                                registers.insert(right.to_string(), ll1 << ll2);
                            }
                            "AND" => {
                                registers.insert(right.to_string(), ll1 & ll2);
                            }
                            "OR" => {
                                registers.insert(right.to_string(), ll1 | ll2);
                            }
                            _ => {
                                panic!("Unexpected op {op}");
                            }
                        }
                    }

                    if let (Some(op), Some(ll2)) = (op, ll2) {
                        if op == "NOT" {
                            // println!("resolving {op} {ll2} -> {right}");
                            registers.insert(right.to_string(), !ll2);
                        }
                    }
                }
            }
        }

        if registers.contains_key("a") {
            let solution = *registers.get("a").unwrap();
            println!("Solution: {}", solution);
            
            if loops == 0 {
                println!("Done!");
                break;
            }
            registers.clear();
            registers.insert("b".to_string(), solution);
            loops -= 1; 
        }
    }
}
