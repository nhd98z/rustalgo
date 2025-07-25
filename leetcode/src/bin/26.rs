#![allow(dead_code)]

use std::collections::BTreeSet;

impl Solution {
    pub fn remove_duplicates(a: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        let mut res = BTreeSet::new();
        while i < a.len() {
            res.insert(a[i]);
            i += 1;
        }
        *a = res.into_iter().collect::<Vec<_>>();
        a.len() as _
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 2]), 2);
}
