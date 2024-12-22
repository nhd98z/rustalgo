#![allow(dead_code)]

impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        let mut res = String::new();
        let mut a = a;
        let mut b = b;
        while a > 0 || b > 0 {
            let write_a = if res.ends_with("aa") {
                false
            } else if res.ends_with("bb") {
                true
            } else {
                a >= b
            };
            if write_a {
                res.push('a');
                a -= 1;
            } else {
                res.push('b');
                b -= 1;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::str_without3a3b(1, 2), "bab".to_string());
}
