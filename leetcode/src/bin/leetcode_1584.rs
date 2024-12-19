struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let rank = vec![0; n];
        UnionFind { parent, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
        true
    }
}

impl Solution {
    // Kruskal's algorithm O(E * logV)
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut edges = vec![];
        for i in 0..n {
            for j in i + 1..n {
                let dist =
                    (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs();
                edges.push((dist, i, j));
            }
        }
        edges.sort_unstable();
        let mut uf = UnionFind::new(n);
        let mut ans = 0;
        for (dist, i, j) in edges {
            if uf.union(i, j) {
                ans += dist;
            }
        }
        ans
    }

    // Prim's algorithm (V^2)
    pub fn min_cost_connect_points_prim(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut dist = vec![i32::MAX; n];
        let mut visited = vec![false; n];
        let mut ans = 0;
        dist[0] = 0;
        for _ in 0..n {
            let mut min_dist = i32::MAX;
            let mut u = 0;
            for i in 0..n {
                if !visited[i] && dist[i] < min_dist {
                    min_dist = dist[i];
                    u = i;
                }
            }
            visited[u] = true;
            ans += min_dist;
            for v in 0..n {
                let d = (points[u][0] - points[v][0]).abs() + (points[u][1] - points[v][1]).abs();
                if !visited[v] && d < dist[v] {
                    dist[v] = d;
                }
            }
        }
        ans
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::min_cost_connect_points(vec![
            vec![0, 0],
            vec![2, 2],
            vec![3, 10],
            vec![5, 2],
            vec![7, 0]
        ]),
        20
    );
    assert_eq!(
        Solution::min_cost_connect_points_prim(vec![
            vec![0, 0],
            vec![2, 2],
            vec![3, 10],
            vec![5, 2],
            vec![7, 0]
        ]),
        20
    );
}
