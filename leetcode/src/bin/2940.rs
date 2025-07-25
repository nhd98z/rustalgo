#![allow(dead_code)]

use std::mem::swap;

struct SegmentTree {
    n: usize,
    arr: Vec<i32>,
    tree: Vec<i32>,
}

impl SegmentTree {
    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let tree = vec![0; 4 * n];
        let mut st = SegmentTree { n, arr, tree };
        st.build(0, 0, n - 1);
        st
    }

    fn build(&mut self, node: usize, l: usize, r: usize) {
        if l == r {
            self.tree[node] = self.arr[l];
            return;
        }
        let node_left = 2 * node + 1;
        let node_right = 2 * node + 2;
        let mid = l + (r - l) / 2;
        self.build(node_left, l, mid);
        self.build(node_right, mid + 1, r);
        self.tree[node] = i32::max(self.tree[node_left], self.tree[node_right]);
    }

    fn find_first_greater(&self, node: usize, l: usize, r: usize, ql: usize, qr: usize, target: i32) -> i32 {
        if qr < l || ql > r {
            return -1;
        }
        if l == r {
            if self.tree[node] > target {
                return l as i32;
            }
            return -1;
        }
        if self.tree[node] <= target {
            return -1;
        }
        let mid = l + (r - l) / 2;
        let result_left = self.find_first_greater(2 * node + 1, l, mid, ql, qr, target);
        if result_left != -1 {
            return result_left;
        }
        self.find_first_greater(2 * node + 2, mid + 1, r, ql, qr, target)
    }
}

impl Solution {
    pub fn leftmost_building_queries_stupid(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];
        for (i, q) in queries.iter().enumerate() {
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
            let from_index = usize::max(left, right);
            let from_height = i32::max(heights[left], heights[right]);
            let heights_slice = heights[from_index + 1..].to_vec();
            for (idx, h) in heights_slice.iter().enumerate() {
                if h > &from_height {
                    ans.push((idx + from_index + 1) as i32);
                    break;
                }
            }
            if ans.len() == i {
                ans.push(-1);
            }
        }
        ans
    }

    pub fn leftmost_building_queries(heights: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        // println!("heights = {:?}", &heights);
        // println!("queries = {:?}", &queries);
        let n = heights.len();
        let st = SegmentTree::new(heights.clone());
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
            let from_index = usize::max(left, right);
            let from_height = i32::max(heights[left], heights[right]);
            let result = st.find_first_greater(0, 0, n - 1, from_index + 1, n - 1, from_height);
            ans.push(result);
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
