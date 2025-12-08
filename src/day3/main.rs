use std::fs;

pub struct Bank {
    digits: Vec<u32>,
}

impl Bank {
    fn new(s: &str) -> Self {
        Self {
            digits: s.chars().filter_map(|c| c.to_digit(10)).collect(),
        }
    }

    fn joltage(&self) -> i32 {
        let mut max = 0;
        let mut sec_max = 0;
        let mut idx = 0;

        for (i, b) in self.digits[0..self.digits.len() - 1].iter().enumerate() {
            if max < *b {
                max = *b;
                idx = i;
            }
        }

        for b in self.digits[idx + 1..].iter() {
            if sec_max < *b {
                sec_max = *b;
            }
        }

        // println!("{max} {sec_max}");
        (10 * max + sec_max).try_into().unwrap()
    }
}

fn main() {
    let contents = fs::read_to_string("input/day3/input.txt").expect("unable to read input file");
    let mut j = 0;
    for line in contents.lines() {
        let b = Bank::new(line);
        j += b.joltage();
    }

    println!("{j}");
}
