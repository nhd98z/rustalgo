impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut res = 0;
        let mut left = 0;
        let mut count = [0; 3];
        let s = s.chars().collect::<Vec<_>>();
        for right in 0..s.len() {
            count[s[right] as usize - 'a' as usize] += 1;
            while count[0] > 0 && count[1] > 0 && count[2] > 0 {
                res += s.len() - right;
                count[s[left] as usize - 'a' as usize] -= 1;
                left += 1;
            }
        }
        res as _
    }
}

struct Solution;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    assert_eq!(Solution::number_of_substrings("abcabc".to_string()), 10);
}
