use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input").expect("Cannot read file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Failed to read file");

    let mut floor = 0;
    let mut i = 0;
    let mut entered_basement = false;

    for char in contents.chars() {
        i += 1;
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}, 
        }

        if floor < 0 && !entered_basement {
            println!("First basement {i}");
            entered_basement = true;
        }
    }

    println!("Current floor: {floor}");
}