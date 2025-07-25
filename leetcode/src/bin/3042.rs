impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        fn is_match(s1: &str, s2: &str) -> i32 {
            if s2.starts_with(s1) && s2.ends_with(s1) { 1 } else { 0 }
        }
        words.iter().enumerate().fold(0, |r, (idx, s1)| {
            r + &words[idx + 1..].iter().fold(0, |r, s2| r + is_match(s1, s2))
        })
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::count_prefix_suffix_pairs(vec![
            "a".to_string(),
            "aba".to_string(),
            "ababa".to_string(),
            "aa".to_string()
        ]),
        4
    );
}
