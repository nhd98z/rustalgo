impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut nums = nums;
        let mut less = Vec::new();
        let mut equal = Vec::new();
        let mut greater = Vec::new();
        for &num in nums.iter() {
            if num < pivot {
                less.push(num);
            } else if num == pivot {
                equal.push(num);
            } else {
                greater.push(num);
            }
        }
        let mut i = 0;
        for num in less {
            nums[i] = num;
            i += 1;
        }
        for num in equal {
            nums[i] = num;
            i += 1;
        }
        for num in greater {
            nums[i] = num;
            i += 1;
        }
        nums
    }
}

struct Solution;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    // Single element
    assert_eq!(Solution::pivot_array(vec![1], 1), vec![1]);

    // All elements equal to pivot
    assert_eq!(Solution::pivot_array(vec![5, 5, 5, 5], 5), vec![5, 5, 5, 5]);

    // No elements equal to pivot
    assert_eq!(Solution::pivot_array(vec![1, 3, 7, 9], 5), vec![1, 3, 7, 9]);

    // All elements less than pivot
    assert_eq!(Solution::pivot_array(vec![1, 2, 3], 4), vec![1, 2, 3]);

    // All elements greater than pivot
    assert_eq!(Solution::pivot_array(vec![5, 6, 7], 4), vec![5, 6, 7]);

    // Negative numbers
    assert_eq!(
        Solution::pivot_array(vec![-5, -3, -1, -2, -4], -3),
        vec![-5, -4, -3, -1, -2]
    );

    // Mix of positive and negative
    assert_eq!(
        Solution::pivot_array(vec![-2, 0, 2, -1, 0, 1], 0),
        vec![-2, -1, 0, 0, 2, 1]
    );

    // Multiple duplicates including pivot
    assert_eq!(
        Solution::pivot_array(vec![2, 2, 1, 3, 2, 1, 2], 2),
        vec![1, 1, 2, 2, 2, 2, 3]
    );

    // Large range of numbers
    assert_eq!(
        Solution::pivot_array(vec![1000, -1000, 0, 1000, -1000], 0),
        vec![-1000, -1000, 0, 1000, 1000]
    );
}
