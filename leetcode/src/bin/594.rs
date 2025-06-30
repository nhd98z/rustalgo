use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        nums.iter().for_each(|&num| {
            map.insert(num, map.get(&num).unwrap_or(&0) + 1);
        });
        map.iter()
            .map(|(&num, &count)| {
                map.get(&(num + 1))
                    .map_or(0, |&next_count| count + next_count)
            })
            .max()
            .unwrap_or(0)
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
}
