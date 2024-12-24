#![allow(dead_code)]

use std::iter;

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let ones = s.as_bytes().iter().filter(|x| **x == b'1').count() - 1;
        iter::repeat('1')
            .take(ones)
            .chain(iter::repeat('0').take(s.len() - ones - 1))
            .chain(iter::once('1'))
            .collect()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::maximum_odd_binary_number("010".to_string()),
        "001".to_string()
    );
}
