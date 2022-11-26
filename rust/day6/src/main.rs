use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    part_1();
    part_2();
}

fn part_2() {
    let mut grid = vec![0; 1000000];

    let input = std::fs::read_to_string("input").unwrap();

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(toggle|turn off|turn on) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    }

    for instruction in input.split("\n") {
        for cap in RE.captures_iter(instruction) {
            let op = match &cap[1] {
                "turn on" => Op::ON,
                "turn off" => Op::OFF,
                "toggle" => Op::TOGGLE,
                _ => panic!("Kapot"),
            };

            let x1 = cap[2].parse::<usize>().unwrap();
            let y1 = cap[3].parse::<usize>().unwrap();
            let x2 = cap[4].parse::<usize>().unwrap();
            let y2 = cap[5].parse::<usize>().unwrap();

            for x in x1..=x2 {
                for y in y1..=y2 {
                    grid[y*999+x] = match op {
                        Op::ON => grid[y*999+x] + 1,
                        Op::OFF => std::cmp::max(0, grid[y*999+x] - 1),
                        Op::TOGGLE => grid[y*999+x] + 2,
                    }
                }
            }
        }
    }


    let mut i = 0u64;
    for light_state in grid.into_iter() {
        i += light_state as u64;
    }

    println!("Count: {i}");
}
fn part_1() {
    let mut grid = vec![false; 1000000];

    let input = std::fs::read_to_string("input").unwrap();

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(toggle|turn off|turn on) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    }

    for instruction in input.split("\n") {
        for cap in RE.captures_iter(instruction) {
            let op = match &cap[1] {
                "turn on" => Op::ON,
                "turn off" => Op::OFF,
                "toggle" => Op::TOGGLE,
                _ => panic!("Kapot"),
            };

            let x1 = cap[2].parse::<usize>().unwrap();
            let y1 = cap[3].parse::<usize>().unwrap();
            let x2 = cap[4].parse::<usize>().unwrap();
            let y2 = cap[5].parse::<usize>().unwrap();

            for x in x1..=x2 {
                for y in y1..=y2 {
                    grid[y*999+x] = match op {
                        Op::ON => true,
                        Op::OFF => false,
                        Op::TOGGLE => !grid[y*999+x],
                    }
                }
            }
        }
    }


    let mut i = 0;
    for light_state in grid.into_iter() {
        if light_state {
            i += 1;
        }
    }

    println!("Count: {i}");
}
#[derive(Debug)]
enum Op {
    ON, OFF, TOGGLE
}