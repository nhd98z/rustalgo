#![allow(dead_code)]

use std::cmp::{max, min};

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut max_left = vec![0; n];
        max_left[0] = height[0];
        for i in 1..n {
            max_left[i] = max(max_left[i - 1], height[i]);
        }
        let mut max_right = vec![0; n];
        max_right[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            max_right[i] = max(max_right[i + 1], height[i]);
        }
        let mut res = 0;
        for i in 1..n - 1 {
            res += max(0, min(max_left[i], max_right[i]) - height[i]);
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}
