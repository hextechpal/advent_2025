use std::cmp::Ordering;
use std::fs;

fn main() {
    let contents =
        fs::read_to_string("input/day5/input.txt").expect("unable to read input file");
    let mut test_input = false;
    let mut ans = 0;
    let mut intervals = Vec::new();
    for line in contents.lines() {
        if line.is_empty() {
            test_input = true;
            intervals = merge(intervals.clone());
            println!("{:?}", intervals);

            

            continue;
        }

        if test_input {
            let target = line.parse::<i64>().unwrap();
            if find(&intervals, target) {
                println!("found={target}");
                ans += 1;
            }else{
                println!("not_found={target}");    
            }
        } else {
            let split = line.split('-').collect::<Vec<_>>();
            if split.len() != 2 {
                panic!("malformed input");
            }
            let left = split[0].parse::<i64>().unwrap();
            let right = split[1].parse::<i64>().unwrap();
            intervals.push((left, right));
        }
    }
    println!("{ans}");
}

/*
    |-----|
        |-----------|
                        |------|
                |-----------------|
                |------------|

*/
fn merge(intervals: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut ans = Vec::new();
    let mut intervals = intervals;
    // sort intervals by x axis
    intervals.sort_by(|&(a1, b1), &(a2, b2)| match a1.cmp(&a2) {
        Ordering::Equal => b2.cmp(&b1),
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
    });

    println!("{:?}", intervals);
    let mut merged = intervals[0];

    for &(x, y) in intervals.iter().skip(1) {
        // Disjoint intervals
        if merged.1 < x {
            ans.push(merged);
            merged = (x, y);
        } else {
            // Overlap
            merged.1 = merged.1.max(y);
        }
    }

    ans.push(merged);
    ans
}

fn find(intervals: &Vec<(i64, i64)>, target: i64) -> bool {
    let mut l = 0 as i32;
    let mut r = (intervals.len() - 1) as i32;

    loop {
        if l > r {
            break;
        }
        let m = (l + (r - l) / 2) as usize;
        if intervals[m].0 <= target && intervals[m].1 >= target {
            return true;
        } else if intervals[m].1 < target {
            l = m as i32 + 1;
        } else {
            r = m as i32 - 1;
        }
    }
    false
}
