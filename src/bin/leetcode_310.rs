use std::collections::VecDeque;

impl Solution {
    pub fn find_height(n: i32, root: i32, edges: &Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let root = root as usize;
        let mut h = vec![0; n];
        h[root] = 1;
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut adj = vec![vec![]; n];
        for edge in edges {
            adj[edge[0] as usize].push(edge[1] as usize);
            adj[edge[1] as usize].push(edge[0] as usize);
        }

        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                if h[v] == 0 {
                    h[v] = h[u] + 1;
                    queue.push_back(v);
                }
            }
        }
        *h.iter().max().unwrap() - 1
    }

    #[allow(dead_code)]
    pub fn find_min_height_trees_tle(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let heights: Vec<i32> = (0..n).map(|i| Self::find_height(n, i, &edges)).collect();
        let min_height = heights.iter().min().unwrap();
        dbg!(&min_height);
        (0..n)
            .filter(|&i| heights[i as usize] == *min_height)
            .collect()
    }

    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        if n == 1 {
            return vec![0];
        }

        // Build the adjacency list
        let mut adj = vec![vec![]; n];
        let mut degree = vec![0; n];
        for edges in edges {
            let (u, v) = (edges[0] as usize, edges[1] as usize);
            adj[u].push(v);
            adj[v].push(u);
            degree[u] += 1;
            degree[v] += 1;
        }

        // Init the queue with all the leaves (degree == 1)
        let mut queue = VecDeque::new();
        for i in 0..n {
            if degree[i] == 1 {
                queue.push_back(i);
            }
        }

        // Iteratively trim the leaves until we have at most 2 nodes left
        let mut remaining = n;
        while remaining > 2 {
            let leaves_count = queue.len();
            remaining -= leaves_count;
            for _ in 0..leaves_count {
                let leaf = queue.pop_front().unwrap();
                for &nei in &adj[leaf] {
                    degree[nei] -= 1;
                    if degree[nei] == 1 {
                        queue.push_back(nei);
                    }
                }
            }
        }

        // The nodes left in the queue are the roots of MHTs
        queue.into_iter().map(|x| x as i32).collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
        vec![1]
    );
    assert_eq!(
        Solution::find_min_height_trees(
            6,
            vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
        ),
        vec![3, 4]
    );
}
