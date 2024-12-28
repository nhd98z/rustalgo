#![allow(dead_code)]

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = std::collections::HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if map[&c] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
}
