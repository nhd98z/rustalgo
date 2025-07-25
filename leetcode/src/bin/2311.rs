macro_rules! s {
    ($str:expr) => {
        String::from($str)
    };
}

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut res = 0;
        let mut val = 0;
        for ch in s.chars().rev() {
            match ch {
                '0' => {
                    res += 1;
                }
                '1' => {
                    if res < 30 {
                        let bit_value = 1 << res;
                        if val + bit_value <= k {
                            res += 1;
                            val += bit_value;
                        }
                    }
                }
                _ => {}
            }
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::longest_subsequence(s!("1001010"), 5), 5);
}
