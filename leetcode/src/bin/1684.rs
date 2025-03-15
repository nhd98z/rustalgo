#![allow(dead_code)]

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut count = 0;
        let mut allowed_chars = [false; 26];
        for c in allowed.bytes() {
            allowed_chars[(c - 'a' as u8) as usize] = true;
        }

        for word in words {
            let mut consistent = true;
            for c in word.bytes() {
                if !allowed_chars[(c - 'a' as u8) as usize] {
                    consistent = false;
                    break;
                }
            }
            if consistent {
                count += 1;
            }
        }

        count
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::count_consistent_strings(
            "ab".to_string(),
            vec![
                "ad".to_string(),
                "bd".to_string(),
                "aaab".to_string(),
                "baa".to_string(),
                "badab".to_string()
            ]
        ),
        2
    );
}
