#![allow(dead_code)]

fn valid(nums: &Vec<i32>, k: i32, target: i32) -> bool {
    let mut cnt = 1;
    let mut sum = 0;
    let mut i = 0;
    while i < nums.len() {
        if sum + nums[i] > target {
            cnt += 1;
            sum = nums[i];
            if cnt > k {
                return false;
            }
        } else {
            sum += nums[i];
        }
        i += 1;
    }
    cnt <= k
}

impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = *nums.iter().max().unwrap();
        let mut r: i32 = nums.iter().sum();
        while l < r {
            let mid = l + (r - l) / 2;
            if valid(&nums, k, mid) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::split_array(vec![7, 2, 5, 10, 8], 2), 18);
    assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5], 2), 9)
}
