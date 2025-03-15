#![allow(dead_code)]

impl Solution {
    pub fn find_target_sum_ways_stupid(nums: Vec<i32>, target: i32) -> i32 {
        fn find(nums: &Vec<i32>, target: i32, sum: i32, pos: usize) -> i32 {
            let n = nums.len();
            if pos == n {
                if sum == target {
                    return 1;
                }
                return 0;
            }
            find(&nums, target, sum - &nums[pos], pos + 1)
                + find(&nums, target, sum + &nums[pos], pos + 1)
        }
        find(&nums, target, 0, 0)
    }

    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let t_sum = nums.iter().sum::<i32>();
        if t_sum < target || -t_sum > target || (t_sum & 1) ^ (target & 1) == 1 {
            0
        } else {
            let limit = ((t_sum + target) >> 1) as usize;
            let mut dp = vec![0; limit + 1];
            dp[0] = 1;
            for i in 1..=nums.len() {
                for j in (0..=limit).rev() {
                    dp[j] += if j >= nums[i - 1] as usize {
                        dp[j - nums[i - 1] as usize]
                    } else {
                        0
                    }
                }
            }
            dp[limit]
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
}
