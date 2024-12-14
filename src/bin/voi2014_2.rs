use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};


fn solve_one(m: usize, n: usize, a: Vec<usize>, b: Vec<usize>) {
    let mut dp = vec![0; n];
    for i in 0..m {
        let mut curr = 0;
        for j in 0..n {
            let prev = curr;
            if b[j] * 2 <= a[i] {
                curr = usize::max(curr, dp[j]);
            }
            if a[i] == b[j] {
                dp[j] = usize::max(dp[j], prev + 1);
            }
        }
    }
    let res = dp.into_iter().max().unwrap();
    println!("{}", res);
}

fn solve<R: BufRead>(reader: &mut R) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let testcase: usize = s.trim().parse().unwrap();
    for _ in 0..testcase {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let m: usize = iter.next().unwrap().parse().unwrap();
        let n: usize = iter.next().unwrap().parse().unwrap();
        s.clear();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let mut a = vec![0; m];
        for i in 0..m {
            a[i] = iter.next().unwrap().parse().unwrap();
        }
        s.clear();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let mut b = vec![0; n];
        for j in 0..n {
            b[j] = iter.next().unwrap().parse().unwrap();
        }
        solve_one(m, n, a, b);
    }
}

fn get_reader() -> Box<dyn BufRead> {
    if env::var("USER").unwrap_or_default() == "nhd98z" {
        let path = format!(
            "src/bin/{}.txt",
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
