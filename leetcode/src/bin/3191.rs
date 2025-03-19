impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut res = 0;
        let n = nums.len();
        for i in 0..=n - 3 {
            if nums[i] == 0 {
                res += 1;
                nums[i] ^= 1;
                nums[i + 1] ^= 1;
                nums[i + 2] ^= 1;
            }
        }
        if nums[n - 2] == 0 || nums[n - 1] == 0 {
            return -1;
        }
        res
    }
}

struct Solution;

fn main() {
    unsafe {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    assert_eq!(Solution::min_operations(vec![0, 1, 1, 1, 0, 0]), 3);
}
