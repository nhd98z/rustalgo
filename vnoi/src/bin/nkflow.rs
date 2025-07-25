#![allow(unused)]

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

// Cạnh trong đồ thị thặng dư cho bài toán luồng cực đại
#[derive(Clone)]
struct Edge {
    to: usize,  // đỉnh đích
    rev: usize, // chỉ số của cạnh ngược trong danh sách kề của đỉnh 'to'
    cap: usize, // khả năng thông qua còn lại của cạnh này
}

fn main() {
    // Đọc input: n đỉnh, m cạnh, nguồn s, đích t
    let [n, m, s, t] = read_arr!(usize).try_into().unwrap();
    println!("=== XÂY DỰNG ĐỒ THỊ ===");
    println!("Số đỉnh: {}, Số cạnh: {}", n, m);
    println!("Nguồn: {}, Đích: {}", s, t);

    let mut raw_edges = vec![];
    for _ in 0..m {
        let [u, v, c] = read_arr!(usize).try_into().unwrap();
        println!("Thêm cạnh: {} -> {} với capacity {}", u, v, c);
        raw_edges.push((u - 1, v - 1, c)); // Convert to 0-indexed
    }

    let mut g = build_graph(n, &raw_edges);
    println!();
    let flow = max_flow(&mut g, s - 1, t - 1); // Convert source and target to 0-indexed
    println!();
    println!("🎯 LUỒNG CỰC ĐẠI: {}", flow);
}

// Xây dựng đồ thị thặng dư với cạnh thuận và cạnh ngược
fn build_graph(n: usize, edges: &[(usize, usize, usize)]) -> Vec<Vec<Edge>> {
    let mut graph = vec![vec![]; n];
    println!("📝 Xây dựng đồ thị thặng dư:");
    for &(u, v, cap) in edges {
        // Lưu chỉ số của các cạnh ngược (cần thiết để cập nhật luồng)
        let rev_u = graph[v].len(); // chỉ số của cạnh ngược u->v trong graph[v]
        let rev_v = graph[u].len(); // chỉ số của cạnh ngược v->u trong graph[u]

        // Thêm cạnh thuận u->v với khả năng thông qua cap
        graph[u].push(Edge { to: v, rev: rev_u, cap });
        // Thêm cạnh ngược v->u với khả năng thông qua 0 (để hủy luồng)
        graph[v].push(Edge {
            to: u,
            rev: rev_v,
            cap: 0,
        });
        println!("  ➕ Cạnh thuận: {} -> {} (cap: {})", u + 1, v + 1, cap);
        println!("  ➕ Cạnh ngược: {} -> {} (cap: 0)", v + 1, u + 1);
    }
    graph
}

// Thuật toán Edmonds-Karp: dùng BFS tìm đường tăng luồng và đẩy luồng
fn bfs(s: usize, t: usize, graph: &mut [Vec<Edge>]) -> usize {
    let n = graph.len();
    // parent[v] = Some((u, edge_idx)) có nghĩa là ta đến v từ u qua cạnh thứ edge_idx
    let mut parent = vec![None; n];
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();

    queue.push_back(s);
    visited[s] = true;
    println!("🔍 Bắt đầu BFS từ đỉnh {}...", s + 1);

    // BFS tìm đường tăng luồng ngắn nhất từ s đến t
    while let Some(u) = queue.pop_front() {
        if u == t {
            println!("🎯 Đã tìm thấy đường đến đích {}!", t + 1);
            break;
        }

        // Thử tất cả các cạnh từ đỉnh u hiện tại
        for (i, edge) in graph[u].iter().enumerate() {
            // Có thể dùng cạnh này nếu: còn khả năng thông qua VÀ đỉnh chưa được thăm
            if !visited[edge.to] && edge.cap > 0 {
                visited[edge.to] = true;
                parent[edge.to] = Some((u, i)); // ghi lại cách ta đến đây
                queue.push_back(edge.to);
                println!("  → Từ {} đến {} (capacity: {})", u + 1, edge.to + 1, edge.cap);
            }
        }
    }

    // Nếu không thể đến được đích, không còn đường tăng luồng nào
    if !visited[t] {
        println!("❌ Không thể đến được đích từ nguồn!");
        return 0;
    }

    // Tìm đường đi và in ra
    let mut path = Vec::new();
    let mut cur = t;
    while let Some((prev, edge_idx)) = parent[cur] {
        path.push((prev, cur, graph[prev][edge_idx].cap));
        cur = prev;
    }
    path.reverse();

    print!("📍 Đường tìm được: {}", s + 1);
    for &(from, to, cap) in &path {
        print!(" -> {}({})", to + 1, cap);
    }
    println!();

    // Tìm khả năng thông qua tối thiểu (bottleneck) dọc theo đường từ s đến t
    let mut bottleneck = usize::MAX;
    cur = t;
    while let Some((prev, edge_idx)) = parent[cur] {
        // Cập nhật bottleneck với khả năng thông qua tối thiểu dọc đường
        bottleneck = bottleneck.min(graph[prev][edge_idx].cap);
        cur = prev;
    }
    println!("🔢 Bottleneck (luồng có thể đẩy): {}", bottleneck);

    // Đẩy luồng dọc theo đường: cập nhật khả năng thông qua thặng dư
    cur = t;
    while let Some((prev, edge_idx)) = parent[cur] {
        let Edge { to, rev, .. } = graph[prev][edge_idx];
        // Giảm khả năng thông qua của cạnh thuận
        graph[prev][edge_idx].cap -= bottleneck;
        // Tăng khả năng thông qua của cạnh ngược (để có thể hủy luồng)
        graph[to][rev].cap += bottleneck;
        println!(
            "  🔄 Cập nhật: {} -> {} (còn {}), {} -> {} (thêm {})",
            prev + 1,
            to + 1,
            graph[prev][edge_idx].cap,
            to + 1,
            prev + 1,
            graph[to][rev].cap
        );
        cur = prev;
    }

    bottleneck // trả về lượng luồng đã đẩy
}

// Phương pháp Ford-Fulkerson: tiếp tục tìm đường tăng luồng cho đến khi không còn
fn max_flow(graph: &mut Vec<Vec<Edge>>, s: usize, t: usize) -> usize {
    let mut flow = 0;
    let mut iteration = 1;
    println!("=== BẮT ĐẦU TÌM LUỒNG CỰC ĐẠI ===");

    loop {
        println!("--- Lần lặp {} ---", iteration);
        let pushed = bfs(s, t, graph); // tìm đường và đẩy luồng
        if pushed == 0 {
            println!("❌ Không tìm thấy đường tăng luồng nào nữa!");
            break; // không còn đường tăng luồng = đã tìm được luồng cực đại
        }
        flow += pushed; // tích lũy tổng luồng
        println!("✅ Đã đẩy {} đơn vị luồng. Tổng luồng hiện tại: {}", pushed, flow);
        println!();
        iteration += 1;
    }
    flow
}
