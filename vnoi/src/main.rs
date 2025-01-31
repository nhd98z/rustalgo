use std::fmt::Debug;
use std::str::FromStr;

fn solve_one(n: usize) {
    println!("n = {:?}", &n);
}

fn solve<R: std::io::BufRead>(reader: &mut R) {
    // let mut s = String::new();
    // reader.read_line(&mut s).unwrap();
    // let testcase: usize = s.trim().parse().unwrap();
    let testcase: usize = 1;
    for _ in 0..testcase {
        let [n]: [usize; 1] = readline(reader).as_slice().try_into().unwrap();
        solve_one(n);
    }
}

fn readline<T: FromStr, R: std::io::BufRead + ?Sized>(reader: &mut R) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    s.split_whitespace()
        .map(|e| e.parse::<T>().unwrap())
        .collect::<Vec<T>>()
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
                    .unwrap_or_else(|_| panic!("Failed to create input file: {}", &path));
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
