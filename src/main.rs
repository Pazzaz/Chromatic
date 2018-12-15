use petgraph::Graph;
use petgraph::prelude::*;
use petgraph::dot::{Dot, Config};

use hashbrown::HashMap;

fn main() {
    let mut test = Graph::<_, (), Undirected>::new_undirected();
    for i in 0..30 {
        test.add_node(i);
        for n in 0..i {
            if n != 0 && i % n == 0 {
                test.add_edge(NodeIndex::new(n), NodeIndex::new(i), ());
            }
        }
    }

    println!("{:?}", Dot::with_config(&test, &[Config::EdgeNoLabel]));

    let v = test.node_count();
    assert!(v < 64);
    let mut neighbours = Vec::new();
    neighbours.resize(v, 0);
    for edge in test.raw_edges() {
        let a = edge.source().index();
        let b = edge.target().index();
        neighbours[a] |= 1 << b;
        neighbours[b] |= 1 << a;
    }
    let mut all: u64 = 0;
    for _ in 0..v {
        all <<= 1;
        all += 1;
    }
    let mut seen_s = HashMap::new();
    seen_s.insert(all, 0);
    let mut collected: HashMap<u64, i32> = HashMap::new();
    for i in (0..all).rev() {
        let result = s(i, &mut seen_s, &neighbours);
        let value = collected.entry(result).or_insert(0);
        if i.count_ones() % 2 == 0 {
            *value += 1
        } else {
            *value -= 1;
        }
    }
    // println!("{:?}", collected);
    let mut total = 0;
    for (k, v) in collected {
        total += (v as i64) * (k.pow(4) as i64);
    }
    println!("{}", total);


}

fn s(set: u64, seen: &mut HashMap<u64, u64>, neighbours: &Vec<u64>) -> u64 {
    if let Some(answer) = seen.get(&set) {
        return *answer;
    }
    let extra = (!set).trailing_zeros();
    let v = 1 << extra;
    let n_v = neighbours.get(extra as usize).expect("Missing neighbours for node");
    let a = s(set | v, seen, neighbours) + s(set | v | n_v, seen, neighbours) + 1;
    seen.insert(set, a);
    a
}