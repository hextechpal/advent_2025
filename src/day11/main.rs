use std::{
    collections::HashMap,
    fs,
};

#[derive(Copy, Clone)]
struct NodeFlags {
    has_fft: bool,
    has_dac: bool,
}

struct Graph {
    adj: Vec<Vec<usize>>,     // adjacency list (node ids)
    flags: Vec<NodeFlags>,    // per-node metadata
}

impl Graph {
    /// Count paths from `from` to `to` that pass through both fft and dac.
    /// Assumes the graph is a DAG.
    fn path_count(&self, from: usize, to: usize) -> i64 {
        let mut memo = vec![[ -1; 4 ]; self.adj.len()];
        self.dfs(from, to, 0, &mut memo)
    }

    fn dfs(
        &self,
        node: usize,
        target: usize,
        mask: u8,
        memo: &mut Vec<[i64; 4]>,
    ) -> i64 {
        if node == target {
            return if mask == 0b11 { 1 } else { 0 };
        }

        let cached = memo[node][mask as usize];
        if cached != -1 {
            return cached;
        }

        let mut count = 0;

        for &next in &self.adj[node] {
            let mut next_mask = mask;
            if self.flags[next].has_fft {
                next_mask |= 0b01;
            }
            if self.flags[next].has_dac {
                next_mask |= 0b10;
            }

            count += self.dfs(next, target, next_mask, memo);
        }

        memo[node][mask as usize] = count;
        count
    }
}

fn main() {
    let contents =
        fs::read_to_string("input/day11/input.txt").expect("unable to read input file");

    // ---------- Phase 1: parse edges as strings ----------
    let mut raw_adj: HashMap<String, Vec<String>> = HashMap::new();

    for line in contents.lines() {
        let colon_idx = line.find(':').unwrap();
        let key = line[0..colon_idx].to_string();
        let vals = line[colon_idx + 2..]
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        raw_adj.insert(key, vals);
    }

    // ---------- Phase 2: assign node IDs ----------
    let mut node_id: HashMap<String, usize> = HashMap::new();
    let mut next_id = 0;

    for (u, neighbors) in &raw_adj {
        if !node_id.contains_key(u) {
            node_id.insert(u.clone(), next_id);
            next_id += 1;
        }
        for v in neighbors {
            if !node_id.contains_key(v) {
                node_id.insert(v.clone(), next_id);
                next_id += 1;
            }
        }
    }

    let n = node_id.len();

    // ---------- Phase 3: build indexed graph ----------
    let mut adj = vec![Vec::new(); n];
    let mut flags = vec![
        NodeFlags {
            has_fft: false,
            has_dac: false
        };
        n
    ];

    for (name, &id) in &node_id {
        if name == "fft" {
            flags[id].has_fft = true;
        }
        if name == "dac" {
            flags[id].has_dac = true;
        }
    }

    for (u, neighbors) in raw_adj {
        let u_id = node_id[&u];
        for v in neighbors {
            let v_id = node_id[&v];
            adj[u_id].push(v_id);
        }
    }

    let graph = Graph { adj, flags };

    // ---------- Phase 4: query ----------
    let from = node_id["svr"];
    let to = node_id["out"];

    let count = graph.path_count(from, to);
    println!("path_count={}", count);
}
