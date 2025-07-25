use std::collections::VecDeque;

impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        // Cho biết có num_courses khóa học (0...num_courses - 1), và prerequisites là các cạnh có hướng của đồ thị.
        // Với mỗi query i, trả về true nếu query[i][0] có thể đi được đến query[i][1]
        //
        // Đồ thị có thể có nhiều thành phần liên thông, và có thể có chu trình.
        //
        //
        // Đối với bài này, ta phải tiền xử lý đồ thị, sử dụng BFS để tìm ra đường đi từ một đỉnh đến tất cả các đỉnh khác. O(N*(V + E))
        //
        // Sau đó, với mỗi query, ta chỉ cần kiểm tra xem đỉnh query[i][0] có thể đi đến đỉnh query[i][1] hay không.
        // O(1)
        //
        // Tổng: O(N*(V + E))
        //
        let n = num_courses as usize;
        let prerequisites = prerequisites
            .into_iter()
            .map(|edge| (edge[0] as usize, edge[1] as usize))
            .collect::<Vec<_>>();
        let queries = queries
            .into_iter()
            .map(|edge| (edge[0] as usize, edge[1] as usize))
            .collect::<Vec<_>>();

        let mut graph = vec![vec![]; n];
        for (u, v) in prerequisites {
            graph[u].push(v);
        }

        let mut reach = vec![vec![false; n]; n];
        for i in 0..n {
            let mut visited = vec![false; n];
            let mut queue = VecDeque::new();
            queue.push_back(i);
            visited[i] = true;
            while let Some(node) = queue.pop_front() {
                for &neighbor in &graph[node] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        reach[i][neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        let mut res = vec![false; queries.len()];
        for (i, query) in queries.into_iter().enumerate() {
            res[i] = reach[query.0][query.1];
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::check_if_prerequisite(2, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]]),
        vec![false, true]
    );
}
