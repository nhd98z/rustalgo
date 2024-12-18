impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = vec![0usize; n];
        sum[0] = nums[0] as usize;
        for i in 1..n {
            sum[i] = sum[i - 1] + nums[i] as usize;
        }
        for i in 0..n {
            let left = if i == 0 { 0 } else { sum[i - 1] };
            let right = if i == n - 1 { 0 } else { sum[n - 1] - sum[i] };
            if left == right {
                return i as i32;
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
}
