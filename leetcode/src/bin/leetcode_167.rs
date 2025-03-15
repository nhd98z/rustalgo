impl Solution {
    pub fn two_sum(a: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = a.len() - 1;
        while l < r {
            let sum = a[l] + a[r];
            if sum == target {
                return vec![l as i32 + 1, r as i32 + 1];
            } else if sum < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
        unreachable!();
    }
}


struct Solution;

fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1"); }
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
}
