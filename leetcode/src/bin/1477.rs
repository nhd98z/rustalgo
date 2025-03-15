#![allow(dead_code)]

use std::collections::HashMap;

impl Solution {
    pub fn min_sum_of_lengths_stupid(arr: Vec<i32>, target: i32) -> i32 {
        let (mut sum, mut res, mut lsize) = (0, i32::MAX, i32::MAX);
        let mut prefix_sum: HashMap<i32, i32> = HashMap::new();

        prefix_sum.insert(0, -1);

        for (i, val) in arr.iter().enumerate() {
            sum += *val;
            prefix_sum.insert(sum, i as i32);
        }

        sum = 0;
        for (i, val) in arr.iter().enumerate() {
            sum += *val;
            let index = i as i32;

            if prefix_sum.contains_key(&(sum - target)) {
                lsize = lsize.min(index - *prefix_sum.get(&(sum - target)).unwrap());
            }

            if prefix_sum.contains_key(&(sum + target)) && lsize != i32::MAX {
                let rsize = *prefix_sum.get(&(sum + target)).unwrap() - index;
                res = res.min(rsize + lsize);
            }
        }

        if res == i32::MAX {
            -1
        } else {
            res
        }
    }

    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        use std::cmp::min;
        let inf = 100001;
        let mut f: Vec<i32> = Vec::with_capacity(arr.len());
        f.resize(arr.len(), inf);

        let mut idx = 0;
        let mut current_sum = 0;
        for i in 0..arr.len() {
            if i == 0 {
                current_sum = arr[i];
            } else {
                current_sum += arr[i];
                while idx < i && current_sum > target {
                    current_sum -= arr[idx];
                    idx += 1;
                }
            }
            if i > 0 {
                f[i] = f[i - 1];
            }
            if current_sum == target {
                if i == 0 {
                    f[i] = 1;
                } else {
                    f[i] = min(f[i], i as i32 - idx as i32 + 1);
                }
            }
        }

        let mut result = inf;
        idx = arr.len() - 1;
        current_sum = 0;
        let mut best_right = inf;
        for ii in 0..arr.len() - 1 {
            let i = arr.len() - 1 - ii;
            if i == arr.len() - 1 {
                current_sum = arr[i];
            } else {
                current_sum += arr[i];
                while idx > i && current_sum > target {
                    current_sum -= arr[idx];
                    idx -= 1;
                }
            }
            if current_sum == target {
                best_right = min(best_right, idx as i32 - i as i32 + 1);
            }
            result = min(result, best_right + f[i - 1]);
        }
        if result != inf {
            result
        } else {
            -1
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
}
