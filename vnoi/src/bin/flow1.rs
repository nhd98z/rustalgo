#![allow(dead_code, unused_macros)]

use std::{
    collections::VecDeque,
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

// Cạnh trong đồ thị thặng dư cho bài toán luồng cực đại
#[derive(Clone)]
struct Edge {
    to: usize,  // đỉnh đích
    rev: usize, // chỉ số của cạnh ngược trong danh sách kề của đỉnh 'to'
    cap: usize, // khả năng thông qua còn lại của cạnh này
}

// Thuật toán Edmonds-Karp: dùng BFS tìm đường tăng luồng và đẩy luồng
fn bfs(s: usize, t: usize, graph: &mut [Vec<Edge>]) -> usize {
    let n = graph.len();
    // parent[v] = Some((u, edge_idx)) có nghĩa là ta đến v từ u qua cạnh thứ edge_idx
    let mut parent = vec![None; n];
    let mut queue = VecDeque::from([s]);
    parent[s] = Some((s, 0)); // đánh dấu nguồn đã được thăm

    // BFS tìm đường tăng luồng ngắn nhất từ s đến t
    while let Some(u) = queue.pop_front() {
        if u == t {
            break; // đã đến đích, dừng BFS
        }
        // Thử tất cả các cạnh từ đỉnh u hiện tại
        for (i, edge) in graph[u].iter().enumerate() {
            // Có thể dùng cạnh này nếu: còn khả năng thông qua VÀ đỉnh chưa được thăm
            if edge.cap > 0 && parent[edge.to].is_none() {
                parent[edge.to] = Some((u, i)); // ghi lại cách ta đến đây
                queue.push_back(edge.to);
            }
        }
    }

    // Nếu không thể đến được đích, không còn đường tăng luồng nào
    if parent[t].is_none() {
        return 0;
    }

    // Tìm khả năng thông qua tối thiểu (bottleneck) dọc theo đường từ s đến t
    let mut flow = usize::MAX;
    let mut cur = t;
    while let Some((prev, edge_idx)) = parent[cur] {
        if cur != s {
            // Cập nhật bottleneck với khả năng thông qua tối thiểu dọc đường
            flow = flow.min(graph[prev][edge_idx].cap);
        }
        cur = prev;
    }

    // Đẩy luồng dọc theo đường: cập nhật khả năng thông qua thặng dư
    cur = t;
    while let Some((prev, edge_idx)) = parent[cur] {
        if cur != s {
            let rev = graph[prev][edge_idx].rev;
            // Giảm khả năng thông qua của cạnh thuận
            graph[prev][edge_idx].cap -= flow;
            // Tăng khả năng thông qua của cạnh ngược (để có thể hủy luồng)
            graph[cur][rev].cap += flow;
        }
        cur = prev;
    }
    flow // trả về lượng luồng đã đẩy
}

fn main() {
    // Đọc input: n đỉnh, m cạnh, nguồn s, đích t
    let [n, m, s, t] = read_arr!(usize).try_into().unwrap();
    let (s, t) = (s - 1, t - 1); // chuyển về chỉ số bắt đầu từ 0

    // Xây dựng đồ thị thặng dư với cạnh thuận và cạnh ngược
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let [u, v, c] = read_arr!(usize).try_into().unwrap();
        let (u, v) = (u - 1, v - 1); // chuyển về chỉ số bắt đầu từ 0

        // Lưu chỉ số của các cạnh ngược (cần thiết để cập nhật luồng)
        let rev_u = graph[v].len(); // chỉ số của cạnh ngược u->v trong graph[v]
        let rev_v = graph[u].len(); // chỉ số của cạnh ngược v->u trong graph[u]

        // Thêm cạnh thuận u->v với khả năng thông qua c
        graph[u].push(Edge {
            to: v,
            rev: rev_u,
            cap: c,
        });
        // Thêm cạnh ngược v->u với khả năng thông qua 0 (để hủy luồng)
        graph[v].push(Edge {
            to: u,
            rev: rev_v,
            cap: 0,
        });
    }

    // Phương pháp Ford-Fulkerson: tiếp tục tìm đường tăng luồng cho đến khi không còn
    let mut flow = 0;
    loop {
        let pushed = bfs(s, t, &mut graph); // tìm đường và đẩy luồng
        if pushed == 0 {
            break; // không còn đường tăng luồng = đã tìm được luồng cực đại
        }
        flow += pushed; // tích lũy tổng luồng
    }
    println!("{}", flow);
}
