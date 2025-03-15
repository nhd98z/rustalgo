use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn three_sum(a: Vec<i32>) -> Vec<Vec<i32>> {
        let n = a.len();
        let mut res = vec![];
        let mut a = a;
        a.sort();
        for i in 0..n - 2 {
            if i > 0 && a[i] == a[i - 1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = n - 1;
            while l < r {
                let sum = a[i] + a[l] + a[r];
                match sum.cmp(&0) {
                    Equal => {
                        res.push(vec![a[i], a[l], a[r]]);
                        while l < r && a[l] == a[l + 1] {
                            l += 1;
                        }
                        while l < r && a[r] == a[r - 1] {
                            r -= 1;
                        }
                        l += 1;
                        r -= 1;
                    }
                    Less => l += 1,
                    Greater => r -= 1,
                }
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
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
}
