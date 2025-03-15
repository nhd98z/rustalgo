#![allow(dead_code)]

use std::cmp::Ordering::Equal;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }
        let m = nums1.len();
        let n = nums2.len();
        let total_left = (m + n + 1) / 2;
        let mut left = 0;
        let mut right = m;
        while left <= right {
            let i = left + (right - left) / 2;
            let j = total_left - i;
            let nums1_left = if i == 0 { i32::MIN } else { nums1[i - 1] };
            let nums1_right = if i == m { i32::MAX } else { nums1[i] };
            let nums2_left = if j == 0 { i32::MIN } else { nums2[j - 1] };
            let nums2_right = if j == n { i32::MAX } else { nums2[j] };
            match (nums1_left.cmp(&nums2_right), nums2_left.cmp(&nums1_right)) {
                (Less | Equal, Less | Equal) => {
                    let max_left = i32::max(nums1_left, nums2_left) as f64;
                    if (m + n) % 2 == 1 {
                        return max_left;
                    }
                    let min_right = i32::min(nums1_right, nums2_right) as f64;
                    return (max_left + min_right) / 2.0;
                }
                (Greater, _) => {
                    right = i - 1;
                }
                (_, Greater) => {
                    left = i + 1;
                }
            }
        }
        unreachable!()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        2.0
    );
}
