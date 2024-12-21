#![allow(dead_code)]

impl Solution {
    pub fn maximum_gap_stupid(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        if nums.len() < 2 {
            return 0;
        }
        let bucket_size = (nums.len() as f64).sqrt() as usize;
        let mut buckets = vec![vec![]; bucket_size];
        let max = nums.iter().max().unwrap();
        let min = nums.iter().min().unwrap();
        let range = max - min;
        for num in &nums {
            let index = ((num - min) as f64 / range as f64) as usize;
            buckets[usize::min(index, bucket_size - 1)].push(num);
        }
        let mut sorted_nums: Vec<usize> = vec![];
        for mut bucket in buckets {
            bucket.sort();
            sorted_nums.extend(bucket.iter().map(|i| **i as usize));
        }
        for i in 1..sorted_nums.len() {
            ans = usize::max(ans, sorted_nums[i] - sorted_nums[i - 1]);
        }
        ans as _
    }

    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let n = nums.len();
        if nums.len() < 2 {
            return 0;
        }
        let (min, max) = nums.iter().fold((i32::MAX, i32::MIN), |(min, max), &num| {
            (i32::min(min, num), i32::max(max, num))
        });
        if min == max {
            return 0;
        }
        let bucket_size = ((max - min) as f64 / (n - 1) as f64).ceil() as i32;
        let bucket_count = ((max - min) / bucket_size) as usize + 1;
        let mut bucket_min = vec![i32::MAX; bucket_count];
        let mut bucket_max = vec![i32::MIN; bucket_count];
        for num in nums {
            let index = ((num - min) / bucket_size) as usize;
            bucket_min[index] = i32::min(bucket_min[index], num);
            bucket_max[index] = i32::max(bucket_max[index], num);
        }
        let mut prev_max = bucket_min[0];
        for i in 0..bucket_count {
            if bucket_min[i] == i32::MAX {
                continue;
            }
            ans = i32::max(ans, bucket_min[i] - prev_max);
            prev_max = bucket_max[i];
        }
        ans
    }
}

struct Solution;

fn main() {
    // assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
    // assert_eq!(Solution::maximum_gap(vec![10]), 0);
    assert_eq!(Solution::maximum_gap(vec![100, 3, 2, 1]), 97);
}
