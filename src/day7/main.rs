use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day7/input.txt").expect("unable to read input file");

    let mut grid = Vec::new();
    for line in contents.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
    }

    println!("{}", traverse(grid));
}

fn traverse(grid: Vec<Vec<char>>) -> i64 {
    let s_pos = grid[0].iter().position(|&c| c == 'S').unwrap();
    let cols: usize = grid[0].len();

    let mut paths = vec![0i64; cols];
    paths[s_pos] = 1;

    for row in grid.iter().skip(1) {
        let mut next = vec![0i64; cols];

        for (col, &cnt) in paths.iter().enumerate() {
            if cnt == 0 {
                continue;
            }

            if row[col] == '^' {
                if col > 0 {
                    next[col - 1] += cnt
                }

                if col + 1 < cols {
                    next[col + 1] += cnt
                }
            } else {
                next[col] += cnt;
            }
        }

        paths = next;
    }

    paths.iter().sum()
}
