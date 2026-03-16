use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn prims_mst_sum(adj: &Vec<Vec<(usize, i32)>>, n: usize) -> i32 {
    let mut total_weight = 0;
    let mut visited = vec![false; n];
    let mut min_heap = BinaryHeap::new();

    // Push (Reverse(weight), node)
    min_heap.push((Reverse(0), 0));

    while let Some((Reverse(weight), u)) = min_heap.pop() {
        if visited[u] {
            continue;
        }

        visited[u] = true;
        total_weight += weight;

        for &(v, w) in &adj[u] {
            if !visited[v] {
                min_heap.push((Reverse(w), v));
            }
        }
    }

    total_weight
}

fn main() {
    let n = 5;
    let mut adj = vec![vec![]; n];

    let mut add_edge = |u: usize, v: usize, w: i32| {
        adj[u].push((v, w));
        adj[v].push((u, w));
    };

    add_edge(0, 1, 2);
    add_edge(0, 3, 6);
    add_edge(1, 2, 3);
    add_edge(1, 3, 8);
    add_edge(1, 4, 5);
    add_edge(2, 4, 7);
    add_edge(3, 4, 9);

    let sum = prims_mst_sum(&adj, n);
    println!("Total MST Weight: {}", sum);
}