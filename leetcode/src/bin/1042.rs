impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut adj_list = vec![vec![]; n];

        for path in &paths {
            let u = path[0] as usize - 1;
            let v = path[1] as usize - 1;
            adj_list[u].push(v);
            adj_list[v].push(u);
        }

        let mut result = vec![0; n];
        for garden in 0..n {
            let mut used_colors = [false; 5];
            for &neighbor in &adj_list[garden] {
                if result[neighbor] > 0 {
                    used_colors[result[neighbor] as usize] = true;
                }
            }
            for color in 1..=4 {
                if !used_colors[color] {
                    result[garden] = color as i32;
                    break;
                }
            }
        }
        result
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::garden_no_adj(3, vec![vec![1, 2], vec![2, 3], vec![3, 1]]), vec![1, 2, 3]);
}
