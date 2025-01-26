#![allow(dead_code)]

impl Solution {
    pub fn max_profit(a: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = a.len();
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n && a[j] >= a[j - 1] {
                j += 1;
            }
            res += a[j - 1] - a[i];
            i = j;
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
}
