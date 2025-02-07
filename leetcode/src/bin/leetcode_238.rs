impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![1; nums.len()];
        let mut left = 1;
        let mut right = 1;
        for i in 0..nums.len() {
            result[i] = left;
            left *= nums[i];
        }
        for i in (0..nums.len()).rev() {
            result[i] *= right;
            right *= nums[i];
        }
        result
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
}
