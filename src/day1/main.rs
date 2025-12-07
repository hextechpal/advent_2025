use std::fs;

enum Direction {
    Right,
    Left,
}

pub struct Turn {
    dir: Direction,
    ticks: i32,
}

impl Turn {
    fn new(dir: Direction, ticks: i32) -> Self {
        Self { dir, ticks }
    }
}

impl<T: ToString> From<T> for Turn {
    fn from(value: T) -> Self {
        let binding = value.to_string();
        let mut chars = binding.chars();
        match chars.next() {
            Some(c) => match c {
                'L' => {
                    let ticks = i32::from_str_radix(chars.as_str(), 10).expect("malformed string");
                    Self::new(Direction::Left, ticks)
                }
                'R' => {
                    let ticks = i32::from_str_radix(chars.as_str(), 10).expect("malformed string");
                    Self::new(Direction::Right, ticks)
                }
                _ => panic!("malformed string"),
            },

            None => panic!("malformed string"),
        }
    }
}

struct Dial {
    pos: i32,
    zeroes: i32,
}

impl Dial {
    pub fn new() -> Self {
        Self { pos: 50, zeroes: 0 }
    }

    pub fn turn(&mut self, turn: Turn) {
        self.zeroes += turn.ticks / 100;
        let ticks = turn.ticks % 100;
        match turn.dir {
            Direction::Right => {
                self.pos += ticks;
                if self.pos > 100 {
                    self.zeroes += 1
                }
            }

            Direction::Left => {
                let prev = self.pos;
                self.pos -= ticks;
                if self.pos < 0 {
                    self.pos += 100;
                    if prev != 0 {
                        self.zeroes += 1
                    }
                }
            }
        }

        self.pos %= 100;
        if self.pos == 0 {
            self.zeroes += 1;
        }
    }

    pub fn zeroes(&self) -> i32 {
        self.zeroes
    }

    pub fn pos(&self) -> i32 {
        self.pos
    }
}

fn main() {
    let mut dial = Dial::new();
    let contents = fs::read_to_string("input/day1/input.txt").expect("unable to read input file");
    for line in contents.lines() {
        let m = Turn::from(line);
        dial.turn(m);
        println!("pos: {}, zeroes: {}", dial.pos(), dial.zeroes());
    }
    // println!("{}", dial.zeroes());
}
