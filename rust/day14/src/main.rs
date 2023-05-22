use std::collections::HashMap;

fn main() {
    let lines = include_str!("../input").lines();
    let mut max_distance = 0; 
    let mut horses = HashMap::new();


    println!("Part 1: race for 2503 seconds");
    for line in lines {
        let tokens = line.split_whitespace().collect::<Vec<_>>();
        let val = (tokens.get(3).and_then(|x| x.parse::<u32>().ok()).unwrap(),
        tokens.get(6).and_then(|x| x.parse::<u32>().ok()).unwrap(),
        tokens.get(13).and_then(|x| x.parse::<u32>().ok()).unwrap());
        
        let distance = calculate_distance(2503, val);

        max_distance = std::cmp::max(distance, max_distance);
        println!("{} {}", tokens.get(0).unwrap(), distance);

        horses.insert(tokens.get(0).unwrap().to_string(), val);
    }

    let mut scores: HashMap<&str, u32> = HashMap::new();
    println!("Max distance: {max_distance}");

    println!("\nPart 2: give the leading horse 1 point each second");
    for i in 1..=2503 {
        let mut best = 0; 
        let mut cur_best = None;
        for (name, horse) in &horses {
            let cur_score = calculate_distance(i, *horse);
            if cur_score > best {
                cur_best = Some(name);
                best = cur_score;
            }
        }
        scores.entry(cur_best.unwrap()).and_modify(|e| *e += 1).or_insert(1);
    }

    let (winning_horse, winning_score) = scores.iter().max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
    println!("The winning horse is {winning_horse} with a score of {winning_score}");

}

fn calculate_distance(target_time: u32, (up_speed, up_time, down_time): (u32, u32, u32)) -> u32
{
    let interval = up_time + down_time;
    let full_cycles = target_time / interval;
    let remaining_seconds = target_time % interval;

    (full_cycles * (up_time * up_speed)) + (std::cmp::min(remaining_seconds, up_time) * up_speed)
}
