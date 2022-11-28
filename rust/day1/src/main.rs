fn main() {
    let contents = std::fs::read_to_string("input").unwrap();

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