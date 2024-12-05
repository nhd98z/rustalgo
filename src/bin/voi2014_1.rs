#![allow(dead_code, unused)]

use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn solve_one<R: BufRead>(reader: &mut R) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let mut iter = s.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();
    let mut d = vec![0; n as usize];
    let mut k = vec![0; n as usize];
    for i in 0..n {
        s.clear();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        d[i as usize] = iter.next().unwrap().parse().unwrap();
        k[i as usize] = iter.next().unwrap().parse().unwrap();
    }
    let mut combined: Vec<_> = d.into_iter().zip(k.into_iter()).collect();
    combined.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let (d, k): (Vec<_>, Vec<_>) = combined.into_iter().unzip();
    let mut aa = 0;
    let mut bb = 0;
    let mut i = 0;
    let mut j = 0;
    let mut res = i64::MAX;
    while j < n {
        if k[j] == 1 {
            aa += 1;
        } else {
            bb += 1;
        }
        while aa >= a && bb >= b && i < j {
            res = i64::min(res, d[j] - d[i]);
            if k[i] == 1 {
                aa -= 1;
            } else {
                bb -= 1;
            }
            i += 1;
        }
        j += 1;
    }
    println!("{}", if res == i64::MAX { -1 } else { res });
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
