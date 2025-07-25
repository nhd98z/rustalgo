#![allow(dead_code)]

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let total = 365;
        let mut dp = vec![i32::MAX; total + 1];
        dp[0] = 0;
        for i in 1..=total {
            if days.contains(&(i as i32)) {
                dp[i] = dp[i].min(dp[i - 1] + costs[0]);
                dp[i] = dp[i].min(dp[i.saturating_sub(7)] + costs[1]);
                dp[i] = dp[i].min(dp[i.saturating_sub(30)] + costs[2]);
            } else {
                dp[i] = dp[i - 1];
            }
        }
        dp[total]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]), 11);
}
