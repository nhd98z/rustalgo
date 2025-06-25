use std::{
    io::{self, BufRead},
    str::SplitWhitespace,
    sync::{Mutex, OnceLock},
};

// Global iterator for buffered token reading
static TOKENS: OnceLock<Mutex<SplitWhitespace<'static>>> = OnceLock::new();

/// Initialize or fetch the token iterator, reading one line and splitting into tokens
fn token_iter() -> &'static Mutex<SplitWhitespace<'static>> {
    TOKENS.get_or_init(|| {
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        let static_str: &'static str = Box::leak(line.into_boxed_str());
        Mutex::new(static_str.split_whitespace())
    })
}

/// Fetch next token of type T, reading a new line if current buffer is exhausted
fn next_token<T: std::str::FromStr>() -> T
where
    T::Err: std::fmt::Debug,
{
    let mut iter = token_iter().lock().unwrap();
    if let Some(tok) = iter.next() {
        tok.parse().unwrap()
    } else {
        drop(iter);
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        let static_str: &'static str = Box::leak(line.into_boxed_str());
        let mut new_iter = static_str.split_whitespace();
        let tok = new_iter.next().expect("No token found");
        *TOKENS.get().unwrap().lock().unwrap() = new_iter;
        tok.parse().unwrap()
    }
}

/// Macro for reading a single whitespace-separated token as type T
macro_rules! read {
    ($t:ty) => {
        next_token::<$t>()
    };
}

/// Macro for reading a full line into Vec<T>, with optional custom delimiter
macro_rules! read_arr {
    ($t:ty) => {{
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        line.split_whitespace()
            .map(|w| w.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
    ($t:ty, $d:expr) => {{
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        line.trim_end_matches('\n')
            .split($d)
            .filter(|s| !s.is_empty())
            .map(|w| w.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}

/// Macro for reading a full line as String or parsing it into T or Vec<T>
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        line.trim_end_matches(&['\r', '\n'][..]).to_string()
    }};
    ($t:ty) => {{
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        line.trim_end_matches(&['\r', '\n'][..])
            .parse::<$t>()
            .unwrap()
    }};
    ($t:ty, $d:expr) => {{
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        line.trim_end_matches('\n')
            .split($d)
            .filter(|s| !s.is_empty())
            .map(|w| w.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}

fn main() {
    let n = read!(usize);
    let m = read!(usize);
    let s = read!(String);
    println!("n = {}, m = {}, s = {}", n, m, s);

    let v = read_arr!(usize);
    println!("read_arr → v={:?}", v);

    let line = read_line!();
    println!("readline → {}", line);
}
