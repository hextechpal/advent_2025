use std::fs;

pub struct Bank<'a> {
    s: &'a str,
}

impl<'a> Bank<'a> {
    fn new(s: &'a str) -> Self {
        Self { s }
    }

    fn joltage(&self) -> i32 {
        let mut max = 0;
        let mut sec_max = 0;
        let mut idx = 0;

        let digits: Vec<u32> = self.s.chars().filter_map(|c| c.to_digit(10)).collect();

        for (i, b) in digits[0..digits.len()-1].iter().enumerate() {
            if max < *b {
                max = *b;
                idx = i;
            }
        }

        for b in digits[idx+1..].iter() {
            if sec_max < *b {
                sec_max = *b;
            }
        }

        println!("{} {max} {sec_max}", self.s);
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
