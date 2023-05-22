use anyhow::Context;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let mut sues: HashMap<u16, HashMap<&str, u16>> = HashMap::new();
    let lines = include_str!("../input").lines().filter(|l| !l.is_empty());
    for line in lines {
        let (sue, props) = line
            .split_once(':')
            .context("Invalid input line: no colon")?;
        let num = sue
            .split_once(' ')
            .and_then(|(_, num)| num.parse::<u16>().ok())
            .context("Invalid input: no number found")?;

        let props = props
            .trim()
            .split(',')
            .flat_map(|p| {
                p.split_once(':')
                    .and_then(|(p, v)| v.trim().parse::<u16>().ok().map(|v| (p.trim(), v)))
            })
            .collect::<HashMap<_, _>>();

        sues.insert(num, props);
    }

    let search_props: HashMap<&str, u16> = HashMap::from_iter(vec![
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);

    let part_1 = sues.iter().find(|(_, props)| {
        for (prop, search_val) in &search_props {
            match props.get(prop) {
                Some(x) if *x != *search_val => return false,
                _ => {},
            }
        }
        true
    });

    println!("Part 1: {:?}", part_1);
    let part_2 = sues.iter().find(|(_, props)| {
        for (prop, search_val) in &search_props {
            if let Some(x) = props.get(prop) {
                match *prop {
                    "cats" | "trees" => {
                        if *x <= *search_val {
                            return false;
                        }
                    }
                    "pomeranians" | "goldfish" => {
                        if *x >= *search_val {
                            return false;
                        }
                    }
                    _ => {
                        if *x != *search_val {
                            return false;
                        }
                    }
                }
            }
        }
        true
    });

    println!("Part 2: {:?}", part_2);

    Ok(())
}
