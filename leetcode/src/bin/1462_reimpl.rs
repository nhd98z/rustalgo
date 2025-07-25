use std::collections::VecDeque;

impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // Đề bài cho đồ thị có hướng, có nhiều thành phần liên thông
        // Ta cần trả lời các truy vấn, mỗi truy vấn yêu cầu kiểm tra xem có đường đi từ đỉnh u đến đỉnh v hay không

        // Gọi số lượng đỉnh là n (max n = 100), số lượng cạnh là m (max m = 1e4), số lượng truy vấn là q (max q = 1e4)

        // Naive approach:
        // - Tạo đồ thị từ prerequisites
        // - Với mỗi truy vấn, kiểm tra xem có đường đi từ u đến v hay không bằng BFS
        // - Độ phức tạp: O(q * (n + m)) = O(1e4 * (100 + 1e4)) = O(1e8)

        // Better approach:
        // - Tạo đồ thị từ prerequisites
        // - Tiền xử lý đồ thị bằng BFS, xây dựng mảng is_reachable[u][v] = true nếu có đường đi từ u đến v
        // - Độ phức tạp: O(n + m) + O(q) = O(1e2 + 1e4) + O(1e4) = O(1e4)

        let n = num_courses as usize;
        let prerequisites = prerequisites
            .into_iter()
            .map(|v| (v[0] as usize, v[1] as usize))
            .collect::<Vec<_>>();
        let queries = queries
            .into_iter()
            .map(|v| (v[0] as usize, v[1] as usize))
            .collect::<Vec<_>>();

        let mut graph = vec![vec![]; n];
        for (u, v) in prerequisites {
            graph[u].push(v);
        }

        let mut is_reachable = vec![vec![false; n]; n];
        for i in 0..n {
            let mut visited = vec![false; n];
            let mut queue = VecDeque::new();
            queue.push_back(i);
            while let Some(u) = queue.pop_front() {
                if visited[u] {
                    continue;
                }
                visited[u] = true;
                for &v in graph[u].iter() {
                    if !visited[v] {
                        queue.push_back(v);
                    }
                    is_reachable[i][v] = true;
                }
            }
        }

        queries.into_iter().map(|(u, v)| is_reachable[u][v]).collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::check_if_prerequisite(2, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]]),
        vec![false, true]
    );
}
