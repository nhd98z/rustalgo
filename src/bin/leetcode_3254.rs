impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let l = nums.len();
        let mut acc = 1;
        let mut res = vec![-1; l - k as usize + 1];
        for i in 0..l {
            if i > 0 && nums[i] == nums[i - 1] + 1 {
                acc += 1;
            } else {
                acc = 1;
            }
            if i >= (k - 1) as usize && acc >= k {
                res[i - k as usize + 1] = nums[i];
            }
        }
        res
    }
}

struct Solution;

fn main() {
    dbg!(Solution::results_array(vec![1, 2, 3, 4, 3, 2, 5], 3));
}
