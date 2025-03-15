macro_rules! s {
    ($str:expr) => {
        String::from($str)
    };
}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        s.trim().split_whitespace().last().unwrap_or("").len() as _
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::length_of_last_word(s!("Hello World")), 5);
    assert_eq!(Solution::length_of_last_word(s!("   fly me   to   the moon  ")), 4);
    assert_eq!(Solution::length_of_last_word(s!("luffy is still joyboy")), 6);
    assert_eq!(Solution::length_of_last_word(s!("a")), 1);
    assert_eq!(Solution::length_of_last_word(s!("Programming")), 11);
    assert_eq!(Solution::length_of_last_word(s!("   spaces   ")), 6);
    assert_eq!(Solution::length_of_last_word(s!("Hello World  ")), 5);
    assert_eq!(Solution::length_of_last_word(s!("  Hello")), 5);
    assert_eq!(Solution::length_of_last_word(s!("Rust Programming Language")), 8);
    assert_eq!(Solution::length_of_last_word(s!("a b c d e")), 1);
    assert_eq!(Solution::length_of_last_word(s!("   ")), 0);
}
