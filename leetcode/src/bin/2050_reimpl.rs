impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        // Đề bài cho đồ thị có hướng, có nhiều thành phần liên thông
        // Để học khóa học ở v, ta cần học xong ít nhất 1 khóa học ở u trước đó
        // Ta cần tìm thời gian tối thiểu để hoàn thành tất cả các khóa học

        // Giả sử khóa học u1 (1 tháng), u2 (2 tháng), u3 (3 tháng) là điều kiện cần để học khóa học v (4 tháng)
        // Ta có quan sát: ta có thể học u1 song song với u2, u3
        // Do đó, thời gian tối thiểu để học hết tất cả các khóa học là max(1, 2, 3) + 4 = 8 tháng

        // Gọi số lượng đỉnh là n (max n = 5e4), số lượng cạnh là m (max m = 5e4)

        // Để giải bài này, ta phải xây dựng đồ thị ngược, sau đó sử dụng DFS kết hợp với DP:
        // Ở mỗi bước DFS, ở đỉnh v, ta tìm được thời gian max để học hết tất cả các khóa học mà phụ thuộc vào v (u1, u2, u3 như ví dụ trên)

        // Vì đồ thị có nhiều thành phần liên thông, ta cần DFS từ từng đỉnh
        // Độ phức tạp: O(n * (n + m)) = O(5e4 * (5e4 + 5e4)) = O(5e8) (cần kiểm chứng)

        let n = n as usize;
        let time = time.into_iter().map(|t| t as usize).collect::<Vec<_>>();

        let mut rev_graph = vec![vec![]; n];
        for relation in relations {
            let u = relation[0] as usize - 1;
            let v = relation[1] as usize - 1;
            rev_graph[v].push(u);
        }

        // let mut res = 0;
        // let mut visited = vec![false; n];
        // for i in 0..n {
        //     if !visited[i] {
        //         fn dfs(v: usize, rev_graph: &Vec<Vec<usize>>, time: &Vec<usize>, visited: &mut Vec<bool>, res: &mut usize) {
        //             visited[v] = true;
        //             let mut max_time = 0;
        //             for &u in rev_graph[v].iter() {
        //                 if !visited[u] {
        //                     dfs(u, rev_graph, time, visited, res);
        //                 }
        //                 max_time = max_time.max(time[u]);
        //             }
        //             *res = max_time + time[v];
        //         }
        //         dfs(i, &rev_graph, &time, &mut visited, &mut res);
        //     }
        // }
        // res as i32

        // Thuật toán trên sai, vì đồ thị có nhiều thành phần liên thông nên res phải thay thành mảng dp
        // dp[i] là thời gian tối thiểu để học hết tất cả các khóa học mà phụ thuộc vào i
        // Kết quả là max(dp[i]) với i chạy từ 0 đến n - 1

        let mut dp = vec![0; n];
        let mut visited = vec![false; n];

        fn dfs(v: usize, rev_graph: &Vec<Vec<usize>>, time: &Vec<usize>, visited: &mut Vec<bool>, dp: &mut Vec<usize>) {
            if visited[v] {
                return;
            }
            visited[v] = true;
            let mut max_prereq_time = 0;
            for &u in rev_graph[v].iter() {
                dfs(u, rev_graph, time, visited, dp);
                max_prereq_time = max_prereq_time.max(dp[u]);
            }
            dp[v] = max_prereq_time + time[v];
        }

        for i in 0..n {
            if !visited[i] {
                dfs(i, &rev_graph, &time, &mut visited, &mut dp);
            }
        }

        *dp.iter().max().unwrap() as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::minimum_time(3, vec![vec![1, 3], vec![2, 3]], vec![3, 2, 5]),
        8
    );
    assert_eq!(
        Solution::minimum_time(
            5,
            vec![vec![1, 5], vec![2, 5], vec![3, 5], vec![3, 4], vec![4, 5]],
            vec![1, 2, 3, 4, 5]
        ),
        12
    );
}
