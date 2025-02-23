macro_rules! s {
    ($str:expr) => {
        String::from($str)
    };
}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 {
            return true;
        }
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = 0;
        let mut j = 0;
        while i < t.len() {
            if t[i] == s[j] {
                i += 1;
                j += 1;
            } else {
                i += 1;
            }
            if j >= s.len() {
                return true;
            }
        }
        false
    }
}

struct Solution;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    assert_eq!(Solution::is_subsequence(s!("abc"), s!("ahbgdc")), true);
    assert_eq!(Solution::is_subsequence(s!("axc"), s!("ahbgdc")), false);
    assert_eq!(Solution::is_subsequence(s!(""), s!("ahbgdc")), true);     // empty string is always a subsequence
    assert_eq!(Solution::is_subsequence(s!("abc"), s!("")), false);       // non-empty string can't be a subsequence of empty string
    assert_eq!(Solution::is_subsequence(s!(""), s!("")), true);           // empty string is a subsequence of empty string
    assert_eq!(Solution::is_subsequence(s!("ace"), s!("abcde")), true);   // same characters
    assert_eq!(Solution::is_subsequence(s!("aec"), s!("abcde")), false);  // order matters
    assert_eq!(Solution::is_subsequence(s!("aaa"), s!("aaa")), true);     // same characters
    assert_eq!(Solution::is_subsequence(s!("aaa"), s!("aaaa")), true);    // repeated characters
    assert_eq!(Solution::is_subsequence(s!("bb"), s!("abc")), false);     // order matters
    assert_eq!(Solution::is_subsequence(s!("xyz"), s!("abcdef")), false); // no common characters
}
