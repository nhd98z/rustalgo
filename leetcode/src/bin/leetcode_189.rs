#![allow(dead_code)]

impl Solution {
    pub fn rotate(a: &mut Vec<i32>, k: i32) {
        let n = a.len();
        a.rotate_right(k as usize % n);
    }
}

struct Solution;

fn main() {
    let mut a = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut a, 3);
    assert_eq!(a, vec![5, 6, 7, 1, 2, 3, 4]);
}
