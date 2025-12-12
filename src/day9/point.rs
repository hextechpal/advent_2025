#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl<T: ToString> From<T> for Point {
    fn from(value: T) -> Self {
        let s = value.to_string();
        let split = s.split(',').collect::<Vec<_>>();

        if split.len() != 2 {
            println!("len={}, str={}", split.len(), s);
            panic!("malformed string");
        }

        let x = split[0].parse::<i64>().unwrap();
        let y = split[1].parse::<i64>().unwrap();

        Self { x, y }
    }
}
