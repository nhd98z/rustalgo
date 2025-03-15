impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let mut count = 0;
            for c in &candidates {
                if c & (1 << i) != 0 {
                    count += 1;
                }
            }
            res = res.max(count);
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::largest_combination(vec![16, 17, 71, 62, 12, 24, 14]),
        4
    );
}
