use std::cmp::{max, min};

impl Solution {
    pub fn max_area(a: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = a.len() - 1;
        let mut res = 0;

        while l < r {
            let area = (r - l) as i32 * min(a[l], a[r]);
            res = max(res, area);
            if a[l] < a[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        res
    }
}

struct Solution;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    assert_eq!(Solution::max_area(vec![3, 6, 1]), 3);
}
