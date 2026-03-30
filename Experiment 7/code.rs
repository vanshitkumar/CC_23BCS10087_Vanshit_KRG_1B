use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (n, src, dst, k) = (n as usize, src as usize, dst as usize, k + 1);
        let mut graph: Vec<Vec<(usize, i32)>> = vec![vec!(); n];
        for flight in flights {
            graph[flight[0] as usize].push((flight[1] as usize, flight[2]));
        }

        let mut min_price = vec![i32::MAX; n];
        let mut min_steps = vec![i32::MAX; n];
        let mut next_to_visit = BinaryHeap::new();
        next_to_visit.push(Reverse((0, 0, src)));

        while !next_to_visit.is_empty() {
            let (price, steps, node) = next_to_visit.pop().unwrap().0;
            if price >= min_price[node] && steps >= min_steps[node] { continue; }
            if node == dst { return price; }
            if steps == k { continue; }

            min_price[node] = std::cmp::min(min_price[node], price);
            min_steps[node] = std::cmp::min(min_steps[node], steps);

            for (next_node, flight_price) in &graph[node] {
                next_to_visit.push(Reverse((price + flight_price, steps + 1, *next_node)));
            }
        }

        -1
    }
}