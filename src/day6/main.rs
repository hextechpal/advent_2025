use std::fs;

fn main() {
    let contents = fs::read_to_string("input/day6/input.txt").expect("unable to read input file");

    let mut it = contents.lines().peekable();

    let mut row_iters = Vec::new();
    let mut ops = Vec::new();
    while let Some(line) = it.next() {
        if it.peek().is_none() {
            ops = line
                .split(' ')
                .filter(|&el| !el.is_empty())
                .collect::<Vec<_>>();
        } else {
            row_iters.push(line.split(' ').filter(|&el| !el.is_empty()));
        }
    }

    let grid = (0..ops.len())
        .map(|_| {
            row_iters
                .iter_mut()
                .filter_map(|it| it.next())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for (i, &op) in ops.iter().enumerate() {
        let nums = grid[i]
            .iter()
            .map(|&el| el.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        match op {
            "+" => ans += nums.iter().fold(0, |acc, &el| acc + el),
            "*" => ans += nums.iter().fold(1, |acc, &el| acc * el),
            _ => panic!("invalid operation"),
        }
    }

    println!("{ans}");
}
