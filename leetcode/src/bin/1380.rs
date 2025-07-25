impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let row: Vec<i32> = matrix.iter().map(|row| *row.iter().min().unwrap()).collect();
        let col: Vec<i32> = (0..n).map(|j| (0..m).map(|i| matrix[i][j]).max().unwrap()).collect();
        let mut res = vec![];
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == row[i] && matrix[i][j] == col[j] {
                    res.push(matrix[i][j]);
                }
            }
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
        vec![15]
    );
    assert_eq!(
        Solution::lucky_numbers(vec![vec![1, 10, 4, 2], vec![9, 3, 8, 7], vec![15, 16, 17, 12]]),
        vec![12]
    );
    assert_eq!(Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]]), vec![7])
}
