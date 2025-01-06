#![allow(dead_code)]

impl Solution {
    pub fn maximum_xor(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc | x)
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::maximum_xor(vec![3, 2, 4, 6]), 7);
    assert_eq!(Solution::maximum_xor(vec![640]), 640);
}
