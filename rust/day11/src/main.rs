use anyhow::{anyhow, bail};
use core::str::FromStr;
use itertools::Itertools;
use std::fmt;

fn main() -> anyhow::Result<()> {
    let input = std::env::args()
        .nth(1)
        .expect("You must give an input password");

    let mut password = Password::from_str(&input)?;

    println!("Input: {}", password);
    password.next();
    println!("Next:  {}", password);
    password.next();
    println!("Next:  {}", password);

    Ok(())
}

#[derive(Debug)]
struct Password {
    data: [u8; 8],
}

impl Password {
    fn is_valid(&self) -> bool {
        // has straight
        let mut has_straight = false;
        for (a, b, c) in self.data.iter().tuple_windows::<(_, _, _)>() {
            if (*a + 2) == (*b + 1) && (*b + 1) == *c {
                has_straight = true;
                break;
            }
        }

        if !has_straight {
            return false;
        }

        // Cannot contain i, o, l
        for n in self.data {
            if n == ('i' as u8 - 'a' as u8)
                || n == ('o' as u8 - 'a' as u8)
                || n == ('l' as u8 - 'a' as u8)
            {
                return false;
            }
        }

        // Must contain two different pair's
        let mut first_pair = None;
        for (a, b) in self.data.iter().tuple_windows() {
            if a == b && first_pair.is_none() {
                first_pair = Some(a);
            }

            if let Some(fp) = first_pair {
                if fp != a && a == b {
                    return true;
                }
            }
        }

        false
    }

    fn next(&mut self) {
        self.increment();
        while !self.is_valid() {
            self.increment();
        }
    }

    fn increment(&mut self) {
        for i in (0..8).rev() {
            match self.data[i] {
                25 => self.data[i] = 0,
                _ => {
                    self.data[i] += 1;
                    break;
                }
            }
        }
    }
}

impl FromStr for Password {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = [0u8; 8];
        if s.len() != 8 {
            bail!("input value must be 8 characters");
        }
        for (i, c) in s.chars().enumerate() {
            if !c.is_alphabetic() {
                bail!("all characters must be alphabetic");
            }
            out[i] = (c.to_ascii_lowercase() as u8) - ('a' as u8);
        }

        Ok(Password { data: out })
    }
}

impl fmt::Display for Password {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buff = String::new();
        for n in self.data {
            buff.push((n + 'a' as u8) as char);
        }

        write!(formatter, "{}", buff)
    }
}
