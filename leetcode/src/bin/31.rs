impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut i = nums.len() - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        if i == 0 {
            nums.reverse();
        } else {
            let mut j = nums.len() - 1;
            while nums[j] <= nums[i - 1] {
                j -= 1;
            }
            nums.swap(i - 1, j);
            nums[i..].reverse();
        }
    }
}

struct Solution;

fn main() {
    let mut nums = vec![1, 2, 3];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 3, 2]);

    let mut nums = vec![1, 3, 2, 5, 4];
    Solution::next_permutation(&mut nums);
    assert_eq!(nums, vec![1, 3, 4, 2, 5]);
}
