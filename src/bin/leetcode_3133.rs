impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let x = x as i64;
        let mut res = x as i64;
        let mut n = (n - 1) as i64;
        let mut p = 1i64;

        while n != 0 {
            if x & p == 0 {
                res |= (n & 1) * p;
                n >>= 1;
            }
            p <<= 1;
        }

        res
    }
}

struct Solution;

fn main() {
    dbg!(Solution::min_end(2, 7));
}
