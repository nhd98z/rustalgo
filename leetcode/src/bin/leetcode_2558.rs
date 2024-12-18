impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap = std::collections::BinaryHeap::new();
        for gift in gifts {
            heap.push(gift as i64);
        }
        for _ in 0..k {
            if let Some(max_gifts) = heap.pop() {
                heap.push((max_gifts as f64).sqrt().floor() as i64);
            }
        }
        heap.iter().sum()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::pick_gifts(vec![25, 64, 9, 4, 100], 4), 29);
}
