use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Debug)]
struct Item {
    init: i64,
    final_state: i64,
    toggles: Vec<i64>,
}

impl Item {
    fn min_presses(&self) -> i64 {
        let mut seen = HashSet::new();
        let mut q = VecDeque::new();
        let mut steps = 0;

        q.push_back(self.init);
        seen.insert(self.init);

        while !q.is_empty() {
            let curr_size = q.len();
            for _ in 0..curr_size {
                let el = q.pop_front().unwrap();
                if el == self.final_state {
                    return steps;
                }

                for toggle in &self.toggles {
                    let next = el ^ toggle;
                    if !seen.contains(&next) {
                        q.push_back(next);
                        seen.insert(next);
                        // println!("next={next}");
                    };
                }
            }
            // println!("curr_size={curr_size}, q={q:?}, seen={seen:?}");
            steps += 1;
        }

        -1
    }
}

fn main() {
    let contents = fs::read_to_string("input/day10/input.txt").expect("unable to read input file");
    let items: Vec<Item> = contents.lines().map(parse_line).collect();
    // println!("item={:?}, presses={}", items[0], items[0].min_presses());

    let mut total = 0;
    for item in &items {
        let p = item.min_presses();
        println!("presses={}", p);
        total += p;
    }

    println!("total={}", total)
}

fn parse_line(s: &str) -> Item {
    let (final_state, width) = parse_final_state(s);

    let mut toggles = Vec::new();
    let mut search_start = 0;
    while let Some(open_rel) = s[search_start..].find('(') {
        let open = search_start + open_rel;
        let after_open = open + 1;
        if let Some(close_rel) = s[after_open..].find(')') {
            let close = after_open + close_rel;
            let mask = parse_toggle_group(&s[after_open..close], width);
            toggles.push(mask);
            search_start = close + 1;
        } else {
            break;
        }
    }

    Item {
        init: 0,
        final_state,
        toggles,
    }
}

fn parse_final_state(line: &str) -> (i64, usize) {
    let start = line
        .find('[')
        .expect("line must contain '[' for final state");
    let end = line[start + 1..]
        .find(']')
        .map(|idx| start + 1 + idx)
        .expect("line must contain closing ']' for final state");

    let pattern = &line[start + 1..end];
    let width = pattern.chars().count();

    let state = pattern.chars().fold(0_i64, |acc, c| {
        let bit = match c {
            '#' => 1,
            '.' => 0,
            other => panic!("unexpected character '{other}' in final state"),
        };
        (acc << 1) | bit
    });

    (state, width)
}

fn parse_toggle_group(group: &str, width: usize) -> i64 {
    let mut mask = 0_i64;

    for entry in group.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        let idx: usize = entry
            .parse()
            .unwrap_or_else(|_| panic!("unable to parse toggle index '{entry}'"));
        assert!(
            idx < width,
            "toggle index {} is out of range for width {}",
            idx,
            width
        );

        let bit_pos = width - 1 - idx;
        mask |= 1_i64 << bit_pos;
    }

    mask
}
