impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        // Create adjacency list representation of the graph
        let mut graph = vec![vec![]; n as usize + 1]; // +1 because nodes are 1-indexed
        for edge in &times {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
            graph[u].push((v, w));
        }

        // Initialize distance array with infinity
        let mut dist = vec![i32::MAX; n as usize + 1];
        dist[k as usize] = 0; // Distance to source is 0

        // Priority queue for Dijkstra's algorithm
        // Min-heap of (distance, node) pairs
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut pq = BinaryHeap::new();
        pq.push(Reverse((0, k as usize)));

        // Dijkstra's algorithm
        while let Some(Reverse((d, u))) = pq.pop() {
            if d > dist[u] {
                continue; // Skip if we've found a better path already
            }

            for &(v, w) in &graph[u] {
                if dist[u] != i32::MAX && dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                    pq.push(Reverse((dist[v], v)));
                }
            }
        }

        // Find the maximum distance (time) to any node
        let mut max_dist = 0;
        for i in 1..=n as usize {
            if dist[i] == i32::MAX {
                return -1; // Some node is unreachable
            }
            max_dist = max_dist.max(dist[i]);
        }

        max_dist
    }
}

struct Solution;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    assert_eq!(
        Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2),
        2
    );
}
