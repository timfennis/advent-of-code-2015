use itertools::Itertools;
use std::{fs::read_to_string, collections::HashSet};

fn main() {
    let input = read_to_string("input").unwrap();
    println!("Nice: {}", count_nice(&input, is_nice));
    println!("Nice: {}", count_nice(&input, is_nice2));
}

fn count_nice(input: &str, is_nice: fn(&str) -> bool) -> i32 {
    return input
        .split("\n")
        .fold(0, |sum, string| if is_nice(string) { sum + 1 } else { sum });
}
fn is_nice(string: &str) -> bool {
    let mut vowels = 0;
    let mut prev = None;
    let mut dup = false;

    for char in string.chars() {
        match char {
            'a' | 'e' | 'o' | 'i' | 'u' => {
                vowels += 1;
            }
            _ => {}
        }

        match prev {
            Some(prev) => {
                if dup == false && prev == char {
                    dup = true;
                }
            }
            None => {}
        }

        prev = Some(char);
    }

    let illegal = string.contains("ab")
        || string.contains("cd")
        || string.contains("pq")
        || string.contains("xy");

    return vowels >= 3 && dup && !illegal;
}

fn is_nice2(string: &str) -> bool {
    let mut found = false; 

    'search: for (i, char) in string.chars().enumerate() {
        let (start, end) = string.split_at(i);

        let pairs = start.chars().tuple_windows::<(_, _)>();

        for (a, b) in pairs {
            let substr = format!("{a}{b}");

            if end.contains(&substr) {
                found = true;
                break 'search;
            }
        }
    }

    if !found {
        return false;
    }

    return string
        .chars()
        .tuple_windows::<(_, _, _)>()
        .any(|(a, _, c)| a == c);
}

#[cfg(test)]
mod tests {
    use crate::{is_nice, is_nice2};

    #[test]
    fn test_is_nice() {
        assert_eq!(true, is_nice("ugknbfddgicrmopn")); //  is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
        assert_eq!(true, is_nice("aaa")); // is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
        assert_eq!(false, is_nice("jchzalrnumimnmhp")); // is naughty because it has no double letter.
        assert_eq!(false, is_nice("haegwjzuvuyypxyu")); // is naughty because it contains the string xy.
        assert_eq!(false, is_nice("dvszwmarrgswjxmb")); // is naughty because it contains only one vowel.
    }

    #[test]
    fn test_is_nice2() {
        assert_eq!(true, is_nice2("qjhvhtzxzqqjkmpb"));
        assert_eq!(true, is_nice2("xxyxx"));
        assert_eq!(false, is_nice2("uurcxstgmygtbstg"));
        assert_eq!(false, is_nice2("ieodomkazucvgmuy"));
    }
}
