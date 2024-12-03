#![allow(dead_code, unused)]

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut s = s;
        while let Some(pos) = s.find(&part) {
            s.replace_range(pos..pos + part.len(), "");
        }
        s
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()),
        "dab".to_string()
    );
    assert_eq!(
        Solution::remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string()),
        "ab".to_string()
    );
    assert_eq!(
        Solution::remove_occurrences("aaa".to_string(), "a".to_string()),
        "".to_string()
    );
}
