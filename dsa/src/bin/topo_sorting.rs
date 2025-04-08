struct Graph {
    n: usize,           // Number of nodes
    g: Vec<Vec<usize>>, // Adjacency list
}

impl Graph {
    fn new(n: usize, m: usize, edges: &Vec<(usize, usize)>) -> Graph {
        let mut g = vec![vec![]; n];
        for i in 0..m {
            let (u, v) = edges[i];
            g[u].push(v);
        }
        Graph { n, g }
    }

    fn topo_kahn(&self) -> Result<Vec<usize>, ()> {
        let mut indegree: Vec<usize> = vec![0; self.n];
        for u in 0..self.n {
            for &v in &self.g[u] {
                indegree[v] += 1;
            }
        }
        let mut sources = vec![];
        for u in 0..self.n {
            if indegree[u] == 0 {
                sources.push(u);
            }
        }
        let mut topo = vec![];
        while let Some(u) = sources.pop() {
            topo.push(u);
            for &v in &self.g[u] {
                indegree[v] -= 1;
                if indegree[v] == 0 {
                    sources.push(v);
                }
            }
        }
        if topo.len() < self.n {
            return Err(("Cycle detected", ()).1);
        }
        Ok(topo)
    }

    fn dfs(&self, st: &mut Vec<usize>, visit: &mut Vec<usize>, u: usize) -> Result<(), ()> {
        visit[u] = 1;
        for &v in &self.g[u] {
            if visit[v] == 1 {
                return Err(("Cycle detected", ()).1);
            }
            if visit[v] == 0 {
                self.dfs(st, visit, v)?;
            }
        }
        visit[u] = 2;
        st.push(u);
        Ok(())
    }

    fn topo_dfs(&self) -> Result<Vec<usize>, ()> {
        // 0 = unvisited, 1 = visiting, 2 = visited
        let mut visit = vec![0; self.n];
        let mut st = vec![];
        for u in 0..self.n {
            if visit[u] == 0 {
                self.dfs(&mut st, &mut visit, u)?;
            }
        }
        let topo = st.iter().rev().map(|&x| x).collect::<Vec<usize>>();
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
    let g = Graph::new(n, m, &edges);
    let topo = g.topo_kahn();
    println!("{:?}", topo);
    let topo = g.topo_dfs();
    println!("{:?}", topo);
}
