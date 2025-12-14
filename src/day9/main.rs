use convex_hull::ConvexHull;
use point::Point;
use std::fs;

use crate::matrix_reduction::MatrixReduction;

mod convex_hull;
mod matrix_reduction;
mod point;

fn main() {
    let contents =
        fs::read_to_string("input/day9/input_test.txt").expect("unable to read input file");
    let points: Vec<Point> = contents.lines().map(Point::from).collect();

    // let hull = ConvexHull::hull(&mut points);

    // println!("points_len={:?}, hull_len={:?}", points.len(), hull.len());
    // let mut area = 0;
    // for p1 in &hull {
    //     for p2 in &hull {
    //         let curr = ((p1.x - p2.x).abs() + 1) * ((p1.y - p2.y).abs() + 1);
    //         if curr > area {
    //             area = curr;
    //         }
    //     }
    // }

    let mr = MatrixReduction::new(points);
    let area = mr.max_area();

    println!("{area}");
}
