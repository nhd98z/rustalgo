#![allow(dead_code)]
fn main() {}

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
        let mid = l + (r - l) / 2;
        let left_child = 2 * node + 1;
        let right_child = 2 * node + 2;
        self.build(left_child, l, mid);
        self.build(right_child, mid + 1, r);
        self.tree[node] = self.tree[left_child] + self.tree[right_child];
    }
}
