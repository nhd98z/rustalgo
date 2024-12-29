#![allow(dead_code)]

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        2 * (n / 2 + 1 - Solution::last_remaining(n / 2))
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::last_remaining(9), 6);
    assert_eq!(Solution::last_remaining(1), 1);
}
