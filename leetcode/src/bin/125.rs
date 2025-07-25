impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();
        let s = s.as_bytes();
        if s.len() == 0 {
            return true;
        }
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
        true
    );
    assert_eq!(Solution::is_palindrome("".to_string()), true);
}
