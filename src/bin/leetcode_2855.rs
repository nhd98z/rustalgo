impl Solution {
    pub fn minimum_right_shifts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut pivot = 0;
        while pivot < n - 1 && nums[pivot] < nums[pivot + 1] {
            pivot += 1;
        }
        if pivot == n - 1 {
            return 0;
        }
        pivot += 1;
        for i in pivot..n - 1 {
            if nums[i] > nums[i + 1] {
                return -1;
            }
        }
        if nums[n - 1] > nums[0] {
            return -1;
        }
        (n - pivot) as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::minimum_right_shifts(vec![3, 4, 5, 1, 2]), 2);
    assert_eq!(Solution::minimum_right_shifts(vec![1, 3, 5]), 0);
    assert_eq!(Solution::minimum_right_shifts(vec![2, 1, 4]), -1);
}
