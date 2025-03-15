#![allow(dead_code)]

impl Solution {
    pub fn get_strongest(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut arr = arr;
        let n = arr.len();
        arr.sort_unstable();
        let m = arr[(n - 1) / 2];
        let mut res = vec![];
        let mut i = 0;
        let mut j = n - 1;
        while res.len() < k as usize {
            let si = (arr[i] - m).abs();
            let sj = (arr[j] - m).abs();
            if si > sj {
                res.push(arr[i]);
                i += 1;
            } else if si <= sj {
                res.push(arr[j]);
                j -= 1;
            }
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::get_strongest(vec![1, 2, 3, 4, 5], 2), vec![5, 1]);
}
