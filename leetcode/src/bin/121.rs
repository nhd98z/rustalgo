#![allow(dead_code)]

use std::*;

impl Solution {
    pub fn max_profit(a: Vec<i32>) -> i32 {
        let mut emin = a[0];
        let mut emax = 0;
        let mut res = 0;
        let n = a.len();
        for i in 1..n {
            if a[i] < emin {
                emin = a[i];
                emax = 0;
                if emax != 0 {
                    res = cmp::max(res, emax - emin);
                }
            } else if a[i] > emax {
                emax = a[i];
                res = cmp::max(res, emax - emin);
            }
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
}
