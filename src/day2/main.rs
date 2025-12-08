use core::str;
use std::fs;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    pub fn invalid_sum(&self) -> i64 {
        let mut ans = 0;
        for i in self.start..=self.end {
            let s = i.to_string();
            for j in 1..s.len() / 2 + 1 {
                if s.len() % j == 0 && Self::check_repeated(&s[0..j], &s) {
                    println!("{i}, {self:?}");
                    ans += i;
                    break;
                }
            }
        }
        ans
    }

    fn check_repeated(p: &str, s: &str) -> bool {
        let reps = s.len() / p.len();
        p.repeat(reps) == s
    }
}

impl<T: ToString> From<T> for Range {
    fn from(value: T) -> Self {
        let target = value.to_string();
        let split: Vec<&str> = target.split('-').collect();

        if split.len() != 2 {
            panic!("invalid input");
        }

        Range {
            start: split[0].parse::<i64>().expect("invalid format string"),
            end: split[1].parse::<i64>().expect("invalid format string"),
        }
    }
}

fn main() {
    let contents =
        fs::read_to_string("input/day2/input.txt").expect("unable to read input file");
    let mut sum = 0;
    for line in contents.split(',') {
        let r = Range::from(line);
        sum += r.invalid_sum();
    }
    println!("{}", sum);
}
