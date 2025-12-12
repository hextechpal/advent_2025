use std::{cmp::Ordering, collections::VecDeque};

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

pub enum Turn {
    CounterClockwise,
    Clockwise,
    Collinear,
}

pub struct ConvexHull;

impl ConvexHull {
    // performs p x q
    fn ccw(p: &Point, q: &Point, re: &Point) -> Turn {
        let v1 = ((p.x - re.x), (p.y - re.y));
        let v2 = ((q.x - re.x), (q.y - re.y));

        let area = (v1.0 * v2.1) - (v1.1 * v2.0);

        match area {
            ..0 => Turn::Clockwise,
            1.. => Turn::CounterClockwise,
            0 => Turn::Collinear,
        }
    }

    fn is_clockwise(p: &Point, q: &Point, re: &Point) -> bool {
        match Self::ccw(p, q, re) {
            Turn::Clockwise => true,
            _ => false,
        }
    }

    pub fn hull(points: &mut Vec<Point>) -> Vec<Point> {
        let mut stack = VecDeque::new();
        let min_y = points
            .iter()
            .min_by(|&p, &q| p.y.cmp(&q.y))
            .unwrap()
            .clone();

        // println!("min_y={:?}", min_y);

        points.sort_by(|p, q| match Self::ccw(p, q, &min_y) {
            Turn::CounterClockwise => Ordering::Less,
            Turn::Clockwise => Ordering::Greater,
            Turn::Collinear => {
                if p.x == q.x {
                    Ordering::Less
                } else {
                    p.x.cmp(&q.x)
                }
            }
        });

        // println!("sorted_points={:?}", points);

        stack.push_front(points[0]);
        stack.push_front(points[1]);

        for next in points.iter().skip(2) {
            // println!("[start] stack={:?}", stack);
            let mut p = stack.pop_front().unwrap();
            // println!("[pop] stack={:?}", stack);
            while !stack.is_empty() && !Self::is_clockwise(&next, &p, stack.front().unwrap()) {
                // println!("poppping_fromnt");
                p = stack.pop_front().unwrap();
            }
            stack.push_front(p);
            stack.push_front(*next);
            // println!("[end] stack={:?}", stack);
        }

        let p = stack.pop_front().unwrap();
        if Self::is_clockwise(&min_y, &p, stack.front().unwrap()) {
            stack.push_front(p);
        }

        let mut hull = Vec::new();
        while !stack.is_empty() {
            hull.push(stack.pop_front().unwrap());
        }

        hull
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_ccw() {
        let origin = Point { x: 0, y: 0 };
        let a = Point { x: 0, y: 1 };
        let b = Point { x: 1, y: 0 };

        match ConvexHull::ccw(&a, &b, &origin) {
            Turn::Clockwise => assert!(true),
            _ => panic!("should be clockwise"),
        }

        match ConvexHull::ccw(&b, &a, &origin) {
            Turn::CounterClockwise => assert!(true),
            _ => panic!("should be clockwise"),
        }
    }

    #[test]
    fn test_sort() {
        let origin = Point { x: 0, y: 0 };
        let a = Point { x: 0, y: 1 };
        let b = Point { x: 1, y: 1 };
        let c = Point { x: 0, y: 1 };
        let d: Point = Point { x: -1, y: 1 };

        let mut points = vec![a, b, c, d];

        points.sort_by(|p, q| match ConvexHull::ccw(p, q, &origin) {
            Turn::CounterClockwise => Ordering::Less,
            Turn::Clockwise => Ordering::Greater,
            Turn::Collinear => {
                if p.x == q.x {
                    Ordering::Less
                } else {
                    p.x.cmp(&q.x)
                }
            }
        });

        println!("{:?}", points);
    }
}
