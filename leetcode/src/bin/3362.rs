use std::collections::BinaryHeap;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let q = queries.len();
        let mut ans = q;
        let mut arr = vec![0; n + 1];
        let mut decr = 0;
        let mut idx = 0;
        let mut queries: Vec<(usize, usize)> = queries.iter().map(|q| (q[0] as usize, q[1] as usize)).collect();
        let mut heap = BinaryHeap::new();
        queries.sort_unstable_by(|a, b| a.cmp(b));
        for i in 0..n {
            decr -= arr[i];
            let mut need = nums[i] - decr;
            while idx < q && queries[idx].0 == i {
                heap.push(queries[idx].1);
                idx += 1;
            }
            while need > 0 {
                if let Some(end) = heap.pop() {
                    if end < i {
                        continue;
                    }
                    arr[end + 1] += 1;
                    decr += 1;
                    need -= 1;
                    ans -= 1;
                } else {
                    return -1;
                }
            }
        }
        ans as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::max_removal(vec![2, 0, 2], vec![vec![0, 2], vec![0, 2], vec![1, 1]]),
        1
    );
}
