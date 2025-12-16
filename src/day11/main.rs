use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub struct Graph<'a> {
    adj: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Graph<'a> {
    fn path_count(&self, from: &'a str, to: &'a str) -> i32 {
        let mut count = 0;

        let mut stack = Vec::new();
        let mut visited = HashSet::new();
        visited.insert(from);
        stack.push((from, visited));

        while let Some((start, visited)) = stack.pop() {
            if start == to {
                count += 1;
                continue;
            }

            if let Some(neighbors) = self.adj.get(&start) {
                for &neighbor in neighbors {
                    if visited.contains(neighbor) {
                        panic!("cycle detected");
                    }

                    let mut new_visited = visited.clone();
                    new_visited.insert(neighbor);
                    stack.push((neighbor, new_visited));
                }
            }
        }

        count
    }
}

fn main() {
    let contents = fs::read_to_string("input/day11/input.txt").expect("unable to read input file");

    let mut adj = HashMap::new();
    for line in contents.lines() {
        let colon_idx = line.find(':').unwrap();
        let key = &line[0..colon_idx];
        // skip 1 for space
        let vals = line[colon_idx + 2..].split(' ').collect::<Vec<_>>();

        adj.entry(key).or_insert(vals);
    }

    let g = Graph { adj };

    println!("path_count={}", g.path_count("you", "out"));
}
