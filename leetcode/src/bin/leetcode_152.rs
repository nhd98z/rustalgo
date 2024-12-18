impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut cmin = nums[0];
        let mut cmax = nums[0];
        for i in 1..nums.len() {
            let candidates = [nums[i], cmin * nums[i], cmax * nums[i]];
            cmin = candidates.into_iter().min().unwrap();
            cmax = candidates.into_iter().max().unwrap();
            res = i32::max(res, cmax);
        }
        res
    }
}

struct Solution;

fn main() {
    dbg!(Solution::max_product(vec![2, 3, -2, 4]));
    dbg!(Solution::max_product(vec![-2, 0, -1]));
    dbg!(Solution::max_product(vec![-3, -1, -1]));
    dbg!(Solution::max_product(vec![0, 2]));
}
