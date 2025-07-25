#![allow(dead_code)]

use std::mem::swap;
use std::*;

struct SegmentTree {
    n: usize,
    arr: Vec<usize>,
    tree: Vec<usize>,
}

impl SegmentTree {
    fn new(arr: Vec<usize>) -> SegmentTree {
        let n = arr.len();
        let tree = vec![0; n * 4];
        let mut st = SegmentTree { n, arr, tree };
        st.build(0, 0, n - 1);
        st
    }

    fn build(&mut self, node: usize, l: usize, r: usize) {
        if l == r {
            self.tree[node] = self.arr[l];
            return;
        }
        let node_l = 2 * node + 1;
        let node_r = 2 * node + 2;
        let mid = l + (r - l) / 2;
        self.build(node_l, l, mid);
        self.build(node_r, mid + 1, r);
        self.tree[node] = cmp::max(self.tree[node_l], self.tree[node_r]);
    }

    fn find_most_left_greater(
        &self,
        node: usize,
        l: usize,
        r: usize,
        ql: usize,
        qr: usize,
        target: usize,
    ) -> Option<usize> {
        if qr < l || ql > r {
            return None;
        }
        if l == r {
            return if self.tree[node] > target { Some(l) } else { None };
        }
        if self.tree[node] <= target {
            return None;
        }
        let mid = l + (r - l) / 2;
        if let Some(result_l) = self.find_most_left_greater(2 * node + 1, l, mid, ql, qr, target) {
            return Some(result_l);
        }
        self.find_most_left_greater(2 * node + 2, mid + 1, r, ql, qr, target)
    }
}

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = heights.len();
        let st = SegmentTree::new(heights.iter().map(|e| *e as usize).collect());
        let mut ans = vec![];
        for q in queries {
            let mut left = q[0] as usize;
            let mut right = q[1] as usize;
            if left > right {
                swap(&mut left, &mut right);
            }
            if left == right {
                ans.push(left as i32);
                continue;
            }
            if heights[left] < heights[right] {
                ans.push(right as i32);
                continue;
            }
            let from_index = cmp::max(left, right);
            let from_height = cmp::max(heights[left], heights[right]);
            if let Some(result) = st.find_most_left_greater(0, 0, n - 1, from_index + 1, n - 1, from_height as _) {
                ans.push(result as _);
            } else {
                ans.push(-1);
            }
        }
        ans
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::leftmost_building_queries(
            vec![6, 4, 8, 5, 2, 7],
            vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]]
        ),
        vec![2, 5, -1, 5, 2]
    );
    assert_eq!(
        Solution::leftmost_building_queries(
            vec![5, 3, 8, 2, 6, 1, 4, 6],
            vec![vec![0, 7], vec![3, 5], vec![5, 2], vec![3, 0], vec![1, 6]]
        ),
        vec![7, 6, -1, 4, 6]
    );
    assert_eq!(
        Solution::leftmost_building_queries(vec![1, 2, 1, 2, 1, 2], vec![vec![0, 2]]),
        vec![3]
    );
}
