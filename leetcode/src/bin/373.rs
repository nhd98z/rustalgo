impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = 0;
        for bit in 0..32 {
            let mut count_ones = 0;
            for &num in &nums {
                count_ones += (num >> bit) & 1;
            }
            sum += count_ones * (n as i32 - count_ones);
        }
        sum
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::total_hamming_distance(vec![4, 14, 2]), 6);
    assert_eq!(Solution::total_hamming_distance(vec![4, 14, 4]), 4);
}
