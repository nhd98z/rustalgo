use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

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
        } else {
            self.parent[root_y] = root_x;
        }
        if self.rank[root_x] == self.rank[root_y] {
            self.rank[root_x] += 1;
        }
        true
    }
}

fn solve_one<R: BufRead>(reader: &mut R) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let mut iter = s.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut edges = vec![];
    for _ in 0..m {
        s.clear();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        let w: i64 = iter.next().unwrap().parse().unwrap();
        edges.push((w, u - 1, v - 1));
    }
    edges.sort_unstable();
    let mut ef = UnionFind::new(n);
    let mut ans = 0;
    for (w, u, v) in edges {
        if ef.union(u, v) {
            ans += w;
        }
    }
    println!("{}", ans);
}

fn solve<R: BufRead>(reader: &mut R) {
    let testcase: usize = 1;
    for _ in 0..testcase {
        solve_one(reader);
    }
}

fn get_reader() -> Box<dyn BufRead> {
    if env::var("USER").unwrap_or_default() == "nhd98z" {
        let path = format!(
            "vnoi/src/bin/{}.txt",
            Path::new(file!()).file_stem().unwrap().to_str().unwrap()
        );
        match File::open(&path) {
            Ok(file) => Box::new(BufReader::new(file)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                File::create(&path).expect(&format!("Failed to create input file: {}", &path));
                panic!("Input file not found. An empty file has been created.");
            }
            Err(e) => {
                panic!("Failed to open input file '{}': {}", &path, e);
            }
        }
    } else {
        Box::new(BufReader::new(io::stdin()))
    }
}

fn main() {
    let mut reader = get_reader();
    solve(&mut reader);
}
