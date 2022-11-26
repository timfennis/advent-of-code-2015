fn main() {
    println!("five zeroes: {}", find_start_with("00000"));
    println!("six zeroes: {}", find_start_with("000000"));
}

fn find_start_with(start: &str) -> u32 {
    let mut i = 0;
    loop {
        let hash = format!("{:x}", md5::compute(format!("iwrupvqb{i}")));

        if hash.starts_with(start) {
            return i;
        }

        i += 1;
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_that_it_works() {
        assert_eq!(1, 1);
    }
}
