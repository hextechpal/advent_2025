use std::fs;
use std::hash::Hash;

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn squared_distance(&self, other: &Self) -> i128 {
        let dx = (self.x - other.x) as i128;
        let dy = (self.y - other.y) as i128;
        let dz = (self.z - other.z) as i128;
        dx * dx + dy * dy + dz * dz
    }
}

impl<T: ToString> From<T> for Point {
    fn from(value: T) -> Self {
        let s = value.to_string();
        let split = s.split(',').collect::<Vec<_>>();

        if split.len() != 3 {
            println!("len={}, str={}", split.len(), s);
            panic!("malformed string");
        }

        let x = split[0].parse::<i64>().unwrap();
        let y = split[1].parse::<i64>().unwrap();
        let z = split[2].parse::<i64>().unwrap();

        Self { x, y, z }
    }
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
struct Pair {
    i: usize,
    j: usize,
    dist2: i128, // squared distance
}

impl Pair {
    fn new(i: usize, j: usize, p1: &Point, p2: &Point) -> Self {
        Self {
            i,
            j,
            dist2: p1.squared_distance(p2),
        }
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist2.cmp(&other.dist2)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return false;
        }

        // union by size
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.count -= 1;
        true
    }
}

fn main() {
    let contents = fs::read_to_string("input/day8/input.txt").expect("unable to read input file");
    let points: Vec<Point> = contents.lines().map(Point::from).collect();
    let n = points.len();

    // Build all pairs
    let mut pairs = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            pairs.push(Pair::new(i, j, &points[i], &points[j]));
        }
    }

    let mut ds = DisjointSet::new(n);
    pairs.sort();

    let mut ec = 0;
    for i in 0..pairs.len() {
        if ec == n - 1 {
            let last = pairs[i-1];
            let p1 = points[last.i];
            let p2 = points[last.j];
            println!("{}", p1.x * p2.x);
            break;
        }
        let pair = pairs[i];
        if ds.union(pair.i, pair.j) {
            ec += 1;
        }
    }
}
