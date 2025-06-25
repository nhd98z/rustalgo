#![allow(dead_code)]

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut balance = 0;
        let mut max_balance = 0;

        for c in s.as_bytes() {
            if c == &b'[' {
                balance += 1;
            } else {
                balance -= 1;
            }
            max_balance = max_balance.max(-balance);
        }

        (max_balance + 1) >> 1
    }

    pub fn min_swaps_stupid(s: String) -> i32 {
        let mut s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut st = vec![];
        let mut cnt = 0;
        for i in 0..n {
            if s[i] == '[' {
                st.push(s[i]);
            } else if st.is_empty() || st[st.len() - 1] == ']' {
                let temp = s[i];
                s[i] = s[n - cnt - 1];
                s[n - cnt - 1] = temp;
                cnt += 1;
                st.push('[');
            } else {
                st.pop();
            }
        }
        cnt as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::min_swaps("][][".to_string()), 1);
    assert_eq!(Solution::min_swaps("]]][[[".to_string()), 2);
    assert_eq!(Solution::min_swaps("[[[]]]][][]][[]]][[[".to_string()), 2);
}
