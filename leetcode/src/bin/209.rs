use std::{cmp::min, usize};

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut curr_sum = 0;
        let mut res = usize::MAX;
        let mut left = 0;
        for right in 0..n {
            curr_sum += nums[right];
            while curr_sum >= target {
                res = min(res, right - left + 1);
                curr_sum -= nums[left];
                left += 1;
            }
        }
        if res == usize::MAX {
            return 0;
        } else {
            return res as _;
        }
    }
}

struct Solution;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
}
