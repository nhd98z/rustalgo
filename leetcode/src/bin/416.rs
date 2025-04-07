impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let nums: Vec<usize> = nums.iter().map(|&num| num as usize).collect();
        let sum: usize = nums.iter().sum();
        if sum % 2 == 1 {
            return false;
        }
        let target = sum / 2;
        let mut dp = vec![false; target + 1];
        dp[0] = true; // dp[target] = true means we can split nums into to 2 subsets with each sum = target
        for num in nums {
            if num > target {
                return false;
            }
            for j in (0..=target - num).rev() {
                if dp[j] {
                    dp[j + num] = true;
                }
            }
            if dp[target] {
                return true;
            }
        }
        dp[target]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
    assert_eq!(Solution::can_partition(vec![1, 2, 3, 5]), false);
    assert_eq!(Solution::can_partition(vec![1, 1, 2, 2]), true);
}
