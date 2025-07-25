use std::collections::HashMap;

impl Solution {
    pub fn intersect(a1: Vec<i32>, a2: Vec<i32>) -> Vec<i32> {
        if a1.len() > a2.len() {
            return Self::intersect(a2, a1);
        }

        let mut map = HashMap::new();
        for num in a2 {
            *map.entry(num).or_insert(0) += 1;
        }

        let mut result = vec![];
        for num in a1 {
            if let Some(count) = map.get_mut(&num) {
                if *count > 0 {
                    result.push(num);
                    *count -= 1;
                }
            }
        }

        result
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2]);
}
