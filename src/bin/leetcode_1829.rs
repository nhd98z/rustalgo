impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let n = nums.len();
        let mask = (1 << maximum_bit) - 1;
        let mut res = vec![0; n];
        let mut curr = 0;
        for i in 0..n {
            curr ^= nums[i];
            res[n - i - 1] = curr ^ mask;
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::get_maximum_xor(vec![0, 1, 1, 3], 2), [0, 3, 2, 3]);
    assert_eq!(Solution::get_maximum_xor(vec![2, 3, 4, 7], 3), [5, 2, 6, 5]);
}
