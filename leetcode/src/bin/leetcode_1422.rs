#![allow(dead_code)]

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut zeros = vec![0; n];
        let mut ones = vec![0; n];
        zeros[0] = if s[0] == b'0' { 1 } else { 0 };
        ones[n - 1] = if s[n - 1] == b'1' { 1 } else { 0 };
        for i in 1..n {
            zeros[i] = zeros[i - 1] + if s[i] == b'0' { 1 } else { 0 };
        }
        for i in (0..n - 1).rev() {
            ones[i] = ones[i + 1] + if s[i] == b'1' { 1 } else { 0 };
        }
        let mut max = 0;
        for i in 0..n - 1 {
            max = max.max(zeros[i] + ones[i + 1]);
        }
        max
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::max_score("011101".to_string()), 5);
}
