use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        for &num in &arr {
            *count.entry(num).or_insert(0) += 1;
        }
        let mut lucky = -1;
        for (num, count) in count {
            if num == count {
                lucky = lucky.max(num);
            }
        }
        lucky
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_lucky(vec![2, 2, 3, 4]), 2);
    assert_eq!(Solution::find_lucky(vec![1, 2, 2, 3, 3, 3]), 3);
    assert_eq!(Solution::find_lucky(vec![2, 2, 2, 3, 3]), -1);
}
