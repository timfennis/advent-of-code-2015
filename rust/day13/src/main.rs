use anyhow::Context;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    let rules = include_str!("../input")
        .lines()
        .map(|line| line.parse::<Rule>())
        .collect::<Result<Vec<_>, _>>()?;

    let people = rules
        .iter()
        .map(|rule| rule.subject.as_str())
        .collect::<HashSet<_>>();

    let mut first_best = 0i64;
    let mut first_worst = i64::max_value();

    for perm in people.iter().permutations(people.len()) {
        let score = score(&perm, &rules);
        first_best = std::cmp::max(first_best, score);
        first_worst = std::cmp::min(first_worst, score);
    }

    println!("Best: {first_best}\nWorst: {first_worst}");

    // Part 2
    let mut rules = rules.clone();
    for person in people {
        rules.push(Rule {
            subject: person.to_string(),
            amount: 0,
            target: "Tim".to_string(),
        });
        rules.push(Rule {
            subject: "Tim".to_string(),
            amount: 0,
            target: person.to_string(),
        });
    }

    let people = rules
        .iter()
        .map(|rule| rule.subject.as_str())
        .collect::<HashSet<_>>();

    let mut second_best = 0i64;
    let mut second_worst = i64::max_value();

    for perm in people.iter().permutations(people.len()) {
        let score = score(&perm, &rules);
        second_best = std::cmp::max(second_best, score);
        second_worst = std::cmp::min(second_worst, score);
    }

    println!("Best: {second_best}\nWorst: {second_worst}");
    Ok(())
}

fn score(order: &[&&str], rules: &[Rule]) -> i64 {
    let mut score = 0;
    for (&&a, &&b, &&c) in order.iter().circular_tuple_windows() {
        for rule in rules {
            if b == rule.subject {
                if a == rule.target {
                    score += rule.amount as i64;
                } else if c == rule.target {
                    score += rule.amount as i64;
                }
            }
        }
    }

    score
}

impl FromStr for Rule {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)\.$"
            )
            .unwrap();
        }

        RE.captures(s)
            .and_then(|cap| {
                Some(Rule {
                    subject: cap.get(1)?.as_str().to_owned(),
                    amount: if cap.get(2)?.as_str() == "gain" {
                        cap.get(3)?.as_str().parse::<i16>().ok()?
                    } else {
                        -cap.get(3)?.as_str().parse::<i16>().ok()?
                    },
                    target: cap.get(4)?.as_str().to_owned(),
                })
            })
            .context("kut")
    }
}

#[derive(Debug, Clone)]
struct Rule {
    subject: String,
    amount: i16,
    target: String,
}
