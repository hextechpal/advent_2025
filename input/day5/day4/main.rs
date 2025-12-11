use std::fs;

const EMPTY: char = '.';

fn main() {
    let mut grid = Vec::new();
    let contents = fs::read_to_string("input/day4/input.txt").expect("unable to read input file");
    for line in contents.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
    }

    println!("{}", accesible(grid));
}

fn accesible(mut grid: Vec<Vec<char>>) -> i32 {
    let mut acc = 0;
    let dirs = vec![
        (1, 0),
        (1, 1),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, -1),
        (-1, 0),
    ];

    let m = grid.len() as i32;
    let n = grid[0].len() as i32;

    let mut next = grid.clone();

    loop {
        let mut ans = 0;

        for (r, s) in grid.iter().enumerate() {
            for (c, ch) in s.iter().enumerate() {
                if *ch == EMPTY {
                    next[r][c] = *ch;
                    continue;
                }

                let mut adj = 0;
                for &(x, y) in &dirs {
                    let nr = r as i32 + x;
                    let nc = c as i32 + y;

                    if nr < 0
                        || nr >= m
                        || nc < 0
                        || nc >= n
                        || grid[nr as usize][nc as usize] == EMPTY
                    {
                        continue;
                    }
                    adj += 1;
                }

                if adj < 4 {
                    next[r][c] = EMPTY;
                    ans += 1
                } else {
                    next[r][c] = grid[r][c];
                }
            }
        }

        if ans == 0 {
            break;
        }

        acc += ans;
        let tmp = grid;
        grid = next;
        next = tmp;
    }

    acc
}
