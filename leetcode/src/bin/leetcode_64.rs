impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut f = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let mut min = i32::MAX;
                if i >= 1 {
                    min = f[i - 1][j];
                }
                if j >= 1 {
                    min = i32::min(min, f[i][j - 1]);
                }
                if min == i32::MAX {
                    f[i][j] = grid[i][j];
                } else {
                    f[i][j] = min + grid[i][j];
                }
            }
        }
        f[m - 1][n - 1]
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
        7
    );
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        12
    );
}
