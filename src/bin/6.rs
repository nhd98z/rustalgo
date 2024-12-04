#![allow(dead_code, unused)]

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut a = vec!["".to_string(); num_rows as usize];
        let mut i = 0;
        let s = s.as_bytes();
        while i < s.len() {
            for j in 0..num_rows {
                if i < s.len() {
                    a[j as usize].push(s[i] as char);
                    i += 1;
                }
            }
            for j in (1..=num_rows - 2).rev() {
                if i < s.len() {
                    a[j as usize].push(s[i] as char);
                    i += 1;
                }
            }
        }
        a.join("")
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR".to_string()
    );
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI".to_string()
    );
}
