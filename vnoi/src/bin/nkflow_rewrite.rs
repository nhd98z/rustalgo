#![allow(unused)]

use std::{
    collections::VecDeque,
    io::{self, BufRead},
    str::SplitWhitespace,
    sync::{Mutex, OnceLock},
    usize,
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
        line.trim_end_matches(&['\r', '\n'][..]).parse::<$t>().unwrap()
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

#[derive(Clone)]
struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

fn main() {
    let [n, m, mut s, mut t] = read_arr!(usize).try_into().unwrap();
    s -= 1; // index 0
    t -= 1;
    let mut g: Vec<Vec<Edge>> = vec![vec![]; n];
    for _ in 0..m {
        let [mut u, mut v, c] = read_arr!(usize).try_into().unwrap();
        u -= 1; // index 0
        v -= 1;
        let rev_u = g[u].len();
        let rev_v = g[v].len();
        g[u].push(Edge {
            to: v,
            cap: c,
            rev: rev_v,
        });
        g[v].push(Edge {
            to: u,
            cap: 0,
            rev: rev_u,
        });
    }

    let mut max_flow = 0;
    loop {
        let inc = bfs(&mut g, n, m, s, t);
        if inc == 0 {
            break;
        }
        max_flow += inc;
    }
    println!("{}", max_flow);
}

fn bfs(g: &mut [Vec<Edge>], n: usize, m: usize, s: usize, t: usize) -> usize {
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    let mut parent = vec![None; n]; // parent[v] = Some((u, edge_idx)) means we go to v from u at edge_idx

    queue.push_back(s);
    visited[s] = true;

    while let Some(u) = queue.pop_front() {
        if u == t {
            break;
        }

        for (i, edge) in g[u].iter().enumerate() {
            if !visited[edge.to] && edge.cap > 0 {
                visited[edge.to] = true;
                parent[edge.to] = Some((u, i));
                queue.push_back(edge.to);
            }
        }
    }

    if !visited[t] {
        return 0;
    }

    // Find bottleneck
    let mut bottleneck = usize::MAX;
    let mut cur = t;
    while let Some((prev, edge_idx)) = parent[cur] {
        bottleneck = bottleneck.min(g[prev][edge_idx].cap);
        cur = prev;
    }

    // Inc flow
    cur = t;
    while let Some((prev, edge_idx)) = parent[cur] {
        let Edge { to, rev, .. } = g[prev][edge_idx];
        g[prev][edge_idx].cap -= bottleneck;
        g[to][rev].cap += bottleneck;
        cur = prev;
    }

    bottleneck
}
