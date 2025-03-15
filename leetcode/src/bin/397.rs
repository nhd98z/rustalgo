#![allow(dead_code)]

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        // Last bits: 00 01 10 11
        // 01 should -1, because 01 - 1 -> 00 -> best
        // 11 should +1, because 11 + 1 -> 100 -> best
        // Special case: 3 (11) should -1, because 11 - 1 -> 10 -> 1 -> better than 11 + 1 -> 100 -> 10 -> 1
        let mut n = n as u64;
        let mut count = 0;
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
            } else if n == 3 || (n & 0b11) == 1 {
                n -= 1;
            } else {
                n += 1;
            }
            count += 1;
        }
        count
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::integer_replacement(8), 3);
}
