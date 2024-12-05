#![allow(dead_code, unused)]

use std::collections::HashMap;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let mut stack = vec![(0, 0, 0, 0)]; // pos, mask, open, close
        while let Some((pos, mask, open, close)) = stack.pop() {
            if pos == n << 1 {
                let mut seq = String::new();
                for i in 0..n << 1 {
                    if mask & (1 << i) == 0 {
                        seq.push('(');
                    } else {
                        seq.push(')');
                    }
                }
                res.push(seq);
                continue;
            }
            if open < n {
                stack.push((pos + 1, mask, open + 1, close));
            }
            if close < open {
                stack.push((pos + 1, mask | (1 << pos), open, close + 1));
            }
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec!("()()()", "()(())", "(())()", "(()())", "((()))")
    );
    assert_eq!(Solution::generate_parenthesis(1), vec!("()"));
}
