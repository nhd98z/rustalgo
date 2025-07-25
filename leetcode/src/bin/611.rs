#![allow(dead_code)]

impl Solution {
    pub fn triangle_number_stupid(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        if n <= 2 {
            return 0;
        }
        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                let a = nums[i];
                let b = nums[j];
                if a == 0 || b == 0 {
                    continue;
                }
                let start = nums[j + 1..].binary_search_by(|&x| {
                    if x < (a - b).abs() + 1 {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    }
                });
                if let Ok(start) = start {
                    if nums[start] < a + b {
                        let end = nums[start + j + 1..]
                            .binary_search_by(|&x| {
                                if x < a + b {
                                    std::cmp::Ordering::Less
                                } else {
                                    std::cmp::Ordering::Greater
                                }
                            })
                            .unwrap_or_else(|x| x);
                        ans += end - start + 1;
                    }
                } else if let Err(start) = start {
                    if start + j + 1 < n && nums[start + j + 1] < a + b {
                        let end = nums[start + j + 1..]
                            .binary_search_by(|&x| {
                                if x < a + b {
                                    std::cmp::Ordering::Less
                                } else {
                                    std::cmp::Ordering::Greater
                                }
                            })
                            .unwrap_or_else(|x| x);
                        ans += end - start;
                    }
                }
            }
        }
        ans as _
    }

    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans = 0;
        let n = nums.len();
        for k in (2..=n - 1).rev() {
            let mut i = 0;
            let mut j = k - 1;
            while i < j {
                if nums[i] + nums[j] > nums[k] {
                    ans += j - i;
                    j -= 1;
                } else {
                    i += 1;
                }
            }
        }
        ans as _
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
    assert_eq!(Solution::triangle_number(vec![4, 2, 3, 4]), 4);
    assert_eq!(Solution::triangle_number(vec![1]), 0);
    assert_eq!(Solution::triangle_number(vec![0, 1, 1, 1]), 1);
    assert_eq!(
        Solution::triangle_number(vec![
            84, 94, 72, 38, 69, 8, 53, 45, 88, 52, 58, 82, 25, 78, 17, 93, 67, 7, 61, 34, 10, 58, 61, 41, 58, 17, 15,
            64, 7, 96, 74, 43, 15, 95, 62, 76, 0, 3, 64, 54, 68, 61, 85, 38, 61, 14, 58, 63, 30, 4, 87, 2, 56, 10, 70,
            97, 11, 81, 53, 82, 39, 30, 0, 84, 85, 35, 17, 5, 0, 33, 66, 21, 26, 16, 94, 59, 81, 0, 1, 26, 14, 62, 16,
            50, 49, 54, 49, 5, 26, 84, 13, 73, 53, 48, 30, 34, 90, 11, 85, 57
        ]),
        69566
    );
}
