impl Solution {
    pub fn maximum_unique_subarray(a: Vec<i32>) -> i32 {
        let a: Vec<usize> = a.iter().map(|&item| item as usize).collect();
        let mut res = 0;
        let mut l = 0;
        let mut sum = 0;
        let mut exist = vec![false; 10001];
        for r in 0..a.len() {
            while exist[a[r]] {
                exist[a[l]] = false;
                sum -= a[l];
                l += 1;
            }

            exist[a[r]] = true;
            sum += a[r];
            res = res.max(sum);
        }
        res as _
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::maximum_unique_subarray(vec![4, 2, 4, 5, 6]), 17);
    assert_eq!(Solution::maximum_unique_subarray(vec![5, 2, 1, 2, 5, 2, 1, 2, 5]), 8);
}
