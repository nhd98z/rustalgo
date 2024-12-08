use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn solve_one<R: BufRead>(reader: &mut R) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let n: usize = s.trim().parse().unwrap();
    dbg!(n);
}

fn solve<R: BufRead>(reader: &mut R) {
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    let testcase: usize = s.trim().parse().unwrap();
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
