#![allow(dead_code)]

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut res = nums[0] + nums[1] + nums[2];
        for i in 0..n {
            let mut l = i + 1;
            let mut r = n - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if (sum - target).abs() < (res - target).abs() {
                    res = sum;
                }
                if sum < target {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
}
