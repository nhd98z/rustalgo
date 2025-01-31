#![allow(dead_code)]

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut cur = 1;
        let mut k = k - 1;
        while k > 0 {
            let steps = Self::cal_steps(n, cur, cur + 1);
            if steps <= k {
                cur += 1;
                k -= steps;
            } else {
                cur *= 10;
                k -= 1;
            }
        }
        cur
    }

    fn cal_steps(n: i32, n1: i32, n2: i32) -> i32 {
        let mut steps = 0;
        let n = n as i64;
        let mut n1 = n1 as i64;
        let mut n2 = n2 as i64;
        while n1 <= n {
            steps += ((n + 1).min(n2) - n1) as i32;
            n1 *= 10;
            n2 *= 10;
        }
        steps
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_kth_number(13, 2), 10);
}
