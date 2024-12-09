use std::collections::BinaryHeap;

impl Solution {
    pub fn max_removal(nums: Vec<i32>, mut queries: Vec<Vec<i32>>) -> i32 {
        let (n, q) = (nums.len(), queries.len());
        queries.sort_unstable_by_key(|q| q[0]);
        let mut arr = vec![0; n + 1];
        let mut decr = 0;
        let mut idx = 0;
        let mut ans = q as i32;
        let mut heap = BinaryHeap::new();
        let queries: Vec<(usize, usize)> = queries
            .into_iter()
            .map(|v| (v[0] as usize, v[1] as usize))
            .collect();

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
                    if end + 1 < arr.len() {
                        arr[end + 1] += 1;
                    }
                    ans -= 1;
                    decr += 1;
                    need -= 1;
                } else {
                    return -1;
                }
            }
        }
        ans
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::max_removal(vec![2, 0, 2], vec![vec![0, 2], vec![0, 2], vec![1, 1]]),
        1
    );
}
