impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let a = text1.as_bytes();
        let b = text2.as_bytes();
        let mut dp = vec![0; b.len() + 1];
        for i in 0..a.len() {
            let mut curr = 0;
            for j in 0..b.len() {
                let prev = curr;
                curr = dp[j + 1];
                dp[j + 1] = if a[i] == b[j] {
                    prev + 1
                } else {
                    i32::max(dp[j], dp[j + 1])
                };
            }
        }
        dp[b.len()]
    }
    // Another approach that can apply to LCIS problem.
    pub fn longest_common_subsequence_2(text1: String, text2: String) -> i32 {
        let a = text1.as_bytes();
        let b = text2.as_bytes();
        let mut dp = vec![0; b.len()];
        for i in 0..a.len() {
            let mut curr = 0;
            for j in 0..b.len() {
                let prev = curr;
                curr = usize::max(curr, dp[j]);
                if a[i] == b[j] {
                    dp[j] = usize::max(dp[j], prev + 1);
                }
            }
        }
        dp.into_iter().max().unwrap() as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()), 3);
    assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()), 3);
    assert_eq!(Solution::longest_common_subsequence("abc".to_string(), "def".to_string()), 0);
    assert_eq!(
        Solution::longest_common_subsequence("abcba".to_string(), "abcbcba".to_string()),
        5
    );
    assert_eq!(Solution::longest_common_subsequence("bsbininm".to_string(), "jmjkbkjkv".to_string()), 1);


    assert_eq!(Solution::longest_common_subsequence_2("abcde".to_string(), "ace".to_string()), 3);
    assert_eq!(Solution::longest_common_subsequence_2("abc".to_string(), "abc".to_string()), 3);
    assert_eq!(Solution::longest_common_subsequence_2("abc".to_string(), "def".to_string()), 0);
    assert_eq!(
        Solution::longest_common_subsequence_2("abcba".to_string(), "abcbcba".to_string()),
        5
    );
    assert_eq!(Solution::longest_common_subsequence_2("bsbininm".to_string(), "jmjkbkjkv".to_string()), 1);
}
