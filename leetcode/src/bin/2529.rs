impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;
        for num in nums {
            if num < 0 {
                l += 1;
            } else if num > 0 {
                r += 1;
            }
        }
        i32::max(l, r)
    }
}

struct Solution;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    assert_eq!(Solution::maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
}
