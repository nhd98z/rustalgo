#![allow(dead_code)]

use std::cmp::max;

impl Solution {
    pub fn jump(a: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut curr_e = 0;
        let mut farthest = 0;
        for i in 0..a.len() - 1 {
            farthest = max(farthest, i + a[i] as usize);
            if i == curr_e {
                ans += 1;
                curr_e = farthest;
            }
        }
        ans
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    assert_eq!(Solution::jump(vec![1, 2, 1, 1, 1]), 3);
    assert_eq!(
        Solution::jump(vec![5, 6, 4, 4, 6, 9, 4, 4, 7, 4, 4, 8, 2, 6, 8]),
        2
    );
}
