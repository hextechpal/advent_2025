use std::fs;

pub struct Bank {
    digits: Vec<u32>,
    activate: usize,
}

impl Bank {
    fn new(s: &str, activate: usize) -> Self {
        Self {
            digits: s.chars().filter_map(|c| c.to_digit(10)).collect(),
            activate,
        }
    }

    // 2 3 4 2 3 4 2 3 4 2 3  4  2  7  8     --> 15
    // 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
    fn joltage(&self) -> i64 {
        let mut v = Vec::with_capacity(self.activate);
        let mut idx = 0;
        let mut rem = self.activate;
        for i in 0..12 {
            if i != 0 {
                idx += 1;
            }

            if idx + rem >= self.digits.len() {
                break;
            }

            // idx + offset + rem - 1 = len
            // offset = len - idx - rem + 1
            let (max_, idx_) = self.find_max(idx, self.digits.len() - rem + 1);
            idx += idx_;
            v.push(max_);
            rem -= 1;
            // println!("idx={idx}, start={idx}, i={i}, max={max_}");
        }


        for j in 0..rem {
            v.push(self.digits[idx+j]);
        }

        // println!("{:?}", v);
        v.iter().fold(0, |acc, &d| acc * 10 + d as i64)
    }

    pub fn find_max(&self, start: usize, end: usize) -> (u32, usize) {
        let mut max = 0;
        let mut idx = 0;
        for (i, d) in self.digits[start..end].iter().enumerate() {
            if max < *d {
                max = *d;
                idx = i;
            }
        }

        (max, idx)
    }
}

fn main() {
    let contents =
        fs::read_to_string("input/day3/input.txt").expect("unable to read input file");
    let mut j = 0;
    for line in contents.lines() {
        let b = Bank::new(line, 12);
        let joltage = b.joltage();
        println!("{joltage}");
        j += joltage;
    }

    println!("{j}");
}
