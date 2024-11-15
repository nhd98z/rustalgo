use std::{cmp::max, collections::HashMap};

#[allow(dead_code)]

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut m: HashMap<char, i32> = HashMap::new();
        let mut res = 0;
        let mut from = 0;
        for (i, c) in s.chars().enumerate() {
            let index = *m.get(&c).unwrap_or(&-1);
            // dbg!(i, c, from);
            if index == -1 {
                res = max(res, i - from + 1);
                m.insert(c, i as i32);
            } else {
                from = max(from, index as usize + 1);
                res = max(res, i - from + 1);
                m.insert(c, i as i32);
            }
        }
        res as i32
    }
}

struct Solution;

fn main() {
    dbg!(Solution::length_of_longest_substring(
        "abcabcbb".to_string()
    ));
    dbg!(Solution::length_of_longest_substring("bbbbb".to_string()));
    dbg!(Solution::length_of_longest_substring("pwwkew".to_string()));
    dbg!(Solution::length_of_longest_substring("aab".to_string()));
    dbg!(Solution::length_of_longest_substring("dvdf".to_string()));
    dbg!(Solution::length_of_longest_substring("tmmzuxt".to_string()));
}
