impl Solution {
    pub fn length_of_lis_stupid(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut dp = vec![1; n];
        let mut res = 1;
        dp[0] = 1;
        for i in 1..n {
            for j in 0..i {
                if a[j] < a[i] {
                    dp[i] = i32::max(dp[i], dp[j] + 1);
                    res = i32::max(res, dp[i]);
                }
            }
        }
        res
    }

    pub fn length_of_lis(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut dp = vec![];
        for i in 0..n {
            let pos = dp.binary_search(&a[i]).unwrap_or_else(|i| i);
            if pos == dp.len() {
                dp.push(a[i]);
            } else {
                dp[pos] = a[i];
            }
        }
        dp.len() as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::length_of_lis_stupid(vec![10, 9, 2, 5, 3, 7, 101, 18]),
        4
    );
    assert_eq!(Solution::length_of_lis_stupid(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(Solution::length_of_lis_stupid(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    assert_eq!(
        Solution::length_of_lis_stupid(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]),
        6
    );

    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    assert_eq!(Solution::length_of_lis(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
}
