#![allow(dead_code)]

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut left_sum: i64 = 0;
        let mut right_sum: i64 = nums.iter().map(|&n| n as i64).sum();
        let mut count = 0;
        for i in 0..nums.len() - 1 {
            left_sum += nums[i] as i64;
            right_sum -= nums[i] as i64;
            if left_sum >= right_sum {
                count += 1;
            }
        }
        count
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::ways_to_split_array(vec![10, 4, -8, 7]), 2);
}
