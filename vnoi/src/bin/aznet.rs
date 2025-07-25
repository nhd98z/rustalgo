use std::cmp::{max, min};

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

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
        }
        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else {
            self.parent[root_y] = root_x;
        }
        if self.rank[root_x] == self.rank[root_y] {
            self.rank[root_x] += 1;
        }
        true
    }
}

fn solve_one(n: usize, m: usize, a: Vec<usize>, b: Vec<usize>, u: Vec<usize>, v: Vec<usize>, c: Vec<usize>) {
    // Create UnionFind to check how many type-1 edges (c[i] == 1)
    // can be added without cycles.
    let mut uf = UnionFind::new(n + 1);
    let mut max_a = 0;
    // Loop through edges; if type is 1 and it connects two different components,
    // increment max_a.
    for i in 1..=m {
        if c[i] == 1 && uf.unite(u[i], v[i]) {
            max_a += 1;
        }
    }

    // Reset UnionFind to check how many type-2 edges (c[i] == 2)
    // can be added without cycles.
    let mut uf = UnionFind::new(n + 1);
    let mut max_b = 0;
    for i in 1..=m {
        if c[i] == 2 && uf.unite(u[i], v[i]) {
            max_b += 1;
        }
    }

    // We want exactly n-1 edges overall to form a spanning tree.
    // For i type-1 edges, the cost is a[i], plus b[n-1-i] for type-2 edges.
    // We'll find the value of i that yields the minimum cost.
    let mut a_optimize: usize = 0;
    let mut min_cost = usize::MAX;
    for i in max(0, n - max_b - 1)..=min(max_a, n - 1) {
        if min_cost > a[i] + b[n - i - 1] {
            min_cost = a[i] + b[n - i - 1];
            a_optimize = i;
        }
    }

    // We'll record which edges are used in the final spanning tree in 'res'.
    // res[i] = 1 means edge[i] is included.
    let mut res = vec![0; m + 1];

    // Attempt to unite type-1 edges on top of the current spanning tree formed by type-2 edges.
    // These type-1 edges must be included in the final result, because without them,
    // we cannot form a spanning tree with the required n - 1 edges.
    for i in 1..=m {
        if c[i] == 1 && uf.unite(u[i], v[i]) {
            res[i] = 1;
        }
    }

    // Rebuild UnionFind to reflect the edges we just chose.
    // Then count how many edges (so far) are actually in our tree.
    let mut uf = UnionFind::new(n + 1);
    for i in 1..=m {
        if res[i] == 1 && uf.unite(u[i], v[i]) {
            uf.unite(u[i], v[i]);
        }
    }
    let mut cnt = res.iter().filter(|e| **e == 1).count();

    // If we haven't used enough type-1 edges to match a_optimize,
    // try adding more (still ensuring they connect different components).
    for i in 1..=m {
        if cnt >= a_optimize {
            break;
        }
        if c[i] == 1 && uf.unite(u[i], v[i]) {
            cnt += 1;
            res[i] = 1;
        }
    }

    // Finally, add type-2 edges to connect the remaining components
    // until we form a valid spanning tree of n-1 edges.
    for i in 1..=m {
        if c[i] == 2 && uf.unite(u[i], v[i]) {
            res[i] = 1;
        }
    }

    // Print out the indices of edges chosen for the final spanning tree.
    for i in 1..=m {
        if res[i] == 1 {
            print!("{} ", i);
        }
    }
    println!();
}

fn solve<R: std::io::BufRead>(reader: &mut R) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let testcase: usize = s.trim().parse().unwrap();
    for _ in 0..testcase {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let n: usize = parse_next(&mut iter);
        let m: usize = parse_next(&mut iter);
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let mut a: Vec<usize> = vec![0; n];
        for i in 1..=n - 1 {
            a[i] = parse_next(&mut iter);
        }
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let mut b: Vec<usize> = vec![0; n];
        for i in 1..=n - 1 {
            b[i] = parse_next(&mut iter);
        }
        let mut u: Vec<usize> = vec![0; m + 1];
        let mut v: Vec<usize> = vec![0; m + 1];
        let mut c: Vec<usize> = vec![0; m + 1];
        for i in 1..=m {
            let mut s = String::new();
            reader.read_line(&mut s).unwrap();
            let mut iter = s.split_whitespace();
            u[i] = parse_next(&mut iter);
            v[i] = parse_next(&mut iter);
            c[i] = parse_next(&mut iter);
        }
        solve_one(n, m, a, b, u, v, c);
    }
}

fn parse_next<T: std::str::FromStr>(iter: &mut std::str::SplitWhitespace) -> T
where
    T::Err: std::fmt::Debug,
{
    iter.next().unwrap().parse().unwrap()
}

fn get_reader() -> Box<dyn std::io::BufRead> {
    if std::env::var("USER").unwrap_or_default() == "nhd98z" {
        let path = format!(
            "vnoi/src/bin/{}.txt",
            std::path::Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );
        match std::fs::File::open(&path) {
            Ok(file) => Box::new(std::io::BufReader::new(file)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                std::fs::File::create(&path).unwrap_or_else(|_| panic!("Failed to create input file: {}", &path));
                panic!("Input file not found. An empty file has been created.");
            }
            Err(e) => {
                panic!("Failed to open input file '{}': {}", &path, e);
            }
        }
    } else {
        Box::new(std::io::BufReader::new(std::io::stdin()))
    }
}

fn main() {
    let mut reader = get_reader();
    solve(&mut reader);
}
