#![allow(dead_code)]

impl Solution {
    fn find(nums: &Vec<i32>, target: i32, sum: i32, pos: usize) -> i32 {
        let n = nums.len();
        if pos == n {
            if sum == target {
                return 1;
            }
            return 0;
        }
        Self::find(&nums, target, sum - &nums[pos], pos + 1) + Self::find(&nums, target, sum + &nums[pos], pos + 1)
    }    
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        Self::find(&nums, target, 0, 0)
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
}
