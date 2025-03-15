impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::reverse_words("the sky is blue".to_string()), "blue is sky the");
}
