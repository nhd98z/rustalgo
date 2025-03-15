#![allow(dead_code)]

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;
        let mut i = 0;
        let mut j = 0;
        let mut idx = 0;
        let mut res = vec![0; m + n];
        while i < m || j < n {
            if i == m {
                for k in idx..m + n {
                    res[k] = nums2[j];
                    j += 1;
                }
                break;
            }
            if j == n {
                for k in idx..m + n {
                    res[k] = nums1[i];
                    i += 1;
                }
                break;
            }
            if nums1[i] < nums2[j] {
                res[idx] = nums1[i];
                idx += 1;
                i += 1;
            } else {
                res[idx] = nums2[j];
                idx += 1;
                j += 1;
            }
        }
        *nums1 = res;
    }
}

struct Solution;

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    Solution::merge(&mut nums1, 3, &mut vec![2, 5, 6], 3);
    println!("nums1 = {:?}", &nums1);
}
