// The problem is: given an undirected graph with n vertices (n <= 1e5) and m edges (m <= n), find all critical edges.
// A critical edge is one whose removal would increase the number of connected components in the graph.
// In undirected graph, such edges are called bridges.

impl Solution {
    // A naive approach is to remove each edge one by one and check if the graph becomes disconnected. If it does, the edge is a bridge.
    // Time complexity: O(n * (n + m)) = O(n^2), which is not feasible.
    pub fn critical_connections_stupid(n: i32, cnts: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for c in &cnts {
            let (u, v) = (c[0], c[1]);
            adj[u as usize].push(v as usize);
            adj[v as usize].push(u as usize);
        }

        fn dfs(adj: &Vec<Vec<usize>>, u: usize, visited: &mut Vec<bool>) {
            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    dfs(adj, v, visited);
                }
            }
        }

        let mut res = vec![];
        for c in &cnts {
            let (u, v) = (c[0] as usize, c[1] as usize);
            adj[u].retain(|&e| e != v);
            adj[v].retain(|&e| e != u);
            let mut visited = vec![false; n];
            visited[u] = true;
            dfs(&adj, u, &mut visited);
            if !visited[v] {
                res.push(c.clone());
            }
            adj[u].push(v);
            adj[v].push(u);
        }
        res
    }

    // The better way to solve this problem is to use Tarjan algorithm
    // The idea is to have disc[] and low[]
    // disc[u] = discovery time of node u
    // low[u] = lowest discovery time of node u
    // For u in 0.. n:
    //  Mark u as visited
    //  Set disc[u] and low[u] to current time
    //  For every neightbor v:
    //      If v isn't visited:
    //          Visit v
    //          Update low[u] = min(low[u], low[v])
    //          If low[v] > disc[u]: The edge u-v is a bridge (no path exists from v back to u or earlier nodes)
    //      If v is visited, and not the parent of u:
    //          low[u] = min(low[u], disc[v])
    //
    // Time complexity: O(n + m) = O(n)
    pub fn critical_connections(n: i32, cnts: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        let n = n as usize;
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for c in cnts {
            let (u, v) = (c[0] as usize, c[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut disc = vec![0; n];
        let mut low = vec![0; n];
        let mut time = 1;

        for u in 0..n {
            if disc[u] == 0 {
                dfs(&adj, &mut disc, &mut low, &mut time, &mut res, u, usize::MAX);
            }
        }

        fn dfs(adj: &Vec<Vec<usize>>, disc: &mut Vec<usize>, low: &mut Vec<usize>, time: &mut usize, res: &mut Vec<Vec<usize>>, u: usize, parent: usize) {
            disc[u] = *time;
            low[u] = *time;
            *time += 1;

            for &v in adj[u].iter() {
                if disc[v] == 0 {
                    dfs(adj, disc, low, time, res, v, u);
                    low[u] = low[u].min(low[v]);
                    // If the lowest reachable vertex from v is after u's discovery, u-v is a bridge
                    if low[v] > disc[u] {
                        res.push(vec![u, v]);
                    }
                } else if v != parent {
                    low[u] = low[u].min(disc[v]);
                }
            }
        }

        res.iter().map(|e| vec![e[0] as i32, e[1] as i32]).collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::critical_connections_stupid(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]]), vec![vec![1, 3]]);

    assert_eq!(Solution::critical_connections_stupid(2, vec![vec![0, 1]]), vec![vec![0, 1]]);

    assert_eq!(Solution::critical_connections_stupid(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]]), Vec::<Vec<i32>>::new());

    assert_eq!(Solution::critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]]), vec![vec![1, 3]]);

    assert_eq!(Solution::critical_connections(2, vec![vec![0, 1]]), vec![vec![0, 1]]);

    assert_eq!(Solution::critical_connections(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]]), Vec::<Vec<i32>>::new());
}
