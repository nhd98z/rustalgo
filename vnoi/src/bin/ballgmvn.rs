#![allow(dead_code)]

fn solve_one<R: std::io::BufRead>(reader: &mut R) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let mut iter = s.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        a.push((x, y));
    }
    for _ in 0..n {
        let mut s = String::new();
        reader.read_line(&mut s).unwrap();
        let mut iter = s.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let y: i32 = iter.next().unwrap().parse().unwrap();
        b.push((x, y));
    }
    for i in 0..n {
        let mut bb = b.clone();
        let mut slopes = vec![];
        for j in 0..n {
            bb[j].0 -= a[i].0;
            bb[j].1 -= a[i].1;
            if bb[j].0 == 0 {
                slopes.push((f64::INFINITY, j));
            } else {
                slopes.push((bb[j].1 as f64 / bb[j].0 as f64, j));
            }
        }
        slopes.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
        // println!("bb = {:?}", &bb);
        for j in 0..n - 1 {
            if slopes[j].0 == slopes[j + 1].0 {
                println!("{} {} {}", i + 1, slopes[j].1 + n + 1, slopes[j + 1].1 + n + 1);
                return;
            }
        }
    }
    for j in 0..n {
        let mut aa = a.clone();
        let mut slopes = vec![];
        for i in 0..n {
            aa[i].0 -= b[j].0;
            aa[i].1 -= b[j].1;
            if aa[i].0 == 0 {
                slopes.push((f64::INFINITY, i));
            } else {
                slopes.push((aa[i].1 as f64 / aa[i].0 as f64, i));
            }
        }
        slopes.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
        // println!("aa = {:?}", &aa);
        for i in 0..n - 1 {
            if slopes[i].0 == slopes[i + 1].0 {
                println!("{} {} {}", slopes[i].1 + 1, slopes[i + 1].1 + 1, j + n + 1);
                return;
            }
        }
    }
    println!("-1");
}

fn solve<R: std::io::BufRead>(reader: &mut R) {
    // let mut s = String::new();
    // reader.read_line(&mut s).unwrap();
    // let testcase: usize = s.trim().parse().unwrap();
    let testcase: usize = 1;
    for _ in 0..testcase {
        solve_one(reader);
    }
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
                std::fs::File::create(&path).expect(&format!("Failed to create input file: {}", &path));
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
