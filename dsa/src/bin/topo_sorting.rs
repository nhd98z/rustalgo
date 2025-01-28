use std::collections::VecDeque;

struct Graph {
    n: usize,           // Number of nodes
    g: Vec<Vec<usize>>, // Adjacency list
}

impl Graph {
    fn new(n: usize, m: usize, edges: Vec<(usize, usize)>) -> Graph {
        let mut g = vec![vec![]; n];
        for i in 0..m {
            let (u, v) = edges[i];
            g[u].push(v);
        }
        Graph { n, g }
    }

    fn topo(self) -> Result<Vec<usize>, ()> {
        let mut indegree: Vec<usize> = vec![0; self.n];
        for u in 0..self.n {
            for &v in &self.g[u] {
                indegree[v] += 1;
            }
        }
        let mut sources = VecDeque::new();
        for u in 0..self.n {
            if indegree[u] == 0 {
                sources.push_back(u);
            }
        }
        let mut topo = vec![];
        while let Some(u) = sources.pop_front() {
            topo.push(u);
            for &v in &self.g[u] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    sources.push_back(v);
                }
            }
        }
        if topo.len() < self.n {
            return Err(());    
        }
        Ok(topo)
    }
}

fn main() {
    let n = 6;
    let m = 6;
    let edges = vec![(5, 2), (5, 0), (4, 0), (4, 1), (2, 3), (3, 1)];
    let g = Graph::new(n, m, edges);
    let topo = g.topo();
    println!("{:?}", topo);
}
