impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut rev_graph = vec![vec![]; n];
        for relation in relations {
            let (prev, next) = (relation[0] as usize - 1, relation[1] as usize - 1);
            rev_graph[next].push(prev);
        }

        let mut memo = vec![0; n];
        let mut max_time = 0;
        for i in 0..n {
            // Đồ thị có nhiều thành phần liên thông, nên cần tính toán từng thành phần liên thông
            max_time = max_time.max(Self::dfs(i, &rev_graph, &time, &mut memo));
        }

        max_time as i32
    }

    fn dfs(node: usize, rev_graph: &Vec<Vec<usize>>, time: &Vec<i32>, memo: &mut Vec<i32>) -> i32 {
        if memo[node] > 0 {
            return memo[node];
        }

        let mut max_prereq_time = 0; // Thời gian lớn nhất của các đỉnh trước đó
        for &prereq in &rev_graph[node] {
            max_prereq_time = max_prereq_time.max(Self::dfs(prereq, rev_graph, time, memo));
        }

        memo[node] = time[node] + max_prereq_time;
        memo[node]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::minimum_time(3, vec![vec![1, 3], vec![2, 3]], vec![3, 2, 5]), 8);
}
