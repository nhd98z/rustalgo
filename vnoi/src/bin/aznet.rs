#![allow(dead_code, non_snake_case)]

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
        // println!("x = {:?}", x);
        // println!("y = {:?}", y);
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

fn solve_one<R: std::io::BufRead>(reader: &mut R) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let mut iter = s.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let mut iter = s.split_whitespace();
    for i in 1..=n - 1 {
        a[i] = iter.next().unwrap().parse().unwrap();
    }
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let mut iter = s.split_whitespace();
    for i in 1..=n - 1 {
        b[i] = iter.next().unwrap().parse().unwrap();
    }
    let mut edges = vec![(0, 0, 0); m + 1];
    for i in 1..=m {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let u: usize = iter.next().unwrap().parse::<usize>().unwrap();
        let v: usize = iter.next().unwrap().parse::<usize>().unwrap();
        let c: usize = iter.next().unwrap().parse().unwrap();
        edges[i] = (u, v, c);
    }
    // // println!("n = {:?}", n);
    // // println!("m = {:?}", m);
    // // println!("a = {:?}", a);
    // // println!("b = {:?}", b);
    // // println!("edges = {:?}", edges);
    let mut A = 0;
    let mut uf = UnionFind::new(n + 1);
    for i in 1..=m {
        if edges[i].2 == 1 && uf.unite(edges[i].0, edges[i].1) {
            A += 1;
        }
    }
    let mut B = 0;
    let mut uf = UnionFind::new(n + 1);
    for i in 1..=m {
        if edges[i].2 == 2 && uf.unite(edges[i].0, edges[i].1) {
            B += 1;
        }
    }
    // println!("A = {:?}", A);
    // println!("B = {:?}", B);
    let mut x = 0;
    let mut min = usize::MAX;
    for i in usize::max(0, n - 1 - B)..=usize::min(n - 1, A) {
        // println!("i = {:?}", i);
        if a[i] + b[n - 1 - i] < min {
            // println!("ii = {:?}", i);
            min = a[i] + b[n - 1 - i];
            x = i;
        }
    }
    // println!("x = {:?}", x);
    let mut dd: Vec<usize> = vec![0; m + 1];
    for i in 1..=m {
        if edges[i].2 == 1 && uf.unite(edges[i].0, edges[i].1) {
            dd[i] = 1;
        }
    }
    // println!("dd = {:?}", dd);
    let mut uf = UnionFind::new(n + 1);
    let mut cnt = 0;
    for i in 1..m {
        if dd[i] == 1 && uf.unite(edges[i].0, edges[i].1) {
            cnt += 1;
        }
    }
    for i in 1..=m {
        if cnt >= x {
            break;
        }
        if edges[i].2 == 1 && dd[i] == 0 && uf.unite(edges[i].0, edges[i].1) {
            dd[i] = 1;
            cnt += 1;
        }
    }
    // println!("dd = {:?}", dd);
    for i in 1..=m {
        if edges[i].2 == 2 && uf.unite(edges[i].0, edges[i].1) {
            dd[i] = 1;
        }
    }
    // println!("dd = {:?}", dd);
    for i in 1..=m {
        if dd[i] == 1 {
            print!("{:?} ", i);
        }
    }
    print!("\n");
}

fn solve<R: std::io::BufRead>(reader: &mut R) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let testcase: usize = s.trim().parse().unwrap();
    // let testcase: usize = 1;
    for _ in 0..testcase {
        solve_one(reader);
    }
}

fn get_reader() -> Box<dyn std::io::BufRead> {
    if std::env::var("USER").unwrap_or_default() == "nhd98z" {
        let path = format!(
            "vnoi/src/bin/{}.txt",
            std::path::Path::new(file!())
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
        );
        match std::fs::File::open(&path) {
            Ok(file) => Box::new(std::io::BufReader::new(file)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                std::fs::File::create(&path)
                    .expect(&format!("Failed to create input file: {}", &path));
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
