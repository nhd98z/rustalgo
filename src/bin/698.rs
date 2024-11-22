#![allow(dead_code, unused)]

use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }
        let target = sum / k;
        if nums.iter().any(|&num| num > target) {
            return false;
        }
        let mut nums = nums;
        nums.sort();
        nums.reverse();
        let mut subsets = vec![0; k as usize];
        Self::dfs(&nums, &mut subsets, 0, target)
    }

    fn dfs(nums: &Vec<i32>, subsets: &mut Vec<i32>, pos: i32, target: i32) -> bool {
        if pos == nums.len() as i32 {
            return subsets.iter().all(|&sum| sum == target);
        }
        for i in 0..subsets.len() {
            if subsets[i] + nums[pos as usize] > target {
                continue;
            }
            if subsets[..i].iter().any(|&sum| sum == subsets[i]) {
                continue;
            }
            subsets[i] += nums[pos as usize];
            if Self::dfs(&nums, subsets, pos + 1, target) {
                return true;
            }
            subsets[i] -= nums[pos as usize];
        }
        false
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4),
        true
    );

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for i in 0..1000 {
        let n = rand::thread_rng().gen_range(1..=20);
        let k = rand::thread_rng().gen_range(1..=n);

        let mut nums = vec![];
        for i in 0..n {
            nums.push(rand::thread_rng().gen_range(1..=10_000));
        }
        dbg!(nums.len(), k);
        dbg!(Solution::can_partition_k_subsets(nums, k));
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    dbg!(end - start);
}
