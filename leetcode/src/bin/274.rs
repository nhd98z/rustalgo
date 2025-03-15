#![allow(dead_code)]

use std::cmp::{max, min};

impl Solution {
    pub fn h_index(a: Vec<i32>) -> i32 {
        let n = a.len();
        let mut a = a;
        a.sort_unstable();
        let mut res = 0;
        for i in 0..n {
            res = max(res, min(a[i], (n - i) as _));
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    assert_eq!(Solution::h_index(vec![100]), 1);
    assert_eq!(Solution::h_index(vec![100, 200]), 2);
    assert_eq!(Solution::h_index(vec![0, 0, 2]), 1);
}
