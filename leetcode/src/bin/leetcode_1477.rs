#![allow(dead_code)]

use std::collections::HashMap;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
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

        if res == i32::MAX { -1 } else { res }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
}
