use std::fs;

enum Movement {
    Right(i32),
    Left(i32),
}

impl<T: ToString> From<T> for Movement {
    fn from(value: T) -> Self {
        let binding = value.to_string();
        let mut chars = binding.chars();
        match chars.next() {
            Some(c) => match c {
                'L' => {
                    let ticks = i32::from_str_radix(chars.as_str(), 10).expect("malformed string");
                    Movement::Left(ticks % 100)
                }
                'R' => {
                    let ticks = i32::from_str_radix(chars.as_str(), 10).expect("malformed string");
                    Movement::Right(ticks % 100)
                }
                _ => panic!("malformed string"),
            },

            None => panic!("malformed string"),
        }
    }
}

struct Dial {
    pos: i32,
    zeroes: usize,
}

impl Dial {
    pub fn new() -> Self {
        Self { pos: 50, zeroes: 0 }
    }

    pub fn turn(&mut self, movement: Movement) {
        match movement {
            Movement::Right(ticks) => {
                self.pos += ticks;
                if self.pos < 0 {
                    self.pos += 100;
                }
                self.pos %= 100;
                if self.pos == 0 {
                    self.zeroes += 1;
                }
            }

            Movement::Left(ticks) => {
                self.pos -= ticks;
                if self.pos < 0 {
                    self.pos += 100;
                }
                self.pos %= 100;
                if self.pos == 0 {
                    self.zeroes += 1;
                }
            }
        }
    }

    pub fn zeroes(&self) -> usize {
        self.zeroes
    }
}

fn main() {
    let mut dial = Dial::new();
    let contents = fs::read_to_string("input/input.txt").expect("unable to read inout file");
    for line in contents.lines() {
        let m = Movement::from(line);
        dial.turn(m);
    }
    println!("{}", dial.zeroes());
}
