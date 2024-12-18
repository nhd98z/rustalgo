impl Solution {
    #[allow(dead_code)]
    pub fn min_cost_climbing_stairs_ugly(cost: Vec<i32>) -> i32 {
        if cost.len() == 2 {
            return i32::min(cost[0], cost[1]);
        }
        let mut a = cost[0];
        let mut b = cost[1];
        for i in 2..cost.len() + 1 {
            let value = if i == cost.len() { 0 } else { cost[i] };
            let temp = b;
            b = i32::min(b, a) + value;
            a = temp;
        }
        b
    }

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![0; n + 1];
        for i in 2..=n {
            dp[i] = (dp[i - 1] + cost[i - 1]).min(dp[i - 2] + cost[i - 2]);
        }
        dp[n]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::min_cost_climbing_stairs(vec![10, 15, 20]), 15);
    assert_eq!(
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]),
        6
    );
}
