use std::collections::HashSet;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut rows = HashSet::new();
        let mut cols = HashSet::new();

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    rows.insert(i);
                    cols.insert(j);
                }
            }
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if rows.contains(&i) || cols.contains(&j) {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

struct Solution;

fn main() {
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
}
