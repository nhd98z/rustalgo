use std::io::{self};

macro_rules! read_line {
    ($t:ty) => {{
        use std::io::BufRead;
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        line.split_whitespace().map(|w| w.parse::<$t>().unwrap()).collect::<Vec<$t>>()
    }};
    ($t:ty, $d:expr) => {{
        use std::io::BufRead;
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        line.trim_end_matches('\n').split($d).filter(|s| !s.is_empty()).map(|w| w.parse::<$t>().unwrap()).collect::<Vec<$t>>()
    }};
}

fn main() {
    // Example 1: read the whole first line into a vector
    let nm: Vec<usize> = read_line!(usize); // e.g. input "12 34\n"
    let (n, m) = (nm[0], nm[1]);
    println!("read_line → n={n}, m={m}");
}
