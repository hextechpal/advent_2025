use std::{cmp::max, cmp::min, collections::HashSet};

use crate::point::Point;

pub struct MatrixReduction {
    reds: Vec<Point>,
}

#[derive(Debug)]
struct Boundary {
    outer: HashSet<Point>,
}

impl Boundary {
    pub fn is_valid(&self, p0: Point, p1: Point) -> bool {
        let min_x = min(p0.x, p1.x);
        let max_x = max(p0.x, p1.x);
        let min_y = min(p0.y, p1.y);
        let max_y = max(p0.y, p1.y);

        for c in min_y..max_y + 1 {
            if self.outer.contains(&Point::new(min_x, c))
                || self.outer.contains(&Point::new(max_x, c))
            {
                return false;
            }
        }

        for c in min_x..max_x + 1 {
            if self.outer.contains(&Point::new(c, min_y))
                || self.outer.contains(&Point::new(c, max_y))
            {
                return false;
            }
        }

        true
    }
}

impl MatrixReduction {
    pub fn new<T>(reds: T) -> Self
    where
        T: IntoIterator<Item = Point>,
    {
        let reds = Vec::from_iter(reds);
        Self { reds }
    }

    fn sign(a: i64) -> i64 {
        if a == 0 {
            return 0;
        } else if a > 0 {
            return 1;
        } else {
            -1
        }
    }

    pub fn max_area(&self) -> i64 {
        let b = self.trace_boundary();
        let n = self.reds.len();
        // bounding box around boundary tiles
        println!("total reds = {}", n);
        let mut count = 0;
        let mut max = -1;
        for i in 0..n {
            for j in i + 1..n {
                let p0 = self.reds[i];
                let p1 = self.reds[j];
                count += 1;
                println!("Checking rectangle: {}", count);
                if b.is_valid(p0, p1) {
                    max = max.max(((p0.x - p1.x).abs() + 1) * ((p0.y - p1.y).abs() + 1));
                }
            }
        }
        max
    }

    fn trace_boundary(&self) -> Boundary {
        let mut boundary = HashSet::new();
        let mut outer = HashSet::new();
        let n = self.reds.len();
        for i in 0..n {
            let p0 = self.reds[i];
            let p1 = self.reds[(i + 1) % n];

            let x1 = p0.x;
            let y1 = p0.y;
            let x2 = p1.x;
            let y2 = p1.y;

            let dx = Self::sign(x2 - x1);
            let dy = Self::sign(y2 - y1);

            let offset = match (dx, dy) {
                (0, -1) => (-1, 0),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (-1, 0) => (0, 1),
                _ => panic!("should not happen"),
            };

            boundary.insert(p0);
            outer.insert(Point::new(p0.x + offset.0, p0.y + offset.1));
            let mut nx = x1 + dx;
            let mut ny = y1 + dy;
            while nx != p1.x && ny != p1.y {
                boundary.insert(Point::new(nx, ny));
                outer.insert(Point::new(nx + offset.0, ny + offset.1));
                nx += dx;
                ny += dy;
            }
            boundary.insert(Point::new(nx, ny));
            outer.insert(Point::new(nx + offset.0, ny + offset.1));
        }

        let outer = outer
            .iter()
            .filter(|&p| !boundary.contains(p))
            .map(|p| *p)
            .collect();

        println!("{:?}\n\n", boundary);
        println!("{:?}\n\n", outer);
        Boundary { outer }
    }
}
